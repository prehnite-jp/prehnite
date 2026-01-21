pub mod main_window;
mod menubar;
pub mod resources;
pub mod version_info_window;

use crate::app::window::main_window::MainWindowMessage;
use crate::app::window::version_info_window::VersionInfoWindowMessage;
use iced::{window, Element, Size, Task};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum WindowMessage {
    MainWindowMessage(MainWindowMessage),
    AboutWindowMessage(VersionInfoWindowMessage),
}

pub trait Window: Debug {
    fn new() -> (Box<dyn Window>, Task<WindowMessage>)
    where
        Self: Sized;

    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage>;

    fn view(&'_ self) -> Element<'_, WindowMessage>;

    fn title(&'_ self) -> String;

    fn set_window_id(&mut self, window_id: window::Id);

    fn default_resizable() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn default_size() -> Size
    where
        Self: Sized,
    {
        Size::new(1024.0, 768.0)
    }
}
