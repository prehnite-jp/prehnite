pub mod resources;
mod window;

use crate::app::window::editor_window::{EditorWindow, EditorWindowMessage};
use crate::app::window::license_info_window::LicenseInfoWindow;
use crate::app::window::main_window::page::item_list::ItemListMessage;
use crate::app::window::main_window::{MainWindow, MainWindowMessage};
use crate::app::window::setting_window::SettingWindow;
use crate::app::window::version_info_window::VersionInfoWindow;
use crate::app::window::{Window, WindowMessage};
use iced::border::Radius;
use iced::widget::{button, space};
use iced::{Border, Element, Font, Subscription, Task};
use prehnite_core::i18n::{change_lang_bundle, i18n_w, DEFAULT_LANG_ID};
use prehnite_core::opt_unwrap_or_return;
use prehnite_core::settings::registry::SettingRegistry;
use prehnite_core::settings::GlobalSettingKey;
use prehnite_core::widget::font::{get_default_font, set_default_font, set_font};
use prehnite_core::font::{get_default_font_family, get_global_font_list, FontLoader};
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
use tracing::error;

#[derive(Clone, Debug)]
pub enum WindowType {
    MainWindow,
    VersionInfoWindow,
    SettingWindow,
    LicenseInfoWindow,
    BiblioGraphyEditorWindow,
    BackgroundInfoEditorWindow,
    EditorWindow(i64),
}

macro_rules! window_opener {
    ($self:ident, $(($window_type:pat, $window_struct:path)),*) => {
        match $self{
            $(
            $window_type => <$window_struct>::window_settings(),
            )*
        }
    };
}

impl WindowType {
    pub fn open_window(&self) -> (iced::window::Id, Task<iced::window::Id>) {
        iced::window::open(window_opener!(
            self,
            (WindowType::MainWindow, MainWindow),
            (WindowType::VersionInfoWindow, VersionInfoWindow),
            (WindowType::SettingWindow, SettingWindow),
            (WindowType::LicenseInfoWindow, LicenseInfoWindow),
            (WindowType::BiblioGraphyEditorWindow, LicenseInfoWindow), // TODO
            (WindowType::BackgroundInfoEditorWindow, LicenseInfoWindow), // TODO
            (WindowType::EditorWindow(_), EditorWindow)
        ))
    }
}

#[derive(Debug)]
struct TypedWindow {
    pub w_type: WindowType,
    pub window: Box<dyn Window>,
}

impl From<(WindowType, Box<dyn Window>)> for TypedWindow {
    fn from((w_type, window): (WindowType, Box<dyn Window>)) -> Self {
        Self { w_type, window }
    }
}

impl From<TypedWindow> for (WindowType, Box<dyn Window>) {
    fn from(value: TypedWindow) -> Self {
        (value.w_type, value.window)
    }
}

#[derive(Debug)]
pub struct PrehniteApp {
    main_window_id: Option<iced::window::Id>,
    version_info_window_id: Option<iced::window::Id>,
    setting_window_id: Option<iced::window::Id>,
    window: BTreeMap<iced::window::Id, TypedWindow>,
    window_was_shown: HashSet<iced::window::Id>,
    license_info_window_id: Option<iced::window::Id>,
    background_info_editor_window_id: Option<iced::window::Id>,
    bibliography_editor_window_id: Option<iced::window::Id>,
    editor_window_id: Option<iced::window::Id>,
}

#[derive(Clone, Debug)]
pub enum DaemonMessage {
    OpenWindow(WindowType),
    WindowOpened(iced::window::Id),
    WindowMessage(iced::window::Id, WindowMessage),
    WindowClosed(iced::window::Id),
    ReloadFont,
}

macro_rules! window_creator {
    ($v_window_type:expr,$(($window_type:pat, $window_struct:path)),*) => {
        match $v_window_type {
            $(
            $window_type => {
                (Box::new(<$window_struct>::new()), <$window_struct>::init_task())
            }
            )*
        }
    };
}

impl PrehniteApp {
    pub fn run() -> Result<(), iced::Error> {
        set_default_font(Font::with_name(get_default_font_family()));
        iced::daemon(Self::new, Self::update, Self::view)
            .title(Self::title)
            .subscription(Self::subscription)
            .load_all_prehnite_bundled_font()
            .default_font(get_default_font())
            .run()
    }

