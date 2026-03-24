use crate::db::{acquire_book_with_alert, query};
use crate::widget::hideable;
use crate::widget::styles::container::{focusable, not_focused_rect_box, rect_box, unborder};
use iced::widget::pane_grid::{Axis, ResizeEvent};
use iced::widget::{button, pane_grid, scrollable, space, Container, MouseArea};
use iced::{padding, widget, Element, Length};
use iced_aw::menu_items;
use iced_aw::{menu_bar, Menu};
use prehnite_core::db::schema::{Headline, Item, ItemType, Paragraph, ParagraphSummary};
use prehnite_core::i18n::{i18n, i18n_w};
use prehnite_core::widget::font::ftext;
use prehnite_core::font::material_symbol;
use prehnite_core::font::material_symbol::CIRCLE;
use prehnite_core::font::widget::material_symbol;
use std::collections::HashMap;
use tracing::error;
use tracing_unwrap::ResultExt;

#[derive(Clone, Debug)]
pub enum ItemListMessage {
    LoadItems,
    ItemListPaneResized(ResizeEvent),
    SetHeadlines(HashMap<i64, Item>),
    SetParagraph(HashMap<i64, HashMap<i64, Item>>),
    ItemSelected(i64),
    OpenEditor(Option<i64>),
    NewParagraph(i64 /* headline-id */),
    NewHeadline(Option<i64> /* parent-id */),
    None,
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
    not_opened: bool,
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
            not_opened: false,
        }
    }
}

impl ItemList {
    pub fn not_opened() -> Self {
        Self {
            not_opened: true,
            ..Default::default()
        }
    }

