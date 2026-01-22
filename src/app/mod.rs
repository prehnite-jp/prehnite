mod page;
mod window;

use crate::app::window::main_window::{MainWindow, MainWindowMessage};
use crate::app::window::version_info_window::VersionInfoWindow;
use crate::app::window::{Window, WindowMessage};
use iced::border::Radius;
use iced::widget::{button, space};
use iced::window::icon::from_file_data;
use iced::{Border, Element, Subscription, Task};
use prehnite_core::i18n::i18n_w;
use prehnite_core::opt_unwrap_or_return;
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
use tracing::error;
use window::resources::APP_ICON_PNG;

#[derive(Debug)]
pub struct PrehniteApp {
    main_window_id: Option<iced::window::Id>,
    window: BTreeMap<iced::window::Id, Box<dyn Window>>,
    window_was_shown: HashSet<iced::window::Id>,
}

#[derive(Clone, Debug)]
pub enum WindowType {
    MainWindow,
    VersionInfoWindow,
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
                window: Default::default(),
                window_was_shown: Default::default(),
            },
            Task::done(DaemonMessage::OpenWindow(WindowType::MainWindow)),
        )
    }

    #[tracing::instrument]
    fn update(&mut self, message: DaemonMessage) -> Task<DaemonMessage> {
        match message {
            DaemonMessage::OpenWindow(w_type) => {
                // メインウィンドウは一つまで
                if let WindowType::MainWindow = w_type {
                    if let Some(_) = self.main_window_id {
                        return Task::none();
                    }
                }

                // ウィンドウを構成
                // 設定を作成
                let mut settings = match w_type {
                    WindowType::MainWindow => MainWindow::window_settings(),
                    WindowType::VersionInfoWindow => VersionInfoWindow::window_settings(),
                };
                // ウィンドウを開く
                let (window_id, open_window_task) = iced::window::open(settings);

                // 指定されたタイプで構築
                let (mut window, init_window_task) = match w_type {
                    WindowType::MainWindow => {
                        self.main_window_id = Some(window_id);
                        MainWindow::new()
                    }
                    WindowType::VersionInfoWindow => VersionInfoWindow::new(),
                };

                // ウィンドウを登録し、開く
                window.set_window_id(window_id);
                self.window.insert(window_id, window);
                return init_window_task
                    .map(move |msg| DaemonMessage::WindowMessage(window_id, msg))
                    .chain(open_window_task.map(DaemonMessage::WindowOpened));
            }
            DaemonMessage::WindowOpened(id) => {
                self.window_was_shown.insert(id);
                return iced::window::gain_focus(id);
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
                return window
                    .update(window_msg)
                    .map(move |msg| DaemonMessage::WindowMessage(id, msg));
            }
            DaemonMessage::WindowClosed(id) => {
                self.window.remove(&id);
                if self.window.is_empty() || Some(id) == self.main_window_id {
                    return iced::exit();
                }
            }
        }
        Task::none()
    }

    #[tracing::instrument]
    fn view(&'_ self, window_id: iced::window::Id) -> Element<'_, DaemonMessage> {
        let v = opt_unwrap_or_return!(self.window.get(&window_id), Element::new(space()));
        v.view()
            .map(move |msg| DaemonMessage::WindowMessage(window_id, msg))
    }

    fn title(&self, window_id: iced::window::Id) -> String {
        let v = opt_unwrap_or_return!(self.window.get(&window_id), "unknown".into());
        v.title()
    }

    fn subscription(&self) -> Subscription<DaemonMessage> {
        iced::window::close_events().map(DaemonMessage::WindowClosed)
    }
}
