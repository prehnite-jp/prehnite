pub mod container {
    use iced::border::Radius;
    use iced::widget::container;
    use iced::Border;

    pub fn unborder<'a>(
        func: impl Fn(&iced::Theme) -> container::Style + 'a,
    ) -> impl Fn(&iced::Theme) -> container::Style + 'a {
        move |theme| func(theme).border(Border::default())
    }

    pub fn rect_box(theme: &iced::Theme) -> container::Style {
        let mut style = container::bordered_box(theme);
        style.border.radius = Radius::new(0);
        style
    }

    pub fn not_focused_rect_box(theme: &iced::Theme) -> container::Style {
        let mut style = rect_box(theme);
        style = style.background(theme.palette().background);
        style
    }

    pub fn focusable<'a>(is_focused: bool) -> impl Fn(&iced::Theme) -> container::Style + 'a {
        move |theme| {
            if is_focused {
                rect_box(theme)
            } else {
                not_focused_rect_box(theme)
            }
        }
    }
}
