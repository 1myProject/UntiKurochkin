use crate::app::App;
use crate::memory_viewer::Meme;

pub trait Move {
    const MORE: i32;
    const MORE_MORE: i32;
    const LESS: i32;
    const LESS_LESS: i32;
    const Y: i32;
    const F: fn(&Meme) -> f32;

    const MAX_VALUE: f32;
    const MIN_VALUE: f32;
    fn val(&self, app: &mut App) -> f32 {
        Self::F(&app.mem)
    }

    #[inline]
    fn more(&self, app: &mut App) {
        app.click(Self::MORE, Self::Y);
    }
    fn more_more(&self, app: &mut App) {
        if Self::F(&app.mem) >= Self::MAX_VALUE {
            return self.more(app);
        }
        app.click(Self::MORE_MORE, Self::Y);
    }
    #[inline]
    fn less(&self, app: &mut App) {
        app.click(Self::LESS, Self::Y);
    }
    fn less_less(&self, app: &mut App) {
        if Self::F(&app.mem) <= Self::MIN_VALUE {
            return self.less(app);
        }
        app.click(Self::LESS_LESS, Self::Y);
    }
}

pub struct LC;
impl Move for LC {
    const MORE: i32 = 195;
    const MORE_MORE: i32 = 186;
    const LESS: i32 = 112;
    const LESS_LESS: i32 = 125;
    const Y: i32 = 589;
    const F: fn(&Meme) -> f32 = Meme::lc;
    const MAX_VALUE: f32 = 0.00989;
    const MIN_VALUE: f32 = 0.00021;
}

pub struct CD;
impl Move for CD {
    const MORE: i32 = 195;
    const MORE_MORE: i32 = 186;
    const LESS: i32 = 112;
    const LESS_LESS: i32 = 125;
    const Y: i32 = 621;
    const F: fn(&Meme) -> f32 = Meme::cd;
    const MAX_VALUE: f32 = 0.000_000_000_190;
    const MIN_VALUE: f32 = 0.000_000_000_002;
}

pub struct LG;
impl Move for LG {
    const MORE: i32 = 694;
    const MORE_MORE: i32 = 684;
    const LESS: i32 = 614;
    const LESS_LESS: i32 = 622;
    const Y: i32 = 623;
    const F: fn(&Meme) -> f32 = Meme::lg;
    const MAX_VALUE: f32 = 0.000_490;
    const MIN_VALUE: f32 = 0.000_015;
}

pub struct CPOS;
impl Move for CPOS {
    const MORE: i32 = 694;
    const MORE_MORE: i32 = 684;
    const LESS: i32 = 614;
    const LESS_LESS: i32 = 622;
    const Y: i32 = 652;
    const F: fn(&Meme) -> f32 = Meme::cpos;
    const MAX_VALUE: f32 = 0.000_000_009_900;
    const MIN_VALUE: f32 = 0.000_000_000_250;
}

pub struct CPAR;

impl Move for CPAR {
    const MORE: i32 = 694;
    const MORE_MORE: i32 = 684;
    const LESS: i32 = 614;
    const LESS_LESS: i32 = 622;
    const Y: i32 = 685;
    const F: fn(&Meme) -> f32 = Meme::cpar;
    const MAX_VALUE: f32 = 0.000_000_000_290;
    const MIN_VALUE: f32 = 0.000_000_000_005;
}
