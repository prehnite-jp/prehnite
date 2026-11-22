
pub trait ToGpui {
    type GpuiType;

    fn to_gpui(&self) -> Self::GpuiType;
}

impl ToGpui for palette::Hsla {
    type GpuiType = gpui_kit::Hsla;

    fn to_gpui(&self) -> Self::GpuiType {
        let palette::Hsla {
            color:
                palette::Hsl {
                    hue,
                    saturation,
                    lightness,
                    standard: _,
                },
            alpha,
        } = self;
        gpui_kit::Hsla {
            h: hue.into_inner(),
            s: *saturation,
            l: *lightness,
            a: *alpha,
        }
    }
}