    fn new() -> (Self, Task<DaemonMessage>) {
        (
            Self {
                main_window_id: None,
                version_info_window_id: None,
                setting_window_id: None,
                window: Default::default(),
                window_was_shown: Default::default(),
                license_info_window_id: None,
                background_info_editor_window_id: None,
                bibliography_editor_window_id: None,
                editor_window_id: None,
            },
            Task::done(DaemonMessage::ReloadFont).chain(Task::done(DaemonMessage::OpenWindow(
                WindowType::MainWindow,
            ))),
        )
    }

    fn before_window_open(&mut self, window_type: &WindowType) -> Option<Task<DaemonMessage>> {
        match window_type {
            WindowType::MainWindow => self.main_window_id.map(iced::window::gain_focus),
            WindowType::VersionInfoWindow => {
                self.version_info_window_id.map(iced::window::gain_focus)
            }
            WindowType::SettingWindow => self.setting_window_id.map(iced::window::gain_focus),
            WindowType::LicenseInfoWindow => {
                self.license_info_window_id.map(iced::window::gain_focus)
            }
            WindowType::BiblioGraphyEditorWindow => self
                .bibliography_editor_window_id
                .map(iced::window::gain_focus),
            WindowType::BackgroundInfoEditorWindow => self
                .background_info_editor_window_id
                .map(iced::window::gain_focus),
            WindowType::EditorWindow(_) => self.editor_window_id.map(iced::window::gain_focus),
        }
    }

    fn on_window_close(&mut self, window: Option<TypedWindow>) -> Task<DaemonMessage> {
        if let Some((w_type, _)) = window.map(|v| v.into()) {
            match w_type {
                WindowType::MainWindow => return iced::exit(),
                WindowType::VersionInfoWindow => self.version_info_window_id = None,
                WindowType::SettingWindow => self.setting_window_id = None,
                WindowType::LicenseInfoWindow => self.license_info_window_id = None,
                WindowType::BiblioGraphyEditorWindow => self.bibliography_editor_window_id = None,
                WindowType::BackgroundInfoEditorWindow => {
                    self.background_info_editor_window_id = None
                }
                WindowType::EditorWindow(_) => self.editor_window_id = None,
            }
        }
        Task::none()
    }

