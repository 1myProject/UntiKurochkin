use crc64fast_nvme::Digest;
use sysinfo::{Disks, System};

const LICENSE: u64 = 0xECB61D7C3B83DF98;
fn sys_face() -> String {
    let mut sys = System::new_all();

    sys.refresh_all();

    let mut os = format!(
        "{:?}\n{:?}\n{:?}",
        System::kernel_version(),
        System::os_version(),
        System::host_name()
    );
    // println!("{os}");

    for cpu in sys.cpus() {
        let a = format!(
            "{} {} {} {}",
            cpu.name(),
            cpu.frequency(),
            cpu.vendor_id(),
            cpu.brand()
        );
        // println!("{}", a);
        os.push_str(a.as_str());
    }

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        if disk.is_removable() {
            continue;
        }
        let a = format!(
            "{} {:?} {} {}",
            disk.name().display(),
            disk.kind(),
            disk.file_system().display(),
            disk.mount_point().display(),
        );
        // println!("{a}");
        os.push_str(a.as_str());
    }
    os
}

fn to_hash(txt: String) -> u64 {
    let mut c = Digest::new();
    c.write(txt.as_bytes());
    c.sum64()
}

#[derive(serde::Deserialize)]
struct Data {
    timestamp: u64,
}

pub fn license() -> Result<bool, Box<dyn std::error::Error>> {
    let txt = sys_face();
    if to_hash(txt) == LICENSE {
        return Ok(true);
    }

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:145.0) Gecko/20100101 Firefox/145.0".parse().unwrap());
    // headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".parse().unwrap());
    // headers.insert("Accept-Language", "ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3".parse().unwrap());
    // headers.insert("Accept-Encoding", "gzip, deflate, br, zstd".parse().unwrap());
    // headers.insert("Connection", "keep-alive".parse().unwrap());
    // headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
    // headers.insert("Sec-Fetch-Dest", "document".parse().unwrap());
    // headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
    // headers.insert("Sec-Fetch-Site", "none".parse().unwrap());
    // headers.insert("Sec-Fetch-User", "?1".parse().unwrap());
    // headers.insert("DNT", "1".parse().unwrap());
    // headers.insert("Sec-GPC", "1".parse().unwrap());
    // headers.insert("Priority", "u=0, i".parse().unwrap());
    // headers.insert("TE", "trailers".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.get("https://aisenseapi.com/services/v1/timestamp")
        .headers(headers)
        .send()?
        .text()?;
    let data: Data = serde_json::from_str::<Data>(&res).unwrap();
    // let res = res.replace("{\"timestamp\":", "").replace("}", "");

    // let time: u64 = res.parse()?;

    Ok(data.timestamp<=1767225600)
}
