mod menubar;
mod page;
mod resources;
mod window;

use crate::app::resources::APP_ICON_PNG;
use crate::app::window::main_window::MainWindow;
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

#[derive(Debug)]
pub struct PrehniteApp {
    window: BTreeMap<iced::window::Id, Box<dyn Window>>,
    window_was_shown: HashSet<iced::window::Id>,
}

#[derive(Clone, Debug)]
pub enum DaemonMessage {
    OpenWindow,
    WindowOpened(iced::window::Id),
    WindowMessage(iced::window::Id, WindowMessage),
    WindowClosed(iced::window::Id)
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
                window: Default::default(),
                window_was_shown: Default::default(),
            },
            Task::done(DaemonMessage::OpenWindow),
        )
    }

    #[tracing::instrument]
    fn update(&mut self, message: DaemonMessage) -> Task<DaemonMessage> {
        match message {
            DaemonMessage::OpenWindow => {
                let mut settings = iced::window::Settings::default();
                settings.icon = from_file_data(APP_ICON_PNG, Some(image::ImageFormat::Png))
                    .or_else(|e| {
                        error!("Failed to setup app icon. Error: {e:#?}");
                        Err(e)
                    })
                    .ok();
                let (window_id, open_window_task) = iced::window::open(settings);
                let (window, init_window_task) = MainWindow::new();
                self.window.insert(window_id, Box::new(window));
                return init_window_task
                    .map(move |msg| DaemonMessage::WindowMessage(window_id, msg))
                    .chain(open_window_task.map(DaemonMessage::WindowOpened));
            }
            DaemonMessage::WindowOpened(id) => {
                self.window_was_shown.insert(id);
            }
            DaemonMessage::WindowMessage(id, window_msg) => {
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
                if self.window.is_empty(){
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

    fn subscription(&self)-> Subscription<DaemonMessage> {
        iced::window::close_events().map(DaemonMessage::WindowClosed)
    }
}