    #[tracing::instrument]
    fn update(&mut self, message: DaemonMessage) -> Task<DaemonMessage> {
        match message {
            DaemonMessage::OpenWindow(w_type) => {
                if let Some(task) = self.before_window_open(&w_type) {
                    return task;
                }
                let (window_id, open_window_task) = w_type.open_window();

                // ウィンドウを構築
                let (mut window, mut init_window_task): (Box<dyn Window>, Task<WindowMessage>) = window_creator!(
                    w_type,
                    (WindowType::MainWindow, MainWindow),
                    (WindowType::VersionInfoWindow, VersionInfoWindow),
                    (WindowType::SettingWindow, SettingWindow),
                    (WindowType::LicenseInfoWindow, LicenseInfoWindow),
                    (WindowType::BiblioGraphyEditorWindow, LicenseInfoWindow), // TODO
                    (WindowType::BackgroundInfoEditorWindow, LicenseInfoWindow), // TODO
                    (WindowType::EditorWindow(_), EditorWindow)
                );

                // 最大1つまでに限定されているウィンドウのIDを保持
                match w_type {
                    WindowType::MainWindow => self.main_window_id = Some(window_id),
                    WindowType::VersionInfoWindow => self.version_info_window_id = Some(window_id),
                    WindowType::SettingWindow => self.setting_window_id = Some(window_id),
                    WindowType::LicenseInfoWindow => self.license_info_window_id = Some(window_id),
                    WindowType::BiblioGraphyEditorWindow => {
                        self.bibliography_editor_window_id = Some(window_id);
                    }
                    WindowType::BackgroundInfoEditorWindow => {
                        self.background_info_editor_window_id = Some(window_id);
                    }
                    WindowType::EditorWindow(_) => self.editor_window_id = Some(window_id),
                };

                // その他特殊処理
                match w_type {
                    WindowType::EditorWindow(id) => {
                        init_window_task = Task::done(WindowMessage::EditorWindowMessage(
                            EditorWindowMessage::ChangeItemFromId(id),
                        ));
                    }
                    _ => {}
                }

                // ウィンドウを登録し、開く
                window.set_window_id(window_id);
                self.window.insert(window_id, (w_type, window).into());
                init_window_task
                    .map(move |msg| DaemonMessage::WindowMessage(window_id, msg))
                    .chain(open_window_task.map(DaemonMessage::WindowOpened))
            }
            DaemonMessage::WindowOpened(id) => {
                self.window_was_shown.insert(id);
                iced::window::gain_focus(id)
            }
            DaemonMessage::WindowMessage(id, window_msg) => {
                // デーモンに移譲されたメッセージを処理
                match &window_msg {
                    WindowMessage::MainWindowMessage(MainWindowMessage::OpenVersionInfoWindow) => {
                        return Task::done(DaemonMessage::OpenWindow(
                            WindowType::VersionInfoWindow,
                        ));
                    }
                    WindowMessage::MainWindowMessage(MainWindowMessage::OpenSettingWindow) => {
                        return Task::done(DaemonMessage::OpenWindow(WindowType::SettingWindow));
                    }
                    WindowMessage::MainWindowMessage(MainWindowMessage::OpenLicenseInfoWindow) => {
                        return Task::done(DaemonMessage::OpenWindow(
                            WindowType::LicenseInfoWindow,
                        ));
                    }
                    WindowMessage::MainWindowMessage(
                        MainWindowMessage::OpenBibliographyEditorWindow,
                    ) => {
                        return Task::done(DaemonMessage::OpenWindow(
                            WindowType::BiblioGraphyEditorWindow,
                        ));
                    }
                    WindowMessage::MainWindowMessage(
                        MainWindowMessage::OpenBackgroundInfoEditorWindow,
                    ) => {
                        return Task::done(DaemonMessage::OpenWindow(
                            WindowType::BackgroundInfoEditorWindow,
                        ));
                    }
                    WindowMessage::MainWindowMessage(MainWindowMessage::OpenEditorWindow(id)) => {
                        return Task::done(DaemonMessage::OpenWindow(WindowType::EditorWindow(
                            *id,
                        )));
                    }
                    WindowMessage::ReloadFont => {
                        return Task::done(DaemonMessage::ReloadFont);
                    }
                    WindowMessage::ReloadLanguage => {
                        return Task::future(async {
                            let lang_id = SettingRegistry::get(&GlobalSettingKey::Locale.into())
                                .and_then(|v| v.get::<String>())
                                .unwrap_or(DEFAULT_LANG_ID.to_string());
                            change_lang_bundle(lang_id.as_str()).await
                        })
                        .discard();
                    }
                    WindowMessage::MainWindowMessage(MainWindowMessage::ItemList(
                        ItemListMessage::OpenEditor(Some(id)),
                    )) => {
                        return Task::done(DaemonMessage::OpenWindow(WindowType::EditorWindow(
                            *id,
                        )));
                    }
                    _ => {}
                }
                // ウィンドウごとのメッセージを処理
                let window = opt_unwrap_or_return!(self.window.get_mut(&id), {
                    error!("Failed to get window. WindowId: {id}");
                    Task::none()
                });
                window
                    .window
                    .update(window_msg)
                    .map(move |msg| DaemonMessage::WindowMessage(id, msg))
            }
            DaemonMessage::WindowClosed(id) => {
                let typed_window = self.window.remove(&id);
                // ウィンドウが存在しないならアプリケーションを終了
                if self.window.is_empty() {
                    return iced::exit();
                }
                self.on_window_close(typed_window)
            }
            DaemonMessage::ReloadFont => {
                let v = SettingRegistry::get(&GlobalSettingKey::Font.into())
                    .and_then(|v| v.get::<String>())
                    .and_then(|v| get_global_font_list().iter().filter(|x| **x == v).next());
                set_font(v);
                Task::none()
            }
        }
    }

    #[tracing::instrument]
    fn view(&'_ self, window_id: iced::window::Id) -> Element<'_, DaemonMessage> {
        let v = opt_unwrap_or_return!(self.window.get(&window_id), Element::new(space()));
        v.window
            .view()
            .map(move |msg| DaemonMessage::WindowMessage(window_id, msg))
    }

    fn title(&self, window_id: iced::window::Id) -> String {
        let v = opt_unwrap_or_return!(self.window.get(&window_id), "unknown".into());
        v.window.title()
    }

    fn subscription(&self) -> Subscription<DaemonMessage> {
        iced::window::close_events().map(DaemonMessage::WindowClosed)
    }
}
