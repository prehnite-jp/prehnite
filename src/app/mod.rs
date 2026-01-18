mod page;

use crate::app::page::background_info_editor::{
    BackgroundInfoEditorActions, BackgroundInfoEditorMessage,
};
use crate::app::page::book_not_opened::{BookNotOpenedActions, BookNotOpenedMessage};
use crate::app::page::draft_editor::{DraftEditorActions, DraftEditorMessage};
use crate::app::page::headline_editor::{HeadlineEditorActions, HeadlineEditorMessage};
use crate::app::page::item_list::{ItemListActions, ItemListMessage};
use crate::app::page::paragraph_editor::{ParagraphEditorActions, ParagraphEditorMessage};
use crate::app::page::{PrehnitePage, PrehnitePageId};
use fluent_bundle::FluentArgs;
use iced::border::Radius;
use iced::widget::button;
use iced::{widget, Border, Element, Length, Task};
use iced_aw::menu::Item;
use iced_aw::{menu, menu_bar, menu_items};
use prehnite_core::db::{
    acquire_err_handled, close_book_err_handled, open_book_err_handled, query, DBType,
};
use prehnite_core::i18n::{i18n, i18n_fmt, i18n_w};
use prehnite_core::settings::SettingKey;
use prehnite_core::util::alert::{alert_info_spawn, UnwrapOrErrorAlert};
use tracing::error;

#[derive(Debug)]
pub struct PrehniteApp {
    page: PrehnitePage,
    is_book_opened: bool,
}

#[derive(Clone, Debug)]
pub enum RootMessage {
    None,
    ChangePage(PrehnitePageId),
    BookNotOpened(BookNotOpenedMessage),
    BgInfoEditor(BackgroundInfoEditorMessage),
    DraftEditor(DraftEditorMessage),
    HeadlineEditor(HeadlineEditorMessage),
    ItemList(ItemListMessage),
    ParagraphEditor(ParagraphEditorMessage),
    MenuBar(MenuBarMessage),
    BookOpened,
}

macro_rules! unwrap_page {
    ($self: ident, $x:path) => {{
        match &mut $self.page {
            $x(page) => page,
            _ => {
                error!("invalid message received.");
                return Task::none();
            }
        }
    }};
}

#[derive(Clone, Debug)]
pub enum MenuType {
    File,
    Show,
    Help,
}

#[derive(Clone, Debug)]
pub enum MenuBarMessage {
    MenuBtnPressed(MenuType),
    NewFile,
    OpenFile,
    CloseFile,
    OpenSettings,
    OpenBackgroundInfoEditor,
    OpenBibliographyEditor,
    OpenVersionInfoDialog,
}

macro_rules! menu_button_maybe {
    ($i18n_id:expr, $message:expr) => {
        button(i18n_w($i18n_id))
            .style(button::text)
            .width(150.0f32)
            .on_press_maybe($message)
    };
}

macro_rules! menu_button {
    ($i18n_id:expr, $message:expr) => {
        menu_button_maybe!($i18n_id, Some($message))
    };
}
macro_rules! top_level_menu_button {
    ($i18n_id:expr, $message:expr) => {
        button(i18n_w($i18n_id))
            .style(|t, s| {
                let palette = t.extended_palette();
                button::Style {
                    border: Border::default()
                        .color(palette.background.strong.color)
                        .rounded(Radius::new(0))
                        .width(1),
                    ..button::text(t, s)
                }
            })
            .on_press($message)
    };
}

fn menubar<'a>(is_book_opened: bool) -> Element<'a, MenuBarMessage> {
    let file_menu: Item<MenuBarMessage, _, _> = Item::with_menu(
        top_level_menu_button!("file", MenuBarMessage::MenuBtnPressed(MenuType::File)),
        menu!(
            (menu_button!("new-file", MenuBarMessage::NewFile)),
            (menu_button!("open-file", MenuBarMessage::OpenFile)),
            (menu_button_maybe!(
                "close-file",
                is_book_opened.then_some(MenuBarMessage::CloseFile)
            )),
            (widget::rule::horizontal(1)),
            (menu_button!("settings", MenuBarMessage::OpenSettings))
        )
        .max_width(180.0f32),
    );
    let show_menu: Item<MenuBarMessage, _, _> = Item::with_menu(
        top_level_menu_button!("show", MenuBarMessage::MenuBtnPressed(MenuType::Show)),
        menu!(
            (menu_button!(
                "background-info-editor",
                MenuBarMessage::OpenBackgroundInfoEditor
            )),
            (menu_button!(
                "bibliography-editor",
                MenuBarMessage::OpenBibliographyEditor
            )),
        )
        .max_width(180.0f32),
    );
    let help_menu: Item<MenuBarMessage, _, _> = Item::with_menu(
        top_level_menu_button!("help", MenuBarMessage::MenuBtnPressed(MenuType::Help)),
        menu!((menu_button!("version-info", MenuBarMessage::OpenVersionInfoDialog)))
            .max_width(180.0f32),
    );
    let menu_bar = menu_bar![file_menu, show_menu, help_menu].close_on_item_click_global(true);
    widget::column![menu_bar, widget::rule::horizontal(1)]
        .width(Length::Fill)
        .into()
}

impl PrehniteApp {
    pub fn run() -> Result<(), iced::Error> {
        iced::application(Self::new, Self::update, Self::view).run()
    }

