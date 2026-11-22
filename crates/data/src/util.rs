use palette::color_difference::{EuclideanDistance, Wcag21RelativeContrast};
use palette::{IntoColor, Srgb};

pub struct FgBg<T>
where
    Srgb: IntoColor<T>,
{
    pub foreground_color: T,
    pub background_color: T,
}

const CONTRAST_THRESHOLD: f32 = 7.0;
const DISTANCE_THRESHOLD: f32 = 0.8;

pub fn find_random_fb_color_pair<T>() -> FgBg<T>
where
    Srgb: IntoColor<T>,
{
    loop {
        let fg = random_color::RandomColor::new().to_srgb();
        let bg = random_color::RandomColor::new().to_srgb();
        if fg.relative_contrast(bg) >= CONTRAST_THRESHOLD && fg.distance(bg) > DISTANCE_THRESHOLD {
            return FgBg {
                foreground_color: fg.into_color(),
                background_color: bg.into_color(),
            };
        }
    }
}
