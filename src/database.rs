use anyhow::Result;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database as SeaDataBase, DatabaseConnection};
use std::time::Duration;

#[derive(Debug)]
pub struct DataBase {
    #[allow(dead_code)] // TODO: Remove me
    conn: DatabaseConnection,
}

pub static DB: OnceCell<DataBase> = OnceCell::new();

impl DataBase {
    pub fn global() -> &'static DataBase {
        DB.get().expect("database not initialized")
    }

    pub async fn from_uri(uri: String) -> Result<DataBase> {
        let mut opt = ConnectOptions::new(uri);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false);

        let db = SeaDataBase::connect(opt).await?;
        Ok(DataBase { conn: db })
    }
}
