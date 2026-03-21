use crate::app::window::{app_default_window_settings, Window, WindowMessage};
use crate::db::acquire_book_with_alert;
use iced::window::Settings;
use iced::{window, Element, Size, Task};
use prehnite_core::db::schema::Item;
use prehnite_core::i18n::{i18n, i18n_w};

// TODO: 実装
#[derive(Clone, Debug)]
pub enum EditorWindowMessage {
    Close,
    ChangeItem(Option<Item>),
    ChangeItemFromId(i64),
}

#[derive(Debug)]
pub struct EditorWindow {
    window_id: Option<window::Id>,
    current_item: Option<Item>,
}

impl EditorWindow {
    fn update_impl(&mut self, message: EditorWindowMessage) -> Task<EditorWindowMessage> {
        match message {
            EditorWindowMessage::Close => window::close(self.window_id.unwrap()),
            EditorWindowMessage::ChangeItem(v) => {
                if v != None {
                    self.current_item = v;
                    Task::none()
                } else {
                    Task::done(EditorWindowMessage::Close)
                }
            }
            EditorWindowMessage::ChangeItemFromId(id) => Task::future(async move {
                let mut conn = acquire_book_with_alert().await;
                EditorWindowMessage::ChangeItem(
                    Item::from_id(&mut *conn, id).await.unwrap_or_default(),
                )
            }),
        }
    }

    fn view_impl(&'_ self) -> Element<'_, EditorWindowMessage> {
        i18n_w("wip").into()
    }
}

impl Window for EditorWindow {
    fn new() -> Self
    where
        Self: Sized,
    {
        EditorWindow {
            window_id: None,
            current_item: None,
        }
    }

    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage> {
        if let WindowMessage::EditorWindowMessage(msg) = message {
            self.update_impl(msg)
                .map(WindowMessage::EditorWindowMessage)
        } else {
            Task::none()
        }
    }

    fn view(&'_ self) -> Element<'_, WindowMessage> {
        self.view_impl().map(WindowMessage::EditorWindowMessage)
    }

    fn title(&'_ self) -> String {
        i18n("wip")
    }

    fn set_window_id(&mut self, window_id: window::Id) {
        self.window_id = Some(window_id)
    }

    fn window_settings() -> Settings
    where
        Self: Sized,
    {
        Settings {
            size: Size::new(400.0f32, 400.0f32),
            resizable: true,
            minimizable: false,
            ..app_default_window_settings()
        }
    }
}
