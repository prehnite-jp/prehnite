use crate::db;
use crate::db::query;
use iced::widget::pane_grid::{Axis, ResizeEvent};
use iced::widget::{container, pane_grid, scrollable, space, Container, MouseArea};
use iced::{padding, widget, Background, Element, Length};
use prehnite_core::db::schema::{Item, ItemType};
use prehnite_core::i18n::i18n_w;
use prehnite_core::util::container_style;
use prehnite_core::widget::font::ftext;
use std::collections::HashMap;
use tracing::error;

#[derive(Clone, Debug)]
pub enum ItemListMessage {
    LoadItems,
    ItemListPaneResized(ResizeEvent),
    SetHeadlines(HashMap<i64, Item>),
    SetParagraph(HashMap<i64, HashMap<i64, Item>>),
    ItemSelected(i64),
}

pub enum ItemListActions {
    Run(iced::Task<ItemListMessage>),
}

#[derive(Clone, Debug)]
enum ItemListPane {
    PaneList,
    PaneDetails,
}

#[derive(Debug, Clone)]
pub struct ItemList {
    headlines: HashMap<i64 /* item_id */, Item>,
    paragraph: HashMap<i64 /* headline_id */, HashMap<i64 /* item_id */, Item>>,
    focused_item_id: Option<i64>,
    per_page: u8,
    page: u32,
    item_list_pane: pane_grid::State<ItemListPane>,
}

impl Default for ItemList {
    fn default() -> Self {
        let (mut item_list_pane, pane) = pane_grid::State::new(ItemListPane::PaneList);
        item_list_pane.split(Axis::Vertical, pane, ItemListPane::PaneDetails);
        Self {
            headlines: Default::default(),
            paragraph: Default::default(),
            focused_item_id: None,
            per_page: 10,
            page: 0,
            item_list_pane,
        }
    }
}

impl ItemList {
    #[tracing::instrument]
    async fn load_headlines(page: u32, per_page: u8) -> ItemListMessage {
        ItemListMessage::SetHeadlines(
            query::fetch_root_headline_items(
                db::acquire_with_alert().await.as_mut(),
                per_page,
                page,
            )
            .await
            .unwrap_or_else(|e| {
                error!("Failed to fetch headlines. Error: {e:#?}");
                Default::default()
            }),
        )
    }

    #[tracing::instrument]
    async fn load_paragraph(page: u32, per_page: u8) -> ItemListMessage {
        ItemListMessage::SetParagraph(
            query::fetch_root_headline_related_paragraph(
                db::acquire_with_alert().await.as_mut(),
                per_page,
                page,
            )
            .await
            .unwrap_or_else(|e| {
                error!("Failed to fetch paragraph of headline related. Error: {e:#?}");
                Default::default()
            }),
        )
    }

    pub fn update(&mut self, msg: ItemListMessage) -> ItemListActions {
        match msg {
            ItemListMessage::ItemListPaneResized(ResizeEvent { split, ratio }) => {
                if ratio > 0.33 && ratio < 0.66 {
                    self.item_list_pane.resize(split, ratio);
                }
            }
            ItemListMessage::LoadItems => {
                return ItemListActions::Run(
                    iced::Task::future(Self::load_headlines(self.page, self.per_page)).chain(
                        iced::Task::future(Self::load_paragraph(self.page, self.per_page)),
                    ),
                );
            }
            ItemListMessage::SetHeadlines(v) => {
                self.headlines = v;
                self.focused_item_id = None;
            }
            ItemListMessage::SetParagraph(v) => self.paragraph = v,
            ItemListMessage::ItemSelected(id) => self.focused_item_id = Some(id),
        }
        ItemListActions::Run(iced::Task::none())
    }

    pub fn item(item: &'_ Item, focused: bool) -> Element<'_, ItemListMessage> {
        MouseArea::new(
            Container::new(ftext(item.title.clone()).size(match item.item_type {
                ItemType::Headline(_) => 24,
                ItemType::Paragraph(_) => 18,
            }))
            .padding(padding::left(match item.item_type {
                ItemType::Headline(_) => 0,
                ItemType::Paragraph(_) => 20,
            }))
            .style(move |t| {
                let p = t.extended_palette();
                container::Style {
                    text_color: Some(if focused {
                        p.background.weaker.text
                    } else {
                        p.background.weakest.text
                    }),
                    background: Some(Background::Color(if focused {
                        p.background.weaker.color
                    } else {
                        p.background.weakest.color
                    })),
                    ..Default::default()
                }
            })
            .width(Length::Fill),
        )
        .on_press(ItemListMessage::ItemSelected(item.id))
        .into()
    }

    pub fn item_list_panel(&'_ self) -> Container<'_, ItemListMessage> {
        Container::new(widget::column(self.headlines.iter().map(|(id, itm)| {
            widget::column![
                Self::item(itm, Some(*id) == self.focused_item_id),
                match self.paragraph.get(id) {
                    None => {
                        Element::from(space())
                    }
                    Some(v) => {
                        widget::column(
                            v.iter().map(|(_, itm)| {
                                Self::item(itm, Some(itm.id) == self.focused_item_id)
                            }),
                        )
                        .into()
                    }
                }
            ]
            .into()
        })))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(container_style::rect_bordered)
    }

    fn get_item_paragraph_or_headline(&'_ self) -> Option<&'_ Item> {
        let focused_item_id = self.focused_item_id?;

        self.headlines.get(&focused_item_id).or_else(|| {
            self.paragraph
                .values()
                .find_map(|v| v.get(&focused_item_id))
        })
    }

    fn item_detail(item: &Item) -> Element<'_, ItemListMessage> {
        widget::column![
            i18n_w(item.item_type.as_ref()),
            ftext(&item.title),
            match item.item_type.clone() {
                ItemType::Headline(_) => {
                    None
                }
                ItemType::Paragraph(p) => {
                    p.and_then(|p| p.accepted_draft.map(|d| Element::from(ftext(d.body))))
                }
            }
            .unwrap_or(Element::new(widget::space()))
        ]
        .into()
    }

    pub fn item_detail_panel(&'_ self) -> Container<'_, ItemListMessage> {
        Container::new(match self.get_item_paragraph_or_headline() {
            None => i18n_w("item-no-select").into(),
            Some(item) => Self::item_detail(item),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .style(container_style::rect_bordered)
    }

    pub fn view(&'_ self) -> Element<'_, ItemListMessage> {
        widget::pane_grid(&self.item_list_pane, |_, state, _| {
            pane_grid::Content::new(
                scrollable(match state {
                    ItemListPane::PaneList => self.item_list_panel(),
                    ItemListPane::PaneDetails => self.item_detail_panel(),
                })
                .spacing(1),
            )
        })
        .spacing(2)
        .on_resize(10, ItemListMessage::ItemListPaneResized)
        .into()
    }
}
