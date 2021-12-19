use std::sync::Arc;
use std::time::Duration;

use eyre::Result;
use log::{error, info};

use super::IoEvent;
use crate::app::state::AppState;
use crate::app::App;

use rusty_bookstore_schema::schema::books_book as book;
use sea_orm::{Database, DatabaseConnection, EntityTrait};

/// In the IO thread, we handle IO event without blocking the UI thread
pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::Sleep(duration) => self.do_sleep(duration).await,
        };

        if let Err(err) = result {
            error!("Oops, something wrong happen: {:?}", err);
        }

        let mut app = self.app.lock().await;
        app.loaded();
    }

    /// We use dummy implementation here, just wait 1s
    async fn do_initialize(&mut self) -> Result<()> {
        info!("🚀 Initialize the application");
        let mut app = self.app.lock().await;

        let db = Database::connect("postgres://user:pass@localhost:15432/postgres").await?;

        app.initialized(db); // we could update the app state
        info!("👍 Application initialized");

        Ok(())
    }

    /// Just take a little break
    async fn do_sleep(&mut self, duration: Duration) -> Result<()> {
        info!("😴 Go sleeping for {:?}...", duration);
        tokio::time::sleep(duration).await;
        info!("⏰ Wake up !");
        // Notify the app for having slept
        let mut app = self.app.lock().await;
        app.slept();

        Ok(())
    }

    pub async fn get_books(&mut self) -> Result<()> {
        if let AppState::Initialized { db, books, .. } = self.app.lock().await.state_mut() {
            let books_query = &book::Entity::find().all(db).await?;
            *books = books_query.to_vec();
        }
        Ok(())
    }
}
