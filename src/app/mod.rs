use iced::{Element, Task};
use prehnite::db::{get_database, Database};
use prehnite::i18n::i18n;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct PrehniteApp;

impl PrehniteApp {
    pub fn run() -> Result<(), iced::Error> {
        iced::application(AppDaemon::new, AppDaemon::update, AppDaemon::view).run()
    }
}

enum RootMessage {}

struct AppDaemon {
    database: Arc<Mutex<Database>>,
}

impl AppDaemon {
    fn new() -> (Self, Task<RootMessage>) {
        (
            Self {
                database: get_database(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, _message: RootMessage) -> Task<RootMessage> {
        Task::none()
    }

    fn view(&'_ self) -> Element<'_, RootMessage> {
        iced::widget::text(i18n("wip")).into()
    }
}
