use std::collections::HashMap;

use tokio::sync::mpsc::{self, error::TryRecvError, UnboundedReceiver, UnboundedSender};

use crate::{barn::Barn, schema::folder::Folder};

pub struct FarmerJohn {
    folders: HashMap<i64, Folder>,
    subfolders_of: HashMap<Option<i64>, Vec<i64>>,

    event_send: UnboundedSender<Event>,
    message_recv: UnboundedReceiver<Message>,
}

impl FarmerJohn {
    pub fn new() -> FarmerJohn {
        let (event_send, event_recv) = mpsc::unbounded_channel::<Event>();
        let (message_send, message_recv) = mpsc::unbounded_channel::<Message>();

        tokio::spawn(async move {
            let mut barn = Barn::new(event_recv, message_send).await;
            barn.start().await.expect("Barn should not crash");
        });

        FarmerJohn {
            event_send,
            message_recv,
            folders: Default::default(),
            subfolders_of: Default::default(),
        }
    }

    pub fn process_messages(&mut self) {
        loop {
            let res = self.message_recv.try_recv();

            if let Ok(message) = res {
                match message {
                    Message::UpdateSubfolders { parent, subfolders } => {
                        self.subfolders_of.insert(parent, subfolders);
                    }
                    Message::UpdateFolder(folder) => {
                        self.folders.insert(folder.id, folder);
                    }
                }
            } else if let Err(TryRecvError::Empty) = res {
                break;
            } else {
                panic!("Channel was closed unexpectedly!");
            }
        }
    }

    pub fn get_subfolders_of(&mut self, parent: Option<i64>) -> &Vec<i64> {
        if !self.subfolders_of.contains_key(&parent) {
            self.subfolders_of.insert(parent, vec![]);
            self.send_event(Event::RequestSubfolders { parent });
        }

        self.subfolders_of.get(&parent).unwrap()
    }

    pub fn get_folder(&mut self, id: i64) -> &Folder {
        if !self.folders.contains_key(&id) {
            self.folders.insert(id, Default::default());
            self.send_event(Event::RequestFolder { id });
        }

        self.folders.get(&id).unwrap()
    }

    fn send_event(&mut self, event: Event) {
        self.event_send
            .send(event)
            .expect("Event should be sent successfully");
    }
}

pub enum Event {
    RequestSubfolders { parent: Option<i64> },
    RequestFolder { id: i64 },
}

pub enum Message {
    UpdateSubfolders {
        parent: Option<i64>,
        subfolders: Vec<i64>,
    },
    UpdateFolder(Folder),
}
