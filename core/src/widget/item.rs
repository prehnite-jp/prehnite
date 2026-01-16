use crate::db::schema::{Item, ItemType};
use crate::i18n::i18n;
use iced::{widget, Length};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum ItemRowMessage {
    Selected(i64),
    ToggleFoldParagraph,
}

#[derive(Debug, Clone)]
pub struct ItemRow {
    folded: bool,
}

pub fn task_progress_status_str(item: &Item) -> String {
    let (finished, total) = match &item.tasks {
        None => (0, 0),
        Some(v) => (v.iter().filter(|v| !v.is_finished).count(), v.len()),
    };
    if total == 0 {
        "-".into()
    } else {
        format!("({finished}/{total})",)
    }
}

impl ItemRow {
    pub fn new() -> Self {
        Self { folded: false }
    }

    fn headline_view(item: &Item) -> iced::Element<'_, ItemRowMessage> {
        widget::column![
            widget::text(&item.title)
                .size(20)
                .wrapping(widget::text::Wrapping::None),
            widget::text!["{}: {}", i18n("task"), task_progress_status_str(item)]
                .wrapping(widget::text::Wrapping::None)
        ]
        .padding(10)
        .into()
    }

    fn paragraph_view(item: &Item) -> iced::Element<'_, ItemRowMessage> {
        widget::column![
            widget::text(&item.title)
                .size(20)
                .wrapping(widget::text::Wrapping::None),
            widget::text!["{}: {}", i18n("task"), task_progress_status_str(item)]
                .wrapping(widget::text::Wrapping::None),
        ]
        .padding(10)
        .into()
    }

    fn paragraph_list_view(
        list: &HashMap<i64, Item>,
        focused_item_id: Option<i64>,
    ) -> iced::Element<'_, ItemRowMessage> {
        widget::Container::new(widget::Column::with_children(list.iter().filter_map(
            |(_, v)| {
                match &v.item_type {
                    ItemType::Paragraph(_) => Some(
                        Self::row_container(
                            v.id,
                            Self::paragraph_view(v),
                            focused_item_id
                                .clone()
                                .map(|focused| v.id == focused)
                                .unwrap_or(false),
                        )
                        .into(),
                    ),
                    _ => None,
                }
            },
        )))
        .into()
    }

    fn row_container(
        item_id: i64,
        element: iced::Element<ItemRowMessage>,
        focused: bool,
    ) -> widget::MouseArea<ItemRowMessage> {
        widget::mouse_area(
            widget::Container::new(element)
                .clip(true)
                .width(Length::Fill)
                .style(move |v| {
                    let style = widget::container::bordered_box(v);
                    let mut border = style.border;
                    if focused {
                        border = border.color(iced::color!(0x000080));
                    }
                    style.border(border)
                }),
        )
        .on_press(ItemRowMessage::Selected(item_id))
    }

    pub fn toggle_folded(&mut self) {
        self.folded = !self.folded;
    }

    pub fn view<'a>(
        &'a self,
        headline: &'a Item,
        paragraph: Option<&'a HashMap<i64, Item>>,
        focused_item_id: Option<i64>,
    ) -> iced::Element<'a, ItemRowMessage> {
        Self::row_container(
            headline.id,
            widget::column![
                Self::headline_view(headline),
                if self.folded || paragraph.is_none() {
                    iced::Element::new(widget::space())
                } else {
                    Self::paragraph_list_view(paragraph.unwrap(), focused_item_id)
                }
            ]
            .padding(10)
            .into(),
            focused_item_id
                .map(|focused| headline.id == focused)
                .unwrap_or(false),
        )
        .on_double_click(ItemRowMessage::ToggleFoldParagraph)
        .into()
    }
}
