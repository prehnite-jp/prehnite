use crate::app::window::main_window::{BookOpenerMessage, MainWindow, MainWindowMessage};
use iced::Task;
use prehnite_core::util::file_dialog::select_and_open_prehnite_book_file;

pub fn book_opener(main_window: &MainWindow, msg: BookOpenerMessage) -> Task<BookOpenerMessage> {
    select_and_open_prehnite_book_file(main_window.window_id.unwrap(), msg.into()).map(|v| {
        if v.is_success() {
            BookOpenerMessage::Opened
        } else {
            BookOpenerMessage::NotOpened
        }
    })
}

pub fn book_opener_handler(
    main_window: &MainWindow,
    msg: BookOpenerMessage,
) -> Task<MainWindowMessage> {
    match msg {
        BookOpenerMessage::Open | BookOpenerMessage::New => {
            return book_opener(main_window, msg).map(MainWindowMessage::BookOpener);
        }
        BookOpenerMessage::Opened => return Task::done(MainWindowMessage::BookOpened),
        BookOpenerMessage::NotOpened => {}
    }
    Task::none()
}
