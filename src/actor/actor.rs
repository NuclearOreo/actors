use crate::actor::msg::Msg;
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
    fn handle_msg(&mut self, msg: Msg ) {
        match msg {
            Msg::GetUniqueId { respond_to } => {
                self.next_id += 1;
                let _ = respond_to.send(self.next_id);
            },
        }
    }
}

pub async fn run_my_actor(mut actor: Actor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_msg(msg);
    }
}