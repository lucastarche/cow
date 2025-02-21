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
                    let subfolders = self.get_subfolders_of(parent.clone()).await;
                    self.send_message(Message::UpdateSubfolders { parent, subfolders });
                }
            }
        }
    }

    fn send_message(&mut self, message: Message) {
        self.message_send
            .send(message)
            .expect("Message should be sent successfully");
    }

    async fn get_subfolders_of(&mut self, parent: Option<i64>) -> Vec<i64> {
        let res = sqlx::query!("SELECT id FROM folders WHERE parent IS ?", parent)
            .fetch_all(&self.pool)
            .await;

        if let Ok(v) = res {
            v.iter().map(|rec| rec.id).collect()
        } else {
            vec![]
        }
    }
}
