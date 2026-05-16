use rand::Rng;
use regex::Regex;
use serde_json::{json, Value};
use std::fs;
use std::fs::DirEntry;

fn parsing_curl(
    curl: String,
) -> Result<(String, String, String, String, String), Box<dyn std::error::Error>> {
    let re_sesskey = Regex::new(r"sesskey=([^&']+)")?;
    let sesskey = re_sesskey
        .captures(&curl)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .expect("sesskey not found");

    let re_referer = Regex::new(r"-H 'Referer: ([^']+)'")?;
    let referer = re_referer
        .captures(&curl)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .expect("Referer not found");

    // Firefox → -H 'Cookie: ...', Chromium → -b '...'
    let cookie = if let Some(caps) = Regex::new(r"-H 'Cookie: ([^']+)")?.captures(&curl) {
        caps.get(1).unwrap().as_str().to_string()
    } else {
        let re = Regex::new(r"-b '([^']+)'")?;
        re.captures(&curl)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .expect("Cookie not found")
    };

    // Парсим name и account name из JSON внутри curl
    let re_names = Regex::new(r#"\\{0,2}"name\\{0,2}":\\{0,2}"([^"]+)"#)?;
    let names: Vec<&str> = re_names
        .captures_iter(&curl)
        .filter_map(|c| c.get(1).map(|m| m.as_str()))
        .take(2)
        .collect();
    assert!(names.len() == 2, "Could not find actor name and account");

    let mut actor_name = names[0].to_string();
    let mut actor_acc = names[1].to_string();

    // Убираем trailing backslash (Firefox-вариант)
    for _ in 0..2 {
        if actor_acc.ends_with('\\') {
            actor_acc.pop();
            actor_name.pop();
        }
    }

    // Unicode-escape decode для Firefox
    let actor_name = decode_unicode_escape(&actor_name);

    Ok((sesskey, referer, cookie, actor_name, actor_acc))
}

/// Декодирует unicode escapes вида \uXXXX в строке.
fn decode_unicode_escape(s: &str) -> String {
    let re = Regex::new(r"\\u([0-9a-fA-F]{4})").unwrap();
    re.replace_all(s, |caps: &regex::Captures| {
        let code = u32::from_str_radix(&caps[1], 16).unwrap();
        char::from_u32(code)
            .map(|c| c.to_string())
            .unwrap_or_else(|| caps[0].to_string())
    })
    .to_string()
}

fn get_list_of_files() -> Result<Vec<DirEntry>, Box<dyn std::error::Error>> {
    let dir = "trenings/ч2/р1";
    let mut entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();

    let dir = "trenings/ч2/р2";
    entries.append(
        &mut fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .collect(),
    );

    entries.sort_by_key(|e| e.file_name());
    Ok(entries)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let curl = fs::read_to_string("curl.txt")?;

    // --- Парсинг curl ---

    let (sesskey, referer, cookie, actor_name, actor_acc) = parsing_curl(curl)?;

    println!("sesskey: {}", sesskey);
    println!("referer: {}", referer);
    println!("cookie: {}", cookie);
    println!("actor_name: {}", actor_name);
    println!("actor_acc: {}", actor_acc);

    // --- HTTP-клиент ---

    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://lms.bsuir.by/lib/ajax/service.php?sesskey={}&info=core_xapi_statement_post",
        sesskey
    );

    let base_headers = {
        use reqwest::header::{
            HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CONNECTION, CONTENT_TYPE, ORIGIN,
            REFERER,
        };
        let mut h = HeaderMap::new();
        h.insert(
            ACCEPT,
            HeaderValue::from_static("application/json, text/javascript, */*; q=0.01"),
        );
        h.insert(
            ACCEPT_LANGUAGE,
            HeaderValue::from_static("ru-RU,ru;q=0.9,en-US;q=0.8,en;q=0.7"),
        );
        h.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        h.insert(ORIGIN, HeaderValue::from_static("https://lms.bsuir.by"));
        h.insert(REFERER, HeaderValue::from_str(&referer)?);
        h.insert(
            "X-Requested-With",
            HeaderValue::from_static("XMLHttpRequest"),
        );
        h.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
        h.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
        h.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
        h.insert("User-Agent",       HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 OPR/129.0.0.0"));
        h.insert(
            "sec-ch-ua",
            HeaderValue::from_static(
                r#""Not:A-Brand";v="99", "Opera";v="129", "Chromium";v="145""#,
            ),
        );
        h.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
        h.insert("sec-ch-ua-platform", HeaderValue::from_static(r#""Linux""#));
        h.insert("Cookie", HeaderValue::from_str(&cookie)?);
        h
    };

    // --- Обход файлов ---

    println!("{}", "-".repeat(20));

    let entries = get_list_of_files()?;

    let mut rng = rand::thread_rng();

    for entry in entries {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_string_lossy();
        print!("{} | ", filename);

        let raw = fs::read_to_string(&path)?;
        let mut info: Vec<Value> = serde_json::from_str(&raw)?;

        // Подставляем actor
        info[0]["actor"]["name"] = json!(actor_name);
        info[0]["actor"]["account"]["name"] = json!(actor_acc);

        // Подставляем правильный ответ и максимальный балл
        let correct = info[0]["object"]["definition"]["correctResponsesPattern"][0]
            .as_str()
            .unwrap_or("")
            .to_string();
        let score_max = info[0]["result"]["score"]["max"].clone();

        info[0]["result"]["response"] = json!(correct);
        info[0]["result"]["score"]["raw"] = score_max;
        info[0]["result"]["score"]["scaled"] = json!(1.0);
        info[0]["result"]["success"] = json!(true);

        // Случайная длительность PT[20-30]M[0-58]S
        let minutes: u64 = rng.gen_range(20..=30);
        let seconds: u64 = rng.gen_range(0..59);
        let dur = if seconds == 0 {
            format!("PT{}M", minutes)
        } else {
            format!("PT{}M{}S", minutes, seconds)
        };
        info[0]["result"]["duration"] = json!(dur);

        let request_json = serde_json::to_string(&info)?;

        let body = json!([{
            "index": 0,
            "methodname": "core_xapi_statement_post",
            "args": {
                "component": "mod_h5pactivity",
                "requestjson": request_json,
            }
        }]);

        let resp = client
            .post(&url)
            .headers(base_headers.clone())
            .json(&body)
            .send()?;

        let status = resp.status().as_u16();
        let text = resp.text()?;

        if status == 200 && text == r#"[{"error":false,"data":[true]}]"# {
            println!("+");
        } else {
            println!("-  (status={}, body={})", status, text);
        }
    }

    println!("Можете обновлять страницу браузера.\nИ закинуть донатик в эту ёбаную кантору (шутка, не говорите Курочкину)");

    Ok(())
}

#[test]
fn test_parse_curl() -> Result<(), Box<dyn std::error::Error>> {
    let curl_chrome = r#"curl 'https://lms.bsuir.by/lib/ajax/service.php?sesskey=xa59UOHCQf&info=core_xapi_statement_post' \
  -H 'Accept: application/json, text/javascript, */*; q=0.01' \
  -H 'Accept-Language: ru-RU,ru;q=0.9,en-US;q=0.8,en;q=0.7' \
  -H 'Connection: keep-alive' \
  -H 'Content-Type: application/json' \
  -b '_ym_uid=177540456955498584; _ym_d=1775404569; _ym_isad=2; MoodleSession=jsqkgph7bnpikct1pnkol0c9hj; MOODLEID1_=sodium%3AsHLZe2Gkle2JkJ31tpo6svSELBMmUZ8ISu9pbqXxJHiCtwfJ5av4CNxkzs%2BiBd5B' \
  -H 'Origin: https://lms.bsuir.by' \
  -H 'Referer: https://lms.bsuir.by/h5p/embed.php?url=https%3A%2F%2Flms.bsuir.by%2Fpluginfile.php%2F413976%2Fmod_h5pactivity%2Fpackage%2F0%2F%25D0%25A8%25D0%2590%25D0%259C%25D0%2590_PNP_%25D0%25BF%25D0%25B0%25D1%2580_%25D0%25BF%25D0%25BE%25D1%2581%25D0%25BB_%25D0%25BA%25D0%25BE%25D0%25BD%25D1%2582%25D1%2583%25D1%2580.h5p&preventredirect=1&component=mod_h5pactivity' \
  -H 'Sec-Fetch-Dest: empty' \
  -H 'Sec-Fetch-Mode: cors' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 OPR/129.0.0.0' \
  -H 'X-Requested-With: XMLHttpRequest' \
  -H 'sec-ch-ua: "Not:A-Brand";v="99", "Opera";v="129", "Chromium";v="145"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "Linux"' \
  --data-raw '[{"index":0,"methodname":"core_xapi_statement_post","args":{"component":"mod_h5pactivity","requestjson":"[{\"actor\":{\"name\":\"Вася Пупкин Аброхимович\",\"objectType\":\"Agent\",\"account\":{\"name\":\"29222\",\"homePage\":\"https://lms.bsuir.by\"}},\"verb\":{\"id\":\"http://adlnet.gov/expapi/verbs/answered\",\"display\":{\"en-US\":\"answered\"}},\"object\":{\"id\":\"https://lms.bsuir.by/xapi/activity/413976\",\"objectType\":\"Activity\",\"definition\":{\"extensions\":{\"http://h5p.org/x-api/h5p-local-content-id\":780},\"name\":{\"en-US\":\"Активная магнитная антенна 8\"},\"description\":{\"en-US\":\"Активная магнитная антенна 8\"},\"type\":\"http://adlnet.gov/expapi/activities/cmi.interaction\",\"interactionType\":\"matching\",\"source\":[{\"id\":\"0\",\"description\":{\"en-US\":\"L1\"}},{\"id\":\"1\",\"description\":{\"en-US\":\"L2\"}},{\"id\":\"2\",\"description\":{\"en-US\":\"L3\"}},{\"id\":\"3\",\"description\":{\"en-US\":\"L4\"}},{\"id\":\"4\",\"description\":{\"en-US\":\"L5\"}},{\"id\":\"5\",\"description\":{\"en-US\":\"L6\"}},{\"id\":\"6\",\"description\":{\"en-US\":\"L7\"}},{\"id\":\"7\",\"description\":{\"en-US\":\"L8\"}},{\"id\":\"8\",\"description\":{\"en-US\":\"L9\"}},{\"id\":\"9\",\"description\":{\"en-US\":\"L10\"}},{\"id\":\"10\",\"description\":{\"en-US\":\"L11\"}},{\"id\":\"11\",\"description\":{\"en-US\":\"L12\"}},{\"id\":\"12\",\"description\":{\"en-US\":\"L15\"}},{\"id\":\"13\",\"description\":{\"en-US\":\"Z\"}},{\"id\":\"14\",\"description\":{\"en-US\":\"Uc\"}},{\"id\":\"15\",\"description\":{\"en-US\":\"Ground\"}},{\"id\":\"16\",\"description\":{\"en-US\":\"Cvar-V\"}},{\"id\":\"17\",\"description\":{\"en-US\":\"R-V\"}},{\"id\":\"18\",\"description\":{\"en-US\":\"R-H\"}},{\"id\":\"19\",\"description\":{\"en-US\":\"C-H\"}},{\"id\":\"20\",\"description\":{\"en-US\":\"C-V\"}},{\"id\":\"21\",\"description\":{\"en-US\":\"Tb\"}},{\"id\":\"22\",\"description\":{\"en-US\":\"Ta_auto2\"}},{\"id\":\"23\",\"description\":{\"en-US\":\"VT-NPN-H\"}},{\"id\":\"24\",\"description\":{\"en-US\":\"B\"}},{\"id\":\"25\",\"description\":{\"en-US\":\"B-I\"}},{\"id\":\"26\",\"description\":{\"en-US\":\"VT-PNP-H\"}}],\"correctResponsesPattern\":[\"0[.]5[,]1[.]10[,]2[.]26[,]3[.]1[,]4[.]10[,]5[.]6[,]6[.]13[,]7[.]13[,]8[.]13[,]9[.]21[,]10[.]20[,]11[.]13[,]12[.]16[,]13[.]22[,]14[.]19[,]15[.]12[,]16[.]14[,]17[.]15[,]18[.]15[,]19[.]11[,]20[.]18[,]21[.]8[,]22[.]2[,]23[.]18[,]24[.]6[,]25[.]13[,]26[.]13[,]27[.]13[,]28[.]20[,]29[.]13[,]30[.]13[,]31[.]20[,]32[.]13[,]33[.]25[,]34[.]13[,]35[.]13[,]36[.]13[,]37[.]15[,]38[.]13[,]39[.]13[,]40[.]15[,]41[.]13[,]42[.]15[,]43[.]13\"],\"target\":[{\"id\":\"0\",\"description\":{\"en-US\":\"1\\n\"}},{\"id\":\"1\",\"description\":{\"en-US\":\"2\\n\"}},{\"id\":\"2\",\"description\":{\"en-US\":\"3\\n\"}},{\"id\":\"3\",\"description\":{\"en-US\":\"4\\n\"}},{\"id\":\"4\",\"description\":{\"en-US\":\"5\\n\"}},{\"id\":\"5\",\"description\":{\"en-US\":\"6\\n\"}},{\"id\":\"6\",\"description\":{\"en-US\":\"7\\n\"}},{\"id\":\"7\",\"description\":{\"en-US\":\"8\\n\"}},{\"id\":\"8\",\"description\":{\"en-US\":\"9\\n\"}},{\"id\":\"9\",\"description\":{\"en-US\":\"10\\n\"}},{\"id\":\"10\",\"description\":{\"en-US\":\"11\\n\"}},{\"id\":\"11\",\"description\":{\"en-US\":\"13\\n\"}},{\"id\":\"12\",\"description\":{\"en-US\":\"14\\n\"}},{\"id\":\"13\",\"description\":{\"en-US\":\"15\\n\"}},{\"id\":\"14\",\"description\":{\"en-US\":\"16\\n\"}},{\"id\":\"15\",\"description\":{\"en-US\":\"17\\n\"}},{\"id\":\"16\",\"description\":{\"en-US\":\"18\\n\"}},{\"id\":\"17\",\"description\":{\"en-US\":\"19\\n\"}},{\"id\":\"18\",\"description\":{\"en-US\":\"20\\n\"}},{\"id\":\"19\",\"description\":{\"en-US\":\"21\\n\"}},{\"id\":\"20\",\"description\":{\"en-US\":\"22\\n\"}},{\"id\":\"21\",\"description\":{\"en-US\":\"23\\n\"}},{\"id\":\"22\",\"description\":{\"en-US\":\"24\\n\"}},{\"id\":\"23\",\"description\":{\"en-US\":\"25\\n\"}},{\"id\":\"24\",\"description\":{\"en-US\":\"26\\n\"}},{\"id\":\"25\",\"description\":{\"en-US\":\"27\\n\"}},{\"id\":\"26\",\"description\":{\"en-US\":\"28\\n\"}},{\"id\":\"27\",\"description\":{\"en-US\":\"29\\n\"}},{\"id\":\"28\",\"description\":{\"en-US\":\"30\\n\"}},{\"id\":\"29\",\"description\":{\"en-US\":\"31\\n\"}},{\"id\":\"30\",\"description\":{\"en-US\":\"32\\n\"}},{\"id\":\"31\",\"description\":{\"en-US\":\"33\\n\"}},{\"id\":\"32\",\"description\":{\"en-US\":\"34\\n\"}},{\"id\":\"33\",\"description\":{\"en-US\":\"35\\n\"}},{\"id\":\"34\",\"description\":{\"en-US\":\"36\\n\"}},{\"id\":\"35\",\"description\":{\"en-US\":\"37\\n\"}},{\"id\":\"36\",\"description\":{\"en-US\":\"38\\n\"}},{\"id\":\"37\",\"description\":{\"en-US\":\"39\\n\"}},{\"id\":\"38\",\"description\":{\"en-US\":\"40\\n\"}},{\"id\":\"39\",\"description\":{\"en-US\":\"41\\n\"}},{\"id\":\"40\",\"description\":{\"en-US\":\"42\\n\"}},{\"id\":\"41\",\"description\":{\"en-US\":\"43\\n\"}},{\"id\":\"42\",\"description\":{\"en-US\":\"44\\n\"}},{\"id\":\"43\",\"description\":{\"en-US\":\"45\\n\"}}]}},\"context\":{\"contextActivities\":{\"category\":[{\"id\":\"http://h5p.org/libraries/H5P.DragQuestion-1.13\",\"objectType\":\"Activity\"}]}},\"result\":{\"score\":{\"min\":0,\"max\":44,\"raw\":0,\"scaled\":0},\"completion\":true,\"success\":false,\"duration\":\"PT2874.01S\",\"response\":\"\"}}]"}}]'
"#.to_string();
    let (sesskey, referer, cookie, actor_name, actor_acc) = parsing_curl(curl_chrome)?;
    assert_eq!(sesskey, "xa59UOHCQf");
    assert_eq!(
        referer,
        "https://lms.bsuir.by/h5p/embed.php?url=https%3A%2F%2Flms.bsuir.by%2Fpluginfile.php%2F413976%2Fmod_h5pactivity%2Fpackage%2F0%2F%25D0%25A8%25D0%2590%25D0%259C%25D0%2590_PNP_%25D0%25BF%25D0%25B0%25D1%2580_%25D0%25BF%25D0%25BE%25D1%2581%25D0%25BB_%25D0%25BA%25D0%25BE%25D0%25BD%25D1%2582%25D1%2583%25D1%2580.h5p&preventredirect=1&component=mod_h5pactivity"
    );
    assert_eq!(
        cookie,
        "_ym_uid=177540456955498584; _ym_d=1775404569; _ym_isad=2; MoodleSession=jsqkgph7bnpikct1pnkol0c9hj; MOODLEID1_=sodium%3AsHLZe2Gkle2JkJ31tpo6svSELBMmUZ8ISu9pbqXxJHiCtwfJ5av4CNxkzs%2BiBd5B"
    );
    assert_eq!(actor_name, "Вася Пупкин Аброхимович");
    assert_eq!(actor_acc, "29222");

    let curl_firefox = r#"curl 'https://lms.bsuir.by/lib/ajax/service.php?sesskey=3oYPi2T0EN&info=core_xapi_statement_post' \
  -X POST \
  -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:150.0) Gecko/20100101 Firefox/150.0' \
  -H 'Accept: application/json, text/javascript, */*; q=0.01' \
  -H 'Accept-Language: ru-RU,ru;q=0.9,en-US;q=0.8,en;q=0.7' \
  -H 'Accept-Encoding: gzip, deflate, br, zstd' \
  -H 'Content-Type: application/json' \
  -H 'X-Requested-With: XMLHttpRequest' \
  -H 'Origin: https://lms.bsuir.by' \
  -H 'Connection: keep-alive' \
  -H 'Referer: https://lms.bsuir.by/h5p/embed.php?url=https%3A%2F%2Flms.bsuir.by%2Fpluginfile.php%2F413950%2Fmod_h5pactivity%2Fpackage%2F0%2F%25D0%2592%25D0%25B0%25D1%2580%25D0%25B8%25D0%25BA%25D0%25B0%25D0%25BF%25D1%258B_%25D0%25B2%25D1%2581%25D1%2582%25D1%2580%25D0%25B5%25D1%2587%25D0%25BD%25D0%25BE_%25D0%25BF%25D0%25BE%25D1%2581%25D0%25BB_%25D0%25B2%25D0%25BA%25D0%25BB.h5p&preventredirect=1&component=mod_h5pactivity' \
  -H 'Cookie: MoodleSession=o21ie4mh9hvcu5kppejquflbof; MOODLEID1_=sodium%3AQWrsdn%2BCK1O1VAfKD6dHEk8ove2JozgTNO7ay4BBWjJ6CvkTXUFHGve7FBxxDWvQ' \
  -H 'Sec-Fetch-Dest: empty' \
  -H 'Sec-Fetch-Mode: cors' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'DNT: 1' \
  -H 'Sec-GPC: 1' \
  -H 'Priority: u=0' \
  --data-raw $'[{"index":0,"methodname":"core_xapi_statement_post","args":{"component":"mod_h5pactivity","requestjson":"[{\\"actor\\":{\\"name\\":\\"\u0410\u0411 \u0412\\",\\"objectType\\":\\"Agent\\",\\"account\\":{\\"name\\":\\"299\\",\\"homePage\\":\\"https://lms.bsuir.by\\"}},\\"verb\\":{\\"id\\":\\"http://adlnet.gov/expapi/verbs/answered\\",\\"display\\":{\\"en-US\\":\\"answered\\"}},\\"object\\":{\\"id\\":\\"https://lms.bsuir.by/xapi/activity/413950\\",\\"objectType\\":\\"Activity\\",\\"definition\\":{\\"extensions\\":{\\"http://h5p.org/x-api/h5p-local-content-id\\":799},\\"name\\":{\\"en-US\\":\\"\u0417\u0430\u0433\u043e\u0442\u043e\u0432\u043a\u0430 \u0412\u0430\u0440\u0438\u043a\u0430\u043f\u044b \u0441 \u0432\u0441\u0442\u0440\u0435\u0447\u043d\u043e-\u043f\u043e\u0441\u043b\u0435\u0434\u043e\u0432\u0430\u0442\u0435\u043b\u044c\u043d\u044b\u043c \u0432\u043a\u043b\u044e\u0447\u0435\u043d\u0438\u0435\u043c\\"},\\"description\\":{\\"en-US\\":\\"\u0417\u0430\u0433\u043e\u0442\u043e\u0432\u043a\u0430 \u0412\u0430\u0440\u0438\u043a\u0430\u043f\u044b \u0441 \u0432\u0441\u0442\u0440\u0435\u0447\u043d\u043e-\u043f\u043e\u0441\u043b\u0435\u0434\u043e\u0432\u0430\u0442\u0435\u043b\u044c\u043d\u044b\u043c \u0432\u043a\u043b\u044e\u0447\u0435\u043d\u0438\u0435\u043c\\"},\\"type\\":\\"http://adlnet.gov/expapi/activities/cmi.interaction\\",\\"interactionType\\":\\"matching\\",\\"source\\":[{\\"id\\":\\"0\\",\\"description\\":{\\"en-US\\":\\"L1\\"}},{\\"id\\":\\"1\\",\\"description\\":{\\"en-US\\":\\"L2\\"}},{\\"id\\":\\"2\\",\\"description\\":{\\"en-US\\":\\"L3\\"}},{\\"id\\":\\"3\\",\\"description\\":{\\"en-US\\":\\"L4\\"}},{\\"id\\":\\"4\\",\\"description\\":{\\"en-US\\":\\"L5\\"}},{\\"id\\":\\"5\\",\\"description\\":{\\"en-US\\":\\"L6\\"}},{\\"id\\":\\"6\\",\\"description\\":{\\"en-US\\":\\"L7\\"}},{\\"id\\":\\"7\\",\\"description\\":{\\"en-US\\":\\"L8\\"}},{\\"id\\":\\"8\\",\\"description\\":{\\"en-US\\":\\"L9\\"}},{\\"id\\":\\"9\\",\\"description\\":{\\"en-US\\":\\"L10\\"}},{\\"id\\":\\"10\\",\\"description\\":{\\"en-US\\":\\"L11\\"}},{\\"id\\":\\"11\\",\\"description\\":{\\"en-US\\":\\"L12\\"}},{\\"id\\":\\"12\\",\\"description\\":{\\"en-US\\":\\"L13\\"}},{\\"id\\":\\"13\\",\\"description\\":{\\"en-US\\":\\"L14\\"}},{\\"id\\":\\"14\\",\\"description\\":{\\"en-US\\":\\"L15\\"}},{\\"id\\":\\"15\\",\\"description\\":{\\"en-US\\":\\"L16\\"}},{\\"id\\":\\"16\\",\\"description\\":{\\"en-US\\":\\"R-H\\"}},{\\"id\\":\\"17\\",\\"description\\":{\\"en-US\\":\\"R-V\\"}},{\\"id\\":\\"18\\",\\"description\\":{\\"en-US\\":\\"C-H\\"}},{\\"id\\":\\"19\\",\\"description\\":{\\"en-US\\":\\"C-V\\"}},{\\"id\\":\\"20\\",\\"description\\":{\\"en-US\\":\\"Rvar1\\"}},{\\"id\\":\\"21\\",\\"description\\":{\\"en-US\\":\\"Rvar2\\"}},{\\"id\\":\\"22\\",\\"description\\":{\\"en-US\\":\\"Ta\\"}},{\\"id\\":\\"23\\",\\"description\\":{\\"en-US\\":\\"Tb\\"}},{\\"id\\":\\"24\\",\\"description\\":{\\"en-US\\":\\"Ta-auto1\\"}},{\\"id\\":\\"25\\",\\"description\\":{\\"en-US\\":\\"Tb-auto1\\"}},{\\"id\\":\\"26\\",\\"description\\":{\\"en-US\\":\\"VD1\\"}},{\\"id\\":\\"27\\",\\"description\\":{\\"en-US\\":\\"Cvar1\\"}},{\\"id\\":\\"28\\",\\"description\\":{\\"en-US\\":\\"VD2\\"}},{\\"id\\":\\"29\\",\\"description\\":{\\"en-US\\":\\"VD3\\"}},{\\"id\\":\\"30\\",\\"description\\":{\\"en-US\\":\\"VD4\\"}},{\\"id\\":\\"31\\",\\"description\\":{\\"en-US\\":\\"Cvar2\\"}},{\\"id\\":\\"32\\",\\"description\\":{\\"en-US\\":\\"Cvar3\\"}},{\\"id\\":\\"33\\",\\"description\\":{\\"en-US\\":\\"Cvar4\\"}},{\\"id\\":\\"34\\",\\"description\\":{\\"en-US\\":\\"Creg-H\\"}},{\\"id\\":\\"35\\",\\"description\\":{\\"en-US\\":\\"Creg-V\\"}},{\\"id\\":\\"36\\",\\"description\\":{\\"en-US\\":\\"Z\\"}},{\\"id\\":\\"37\\",\\"description\\":{\\"en-US\\":\\"Ground\\"}}],\\"correctResponsesPattern\\":[\\"0[.]36[,]1[.]36[,]2[.]36[,]3[.]36[,]4[.]5[,]5[.]1[,]6[.]1[,]7[.]1[,]8[.]1[,]9[.]1[,]10[.]18[,]11[.]6[,]12[.]5[,]13[.]18[,]14[.]6[,]15[.]5[,]16[.]3[,]17[.]10[,]18[.]10[,]19[.]6[,]20[.]36[,]21[.]36[,]22[.]36[,]23[.]17[,]24[.]17[,]25[.]36[,]26[.]22[,]27[.]25[,]28[.]7[,]29[.]35[,]30[.]19[,]31[.]31[,]32[.]36[,]33[.]36[,]34[.]15[,]35[.]37[,]36[.]37[,]37[.]36[,]38[.]37[,]39[.]37[,]40[.]36[,]41[.]37[,]42[.]37[,]43[.]11[,]44[.]16[,]45[.]10[,]46[.]20[,]47[.]36[,]48[.]36[,]49[.]36[,]50[.]36[,]51[.]36[,]52[.]36[,]53[.]36[,]54[.]36[,]55[.]33[,]56[.]36[,]57[.]19[,]58[.]0[,]59[.]36[,]60[.]36[,]61[.]36[,]62[.]36[,]63[.]36[,]64[.]36[,]65[.]36[,]66[.]36[,]67[.]37[,]68[.]36[,]69[.]37[,]70[.]37[,]71[.]36\\"],\\"target\\":[{\\"id\\":\\"0\\",\\"description\\":{\\"en-US\\":\\"1\\\\n\\"}},{\\"id\\":\\"1\\",\\"description\\":{\\"en-US\\":\\"2\\\\n\\"}},{\\"id\\":\\"2\\",\\"description\\":{\\"en-US\\":\\"3\\\\n\\"}},{\\"id\\":\\"3\\",\\"description\\":{\\"en-US\\":\\"4\\\\n\\"}},{\\"id\\":\\"4\\",\\"description\\":{\\"en-US\\":\\"5\\\\n\\"}},{\\"id\\":\\"5\\",\\"description\\":{\\"en-US\\":\\"6\\\\n\\"}},{\\"id\\":\\"6\\",\\"description\\":{\\"en-US\\":\\"7\\\\n\\"}},{\\"id\\":\\"7\\",\\"description\\":{\\"en-US\\":\\"8\\\\n\\"}},{\\"id\\":\\"8\\",\\"description\\":{\\"en-US\\":\\"9\\\\n\\"}},{\\"id\\":\\"9\\",\\"description\\":{\\"en-US\\":\\"10\\\\n\\"}},{\\"id\\":\\"10\\",\\"description\\":{\\"en-US\\":\\"11\\\\n\\"}},{\\"id\\":\\"11\\",\\"description\\":{\\"en-US\\":\\"12\\\\n\\"}},{\\"id\\":\\"12\\",\\"description\\":{\\"en-US\\":\\"13\\\\n\\"}},{\\"id\\":\\"13\\",\\"description\\":{\\"en-US\\":\\"14\\\\n\\"}},{\\"id\\":\\"14\\",\\"description\\":{\\"en-US\\":\\"15\\\\n\\"}},{\\"id\\":\\"15\\",\\"description\\":{\\"en-US\\":\\"16\\\\n\\"}},{\\"id\\":\\"16\\",\\"description\\":{\\"en-US\\":\\"17\\\\n\\"}},{\\"id\\":\\"17\\",\\"description\\":{\\"en-US\\":\\"18\\\\n\\"}},{\\"id\\":\\"18\\",\\"description\\":{\\"en-US\\":\\"19\\\\n\\"}},{\\"id\\":\\"19\\",\\"description\\":{\\"en-US\\":\\"20\\\\n\\"}},{\\"id\\":\\"20\\",\\"description\\":{\\"en-US\\":\\"21\\\\n\\"}},{\\"id\\":\\"21\\",\\"description\\":{\\"en-US\\":\\"22\\\\n\\"}},{\\"id\\":\\"22\\",\\"description\\":{\\"en-US\\":\\"23\\\\n\\"}},{\\"id\\":\\"23\\",\\"description\\":{\\"en-US\\":\\"24\\\\n\\"}},{\\"id\\":\\"24\\",\\"description\\":{\\"en-US\\":\\"25\\\\n\\"}},{\\"id\\":\\"25\\",\\"description\\":{\\"en-US\\":\\"26\\\\n\\"}},{\\"id\\":\\"26\\",\\"description\\":{\\"en-US\\":\\"27\\\\n\\"}},{\\"id\\":\\"27\\",\\"description\\":{\\"en-US\\":\\"28\\\\n\\"}},{\\"id\\":\\"28\\",\\"description\\":{\\"en-US\\":\\"29\\\\n\\"}},{\\"id\\":\\"29\\",\\"description\\":{\\"en-US\\":\\"30\\\\n\\"}},{\\"id\\":\\"30\\",\\"description\\":{\\"en-US\\":\\"31\\\\n\\"}},{\\"id\\":\\"31\\",\\"description\\":{\\"en-US\\":\\"32\\\\n\\"}},{\\"id\\":\\"32\\",\\"description\\":{\\"en-US\\":\\"33\\\\n\\"}},{\\"id\\":\\"33\\",\\"description\\":{\\"en-US\\":\\"34\\\\n\\"}},{\\"id\\":\\"34\\",\\"description\\":{\\"en-US\\":\\"35\\\\n\\"}},{\\"id\\":\\"35\\",\\"description\\":{\\"en-US\\":\\"36\\\\n\\"}},{\\"id\\":\\"36\\",\\"description\\":{\\"en-US\\":\\"37\\\\n\\"}},{\\"id\\":\\"37\\",\\"description\\":{\\"en-US\\":\\"38\\\\n\\"}},{\\"id\\":\\"38\\",\\"description\\":{\\"en-US\\":\\"39\\\\n\\"}},{\\"id\\":\\"39\\",\\"description\\":{\\"en-US\\":\\"40\\\\n\\"}},{\\"id\\":\\"40\\",\\"description\\":{\\"en-US\\":\\"41\\\\n\\"}},{\\"id\\":\\"41\\",\\"description\\":{\\"en-US\\":\\"42\\\\n\\"}},{\\"id\\":\\"42\\",\\"description\\":{\\"en-US\\":\\"43\\\\n\\"}},{\\"id\\":\\"43\\",\\"description\\":{\\"en-US\\":\\"44\\\\n\\"}},{\\"id\\":\\"44\\",\\"description\\":{\\"en-US\\":\\"45\\\\n\\"}},{\\"id\\":\\"45\\",\\"description\\":{\\"en-US\\":\\"46\\\\n\\"}},{\\"id\\":\\"46\\",\\"description\\":{\\"en-US\\":\\"47\\\\n\\"}},{\\"id\\":\\"47\\",\\"description\\":{\\"en-US\\":\\"48\\\\n\\"}},{\\"id\\":\\"48\\",\\"description\\":{\\"en-US\\":\\"49\\\\n\\"}},{\\"id\\":\\"49\\",\\"description\\":{\\"en-US\\":\\"50\\\\n\\"}},{\\"id\\":\\"50\\",\\"description\\":{\\"en-US\\":\\"51\\\\n\\"}},{\\"id\\":\\"51\\",\\"description\\":{\\"en-US\\":\\"52\\\\n\\"}},{\\"id\\":\\"52\\",\\"description\\":{\\"en-US\\":\\"53\\\\n\\"}},{\\"id\\":\\"53\\",\\"description\\":{\\"en-US\\":\\"54\\\\n\\"}},{\\"id\\":\\"54\\",\\"description\\":{\\"en-US\\":\\"55\\\\n\\"}},{\\"id\\":\\"55\\",\\"description\\":{\\"en-US\\":\\"56\\\\n\\"}},{\\"id\\":\\"56\\",\\"description\\":{\\"en-US\\":\\"57\\\\n\\"}},{\\"id\\":\\"57\\",\\"description\\":{\\"en-US\\":\\"58\\\\n\\"}},{\\"id\\":\\"58\\",\\"description\\":{\\"en-US\\":\\"59\\\\n\\"}},{\\"id\\":\\"59\\",\\"description\\":{\\"en-US\\":\\"60\\\\n\\"}},{\\"id\\":\\"60\\",\\"description\\":{\\"en-US\\":\\"61\\\\n\\"}},{\\"id\\":\\"61\\",\\"description\\":{\\"en-US\\":\\"62\\\\n\\"}},{\\"id\\":\\"62\\",\\"description\\":{\\"en-US\\":\\"63\\\\n\\"}},{\\"id\\":\\"63\\",\\"description\\":{\\"en-US\\":\\"64\\\\n\\"}},{\\"id\\":\\"64\\",\\"description\\":{\\"en-US\\":\\"65\\\\n\\"}},{\\"id\\":\\"65\\",\\"description\\":{\\"en-US\\":\\"66\\\\n\\"}},{\\"id\\":\\"66\\",\\"description\\":{\\"en-US\\":\\"67\\\\n\\"}},{\\"id\\":\\"67\\",\\"description\\":{\\"en-US\\":\\"68\\\\n\\"}},{\\"id\\":\\"68\\",\\"description\\":{\\"en-US\\":\\"69\\\\n\\"}},{\\"id\\":\\"69\\",\\"description\\":{\\"en-US\\":\\"70\\\\n\\"}},{\\"id\\":\\"70\\",\\"description\\":{\\"en-US\\":\\"71\\\\n\\"}},{\\"id\\":\\"71\\",\\"description\\":{\\"en-US\\":\\"72\\\\n\\"}}]}},\\"context\\":{\\"contextActivities\\":{\\"category\\":[{\\"id\\":\\"http://h5p.org/libraries/H5P.DragQuestion-1.13\\",\\"objectType\\":\\"Activity\\"}]}},\\"result\\":{\\"score\\":{\\"min\\":0,\\"max\\":72,\\"raw\\":0,\\"scaled\\":0},\\"completion\\":true,\\"success\\":false,\\"duration\\":\\"PT18.14S\\",\\"response\\":\\"\\"}}]"}}]'
  "#.to_string();
    let (sesskey, referer, cookie, actor_name, actor_acc) = parsing_curl(curl_firefox)?;
    assert_eq!(sesskey, "3oYPi2T0EN");
    assert_eq!(
        referer,
        "https://lms.bsuir.by/h5p/embed.php?url=https%3A%2F%2Flms.bsuir.by%2Fpluginfile.php%2F413950%2Fmod_h5pactivity%2Fpackage%2F0%2F%25D0%2592%25D0%25B0%25D1%2580%25D0%25B8%25D0%25BA%25D0%25B0%25D0%25BF%25D1%258B_%25D0%25B2%25D1%2581%25D1%2582%25D1%2580%25D0%25B5%25D1%2587%25D0%25BD%25D0%25BE_%25D0%25BF%25D0%25BE%25D1%2581%25D0%25BB_%25D0%25B2%25D0%25BA%25D0%25BB.h5p&preventredirect=1&component=mod_h5pactivity"
    );
    assert_eq!(
        cookie,
        "MoodleSession=o21ie4mh9hvcu5kppejquflbof; MOODLEID1_=sodium%3AQWrsdn%2BCK1O1VAfKD6dHEk8ove2JozgTNO7ay4BBWjJ6CvkTXUFHGve7FBxxDWvQ"
    );
    assert_eq!(actor_name, "АБ В");
    assert_eq!(actor_acc, "299");

    Ok(())
}
