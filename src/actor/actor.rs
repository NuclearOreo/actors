use super::msg::Msg;
use tokio::sync::mpsc;

pub struct Actor {
    receiver: mpsc::Receiver<Msg>,
    next_id: i32
}

impl Actor {
    pub fn new(receiver: mpsc::Receiver<Msg>) -> Self {
        Self {
            receiver,
            next_id: 0,
        }
    }
    pub async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_msg(msg);
        }
    }
    fn handle_msg(&mut self, msg: Msg ) {
        match msg {
            Msg::GetUniqueId { respond_to } => {
                self.next_id += 1;
                let _ = respond_to.send(self.next_id);
            },
        }
    }
}
