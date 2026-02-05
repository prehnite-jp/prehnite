use crate::db;
use crate::db::query;
use iced::widget::pane_grid::{Axis, ResizeEvent};
use iced::widget::{pane_grid, scrollable, Container};
use iced::{widget, Element, Length};
use prehnite_core::db::schema::{Item, ItemType};
use prehnite_core::i18n::i18n_w;
use prehnite_core::util::container_style;
use prehnite_core::widget::item::{ItemRow, ItemRowMessage};
use std::collections::HashMap;
use tracing::error;

#[derive(Clone, Debug)]
pub enum ItemListMessage {
    LoadItems,
    ItemListPaneResized(ResizeEvent),
    SetHeadlines(HashMap<i64, Item>),
    SetParagraph(HashMap<i64, HashMap<i64, Item>>),
    ItemMessage(i64, ItemRowMessage),
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
    item_row_with_headline: HashMap<i64 /* item_id */, (Item, ItemRow)>,
    paragraph: HashMap<i64 /* item_id */, HashMap<i64 /* item_id */, Item /* item_id */>>,
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
            item_row_with_headline: Default::default(),
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
                self.item_row_with_headline = v
                    .into_iter()
                    .map(|(id, v)| (id, (v, ItemRow::new())))
                    .collect();
                self.focused_item_id = None;
            }
            ItemListMessage::ItemMessage(id, msg) => match msg {
                ItemRowMessage::Selected(focused_id) => {
                    self.focused_item_id = Some(focused_id);
                }
                ItemRowMessage::ToggleFoldParagraph => {
                    self.item_row_with_headline
                        .get_mut(&id)
                        .map(|(_, row)| row.toggle_folded());
                }
            },
            ItemListMessage::SetParagraph(v) => {
                self.paragraph = v;
            }
        }
        ItemListActions::Run(iced::Task::none())
    }

    pub fn item_list_panel(&'_ self) -> Container<'_, ItemListMessage> {
        Container::new(widget::column(self.item_row_with_headline.iter().map(
            |(i, (itm, row))| {
                row.view(itm, self.paragraph.get(i), self.focused_item_id)
                    .map(move |msg| ItemListMessage::ItemMessage(*i, msg))
            },
        )))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(container_style::rect_bordered)
    }

    fn get_item_paragraph_or_headline(&'_ self) -> Option<&'_ Item> {
        let focused_item_id = self.focused_item_id?;

        self.item_row_with_headline
            .get(&focused_item_id)
            .map(|v| &v.0)
            .or_else(|| {
                self.paragraph
                    .values()
                    .find_map(|v| v.get(&focused_item_id))
            })
    }

    fn item_detail(item: &Item) -> Element<'_, ItemListMessage> {
        widget::column![
            i18n_w(item.item_type.as_ref()),
            widget::text(&item.title),
            match item.item_type.clone() {
                ItemType::Headline(_) => {
                    None
                }
                ItemType::Paragraph(p) => {
                    p.and_then(|p| {
                        p.accepted_draft
                            .map(|d| Element::from(widget::text(d.body)))
                    })
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
