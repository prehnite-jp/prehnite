use iced::border::Radius;
use iced::widget::container;
use iced::Theme;

pub fn rect_bordered(theme: &Theme) -> container::Style {
    let style = container::bordered_box(theme);
    let border = style.border.rounded(Radius::new(0));
    style.border(border)
}
