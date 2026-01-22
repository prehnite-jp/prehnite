mod window;

use crate::app::window::main_window::{MainWindow, MainWindowMessage};
use crate::app::window::version_info_window::VersionInfoWindow;
use crate::app::window::{Window, WindowMessage};
use iced::border::Radius;
use iced::widget::{button, space};
use iced::{Border, Element, Subscription, Task};
use prehnite_core::i18n::i18n_w;
use prehnite_core::opt_unwrap_or_return;
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
use tracing::error;

#[derive(Clone, Debug)]
pub enum WindowType {
    MainWindow,
    VersionInfoWindow,
}

impl WindowType {
    pub fn open_window(&self) -> (iced::window::Id, Task<iced::window::Id>) {
        iced::window::open(match self {
            WindowType::MainWindow => MainWindow::window_settings(),
            WindowType::VersionInfoWindow => VersionInfoWindow::window_settings(),
        })
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
    window: BTreeMap<iced::window::Id, TypedWindow>,
    window_was_shown: HashSet<iced::window::Id>,
}

#[derive(Clone, Debug)]
pub enum DaemonMessage {
    OpenWindow(WindowType),
    WindowOpened(iced::window::Id),
    WindowMessage(iced::window::Id, WindowMessage),
    WindowClosed(iced::window::Id),
}

impl PrehniteApp {
    pub fn run() -> Result<(), iced::Error> {
        iced::daemon(Self::new, Self::update, Self::view)
            .title(Self::title)
            .subscription(Self::subscription)
            .run()
    }

    fn new() -> (Self, Task<DaemonMessage>) {
        (
            Self {
                main_window_id: None,
                version_info_window_id: None,
                window: Default::default(),
                window_was_shown: Default::default(),
            },
            Task::done(DaemonMessage::OpenWindow(WindowType::MainWindow)),
        )
    }

    fn before_window_open(&mut self, window_type: &WindowType) -> Option<Task<DaemonMessage>> {
        match window_type {
            WindowType::MainWindow => self
                .main_window_id
                .and_then(|id| Some(iced::window::gain_focus(id))),
            WindowType::VersionInfoWindow => self
                .version_info_window_id
                .and_then(|id| Some(iced::window::gain_focus(id))),
        }
    }

    fn on_window_close(&mut self, window: Option<TypedWindow>) -> Task<DaemonMessage> {
        if let Some((w_type, _)) = window.map(|v| v.into()) {
            match w_type {
                WindowType::MainWindow => return iced::exit(),
                WindowType::VersionInfoWindow => self.version_info_window_id = None,
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

                // 指定されたタイプで構築
                let (mut window, init_window_task) = match w_type {
                    WindowType::MainWindow => {
                        self.main_window_id = Some(window_id);
                        MainWindow::new()
                    }
                    WindowType::VersionInfoWindow => {
                        self.version_info_window_id = Some(window_id);
                        VersionInfoWindow::new()
                    }
                };

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
                if let WindowMessage::MainWindowMessage(MainWindowMessage::OpenVersionInfoWindow) =
                    window_msg
                {
                    return Task::done(DaemonMessage::OpenWindow(WindowType::VersionInfoWindow));
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
