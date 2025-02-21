use std::env;

use sqlx::SqlitePool;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::farmer_john::{Event, Message};

pub struct Barn {
    pool: SqlitePool,
    event_recv: UnboundedReceiver<Event>,
    message_send: UnboundedSender<Message>,
}

impl Barn {
    pub async fn new(
        event_recv: UnboundedReceiver<Event>,
        message_send: UnboundedSender<Message>,
    ) -> Barn {
        let path = &env::var("DATABASE_URL").expect("env variable `DATABASE_URL` should be set");
        let pool = SqlitePool::connect(&path)
            .await
            .expect("The connection to the database should be successful");

        Barn {
            pool,
            event_recv,
            message_send,
        }
    }

    pub async fn start(&mut self) {
        while let Some(e) = self.event_recv.recv().await {
            match e {
                Event::RequestSubfolders { parent } => {
                    println!("REQUEST {parent:#?}")
                }
            }
        }
    }
}
