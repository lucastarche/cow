use std::env;

use sqlx::SqlitePool;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    farmer_john::{Event, Message},
    schema::folder::Folder,
};

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

    pub async fn start(&mut self) -> anyhow::Result<()> {
        while let Some(e) = self.event_recv.recv().await {
            match e {
                Event::RequestSubfolders { parent } => {
                    let subfolders = self.get_subfolders_of(parent.clone()).await?;
                    self.send_message(Message::UpdateSubfolders { parent, subfolders });
                }
                Event::RequestFolder { id } => {
                    let folder = self.get_folder(id).await?;
                    if let Some(folder) = folder {
                        self.send_message(Message::UpdateFolder(folder));
                    }
                }
            }
        }

        Ok(())
    }

    fn send_message(&mut self, message: Message) {
        self.message_send
            .send(message)
            .expect("Message should be sent successfully");
    }

    async fn get_subfolders_of(&mut self, parent: Option<i64>) -> anyhow::Result<Vec<i64>> {
        let res = sqlx::query!("SELECT id FROM folders WHERE parent IS ?", parent)
            .fetch_all(&self.pool)
            .await?;

        Ok(res.iter().map(|rec| rec.id).collect())
    }

    async fn get_folder(&mut self, id: i64) -> anyhow::Result<Option<Folder>> {
        let res = sqlx::query_as!(Folder, r#"SELECT * FROM folders WHERE id = ?"#, id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(res)
    }
}
