pub mod main_window;
mod menubar;
pub mod resources;

use crate::app::window::main_window::MainWindowMessage;
use iced::{window, Element, Task};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum WindowMessage {
    MainWindowMessage(MainWindowMessage),
}

pub trait Window: Debug {
    fn new() -> (Box<dyn Window>, Task<WindowMessage>)
    where
        Self: Sized;

    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage>;

    fn view(&'_ self) -> Element<'_, WindowMessage>;

    fn title(&'_ self) -> String;

    fn set_window_id(&mut self, window_id: window::Id);
}