    #[tracing::instrument]
    async fn open_last_opened_book() -> RootMessage {
        let mut conn = acquire_err_handled(DBType::AppGlobal)
            .await
            .unwrap_or_alert();
        let last_opened = query::fetch_setting(&mut conn, SettingKey::LastOpened)
            .await
            .unwrap_or_else(|e| {
                error!("Failed to fetch last opened settings. Error: {:#?}", e);
                None
            });
        fn return_err() -> RootMessage {
            RootMessage::ChangePage(PrehnitePageId::BookNotOpened)
        }
        match last_opened
            .and_then(|v| v.setting_value)
            .and_then(|v| v.parse().ok())
        {
            None => return_err(),
            Some(v) => {
                if open_book_err_handled(v).await {
                    RootMessage::BookOpened
                } else {
                    return_err()
                }
            }
        }
    }

    fn new() -> (Self, Task<RootMessage>) {
        (
            Self {
                page: Default::default(),
                is_book_opened: false,
            },
            Task::future(Self::open_last_opened_book()),
        )
    }

    #[tracing::instrument]
    fn update(&mut self, message: RootMessage) -> Task<RootMessage> {
        match message {
            RootMessage::None => {}
            RootMessage::BookNotOpened(msg) => {
                let page = unwrap_page!(self, PrehnitePage::BookNotOpened);
                match page.update(msg) {
                    BookNotOpenedActions::Run(v) => return v.map(RootMessage::BookNotOpened),
                    BookNotOpenedActions::Opened => return Task::done(RootMessage::BookOpened),
                    BookNotOpenedActions::NotOpened => {}
                };
            }
            RootMessage::BgInfoEditor(msg) => {
                let page = unwrap_page!(self, PrehnitePage::BgInfoEditor);
                match page.update(msg) {
                    BackgroundInfoEditorActions::None => {}
                }
            }
            RootMessage::DraftEditor(msg) => {
                let page = unwrap_page!(self, PrehnitePage::DraftEditor);
                match page.update(msg) {
                    DraftEditorActions::None => {}
                }
            }
            RootMessage::HeadlineEditor(msg) => {
                let page = unwrap_page!(self, PrehnitePage::HeadlineEditor);
                match page.update(msg) {
                    HeadlineEditorActions::None => {}
                }
            }
            RootMessage::ItemList(msg) => {
                let page = unwrap_page!(self, PrehnitePage::ItemList);
                return match page.update(msg) {
                    ItemListActions::Run(v) => v.map(RootMessage::ItemList),
                };
            }
            RootMessage::ParagraphEditor(msg) => {
                let page = unwrap_page!(self, PrehnitePage::ParagraphEditor);
                match page.update(msg) {
                    ParagraphEditorActions::None => {}
                }
            }
            RootMessage::ChangePage(page) => {
                self.page = page.clone().into();
                match page {
                    PrehnitePageId::NowLoading => {}
                    PrehnitePageId::BookNotOpened => {}
                    PrehnitePageId::BgInfoEditor => {}
                    PrehnitePageId::DraftEditor => {}
                    PrehnitePageId::HeadlineEditor => {}
                    PrehnitePageId::ItemList => {
                        return Task::done(RootMessage::ItemList(ItemListMessage::LoadItems));
                    }
                    PrehnitePageId::ParagraphEditor => {}
                }
            }
            RootMessage::MenuBar(v) => match v {
                MenuBarMessage::MenuBtnPressed(menu_type) => match menu_type {
                    MenuType::File => {}
                    MenuType::Show => {}
                    MenuType::Help => {}
                },
                MenuBarMessage::NewFile => {
                    return Task::done(RootMessage::BookNotOpened(BookNotOpenedMessage::New));
                }
                MenuBarMessage::OpenFile => {
                    return Task::done(RootMessage::BookNotOpened(BookNotOpenedMessage::Open));
                }
                MenuBarMessage::CloseFile => {
                    self.is_book_opened = false;
                    return Task::future(async {
                        close_book_err_handled().await;
                        RootMessage::ChangePage(PrehnitePageId::BookNotOpened)
                    });
                }
                MenuBarMessage::OpenSettings => {}
                MenuBarMessage::OpenBackgroundInfoEditor => {}
                MenuBarMessage::OpenBibliographyEditor => {}
                MenuBarMessage::OpenVersionInfoDialog => {
                    return Task::future(async {
                        let mut args = FluentArgs::new();
                        args.set("app-name", env!("CARGO_PKG_NAME"));
                        args.set("version", env!("CARGO_PKG_VERSION"));
                        alert_info_spawn((
                            i18n("version-info").as_str(),
                            i18n_fmt("version-info-detail", Some(&args)).as_str(),
                        ))
                        .await;
                        RootMessage::None
                    });
                }
            },
            RootMessage::BookOpened => {
                self.is_book_opened = true;
                return Task::done(RootMessage::ChangePage(PrehnitePageId::ItemList));
            }
        }
        Task::none()
    }

    #[tracing::instrument]
    fn view(&'_ self) -> Element<'_, RootMessage> {
        iced::widget::column![
            menubar(self.is_book_opened).map(RootMessage::MenuBar),
            match &self.page {
                PrehnitePage::NowLoading => i18n_w("now-loading").into(),
                PrehnitePage::BookNotOpened(v) => v.view().map(RootMessage::BookNotOpened),
                PrehnitePage::BgInfoEditor(v) => v.view().map(RootMessage::BgInfoEditor),
                PrehnitePage::DraftEditor(v) => v.view().map(RootMessage::DraftEditor),
                PrehnitePage::HeadlineEditor(v) => v.view().map(RootMessage::HeadlineEditor),
                PrehnitePage::ItemList(v) => v.view().map(RootMessage::ItemList),
                PrehnitePage::ParagraphEditor(v) => v.view().map(RootMessage::ParagraphEditor),
            }
        ]
        .into()
    }
}