    #[tracing::instrument]
    async fn load_headlines(page: u32, per_page: u8) -> ItemListMessage {
        ItemListMessage::SetHeadlines(
            query::fetch_root_headline_items(
                acquire_book_with_alert().await.as_mut(),
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
        let mut conn = acquire_book_with_alert().await;
        let mut res = query::fetch_root_headline_related_paragraph(&mut conn, per_page, page)
            .await
            .unwrap_or_else(|e| {
                error!("Failed to fetch paragraph of headline related. Error: {e:#?}");
                Default::default()
            });
        for i in res.values_mut() {
            for x in i.values_mut() {
                match &mut x.item_type {
                    ItemType::Headline(_) => {}
                    ItemType::Paragraph(v) => {
                        if let Some(v) = v {
                            v.load_summary(&mut conn).await.unwrap_or_else(|e| {
                                error!(
                                    "Failed to fetch references of paragraph related. Error: {e:#?}"
                                );
                            });
                        }
                    }
                }
            }
        }
        ItemListMessage::SetParagraph(res)
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
            ItemListMessage::OpenEditor(_) => { /* handled by daemon */ }
            ItemListMessage::NewParagraph(mut parent_item_id) => {
                if let Some(Item {
                    item_type: ItemType::Paragraph(Some(p)),
                    ..
                }) = self.get_item_paragraph_or_headline(Some(parent_item_id))
                {
                    parent_item_id = p.headline.id;
                };
                return ItemListActions::Run(
                    iced::Task::future(async move {
                        let item = Item {
                            item_type: ItemType::Paragraph(None),
                            title: i18n("no-title"),
                            ..Default::default()
                        };
                        let mut conn = acquire_book_with_alert().await;
                        if let Some(item) = item.register(&mut *conn, false).await.ok_or_log() {
                            let paragraph = Paragraph {
                                item_id: item.id,
                                headline: Headline {
                                    id: parent_item_id,
                                    ..Default::default()
                                },
                                ..Paragraph::default()
                            };
                            if let Some(_) = paragraph.register(&mut *conn, true).await.ok_or_log()
                            {
                                return ItemListMessage::OpenEditor(Some(item.id));
                            }
                        }
                        ItemListMessage::None
                    })
                    .chain(iced::Task::done(ItemListMessage::LoadItems)),
                );
            }
            ItemListMessage::NewHeadline(mut parent_item_id) => {
                if let Some(Item {
                    item_type: ItemType::Paragraph(Some(p)),
                    ..
                }) = self.get_item_paragraph_or_headline(parent_item_id)
                {
                    parent_item_id = Some(p.headline.id);
                };
                return ItemListActions::Run(
                    iced::Task::future(async move {
                        let item = Item {
                            item_type: ItemType::Headline(None),
                            title: i18n("no-title"),
                            ..Default::default()
                        };
                        let mut conn = acquire_book_with_alert().await;
                        if let Some(item) = item.register(&mut *conn, false).await.ok_or_log() {
                            let headline = Headline {
                                item_id: item.id,
                                parent_id: parent_item_id,
                                ..Headline::default()
                            };
                            if let Some(_) = headline.register(&mut *conn, true).await.ok_or_log() {
                                return ItemListMessage::OpenEditor(Some(item.id));
                            }
                        }
                        ItemListMessage::None
                    })
                    .chain(iced::Task::done(ItemListMessage::LoadItems)),
                );
            }
            ItemListMessage::None => {}
        }
        ItemListActions::Run(iced::Task::none())
    }

    pub fn summary<'a>(summary: ParagraphSummary) -> Element<'a, ItemListMessage> {
        widget::row![material_symbol(CIRCLE), ftext(summary.title)].into()
    }

    pub fn item(item: &'_ Item, focused: bool) -> Element<'_, ItemListMessage> {
        let paragraph = item.item_type.clone().paragraph_unwrap_or_default();
        let is_summary_visible = paragraph.is_some();
        MouseArea::new(
            Container::new(widget::column![
                Container::new(widget::column![
                    ftext(item.title.clone()).size(match item.item_type {
                        ItemType::Headline(_) => 24,
                        ItemType::Paragraph(_) => 18,
                    }),
                    hideable(
                        widget::column(
                            paragraph
                                .and_then(|v| v.summary)
                                .map(|v| v
                                    .into_iter()
                                    .map(Self::summary)
                                    .collect::<Vec<Element<'_, ItemListMessage>>>())
                                .unwrap_or_default()
                        )
                        .padding(padding::left(40)),
                        is_summary_visible
                    )
                ])
                .style(unborder(focusable(focused)))
                .width(Length::Fill),
                widget::rule::horizontal(1)
            ])
            .padding(padding::left(match item.item_type {
                ItemType::Headline(_) => 0,
                ItemType::Paragraph(_) => 20,
            }))
            .width(Length::Fill),
        )
        .on_press(ItemListMessage::ItemSelected(item.id))
        .into()
    }

    pub fn item_list_panel(&'_ self) -> Element<'_, ItemListMessage> {
        if self.not_opened {
            widget::container(i18n_w("not-opened"))
                .center(Length::Fill)
                .into()
        } else {
            widget::column![
                widget::column![
                    widget::row![
                        space().width(Length::Fill),
                        menu_bar!((
                            button(material_symbol(material_symbol::ADD).size(20))
                                .style(button::text),
                            {
                                Menu::new(menu_items!(
                                    (button(i18n_w("new-parent-headline"))
                                        .style(button::text)
                                        .on_press(ItemListMessage::NewHeadline(None))),
                                    (button(i18n_w("new-headline"))
                                        .style(button::text)
                                        .on_press_maybe(
                                            self.focused_item_id
                                                .map(|v| ItemListMessage::NewHeadline(Some(v)))
                                        )),
                                    (button(i18n_w("new-paragraph"))
                                        .style(button::text)
                                        .on_press_maybe(
                                            self.focused_item_id
                                                .map(|v| ItemListMessage::NewParagraph(v))
                                        ))
                                ))
                                .width(Length::Shrink)
                            }
                        ))
                    ]
                    .width(Length::Fill),
                    widget::rule::horizontal(1)
                ],
                scrollable(
                    Container::new(widget::column(self.headlines.iter().map(|(id, itm)| {
                        widget::column![
                            Self::item(itm, Some(*id) == self.focused_item_id),
                            match self.paragraph.get(id) {
                                None => {
                                    Element::from(space())
                                }
                                Some(v) => {
                                    widget::column(v.iter().map(|(_, itm)| {
                                        Self::item(itm, Some(itm.id) == self.focused_item_id)
                                    }))
                                    .into()
                                }
                            }
                        ]
                        .into()
                    })))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(5)
                    .style(unborder(not_focused_rect_box))
                )
                .spacing(1)
            ]
            .into()
        }
    }

    fn get_focused_item(&'_ self) -> Option<&'_ Item> {
        self.get_item_paragraph_or_headline(self.focused_item_id)
    }

    fn get_item_paragraph_or_headline(&'_ self, id: Option<i64>) -> Option<&'_ Item> {
        let id = id?;
        self.headlines
            .get(&id)
            .or_else(|| self.paragraph.values().find_map(|v| v.get(&id)))
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

    pub fn item_detail_panel(&'_ self) -> Element<'_, ItemListMessage> {
        scrollable(
            Container::new(match self.get_focused_item() {
                None => i18n_w("item-no-select").into(),
                Some(item) => Self::item_detail(item),
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .style(unborder(rect_box)),
        )
        .spacing(1)
        .into()
    }

    pub fn view(&'_ self) -> Element<'_, ItemListMessage> {
        widget::pane_grid(&self.item_list_pane, |_, state, _| {
            pane_grid::Content::new(match state {
                ItemListPane::PaneList => self.item_list_panel(),
                ItemListPane::PaneDetails => self.item_detail_panel(),
            })
        })
        .spacing(2)
        .on_resize(10, ItemListMessage::ItemListPaneResized)
        .into()
    }
}
