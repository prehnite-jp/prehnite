use prehnite::db::Database;

pub struct PrehniteApp {
    database: Database,
}

impl PrehniteApp {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}
