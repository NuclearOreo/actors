use super::msg::Msg;
use super::actor::Actor;
use tokio::sync::{mpsc, oneshot};


#[derive(Clone)]
pub struct ActorHandle {
    sender: mpsc::Sender<Msg>,
}

impl ActorHandle {
    pub fn new() -> Self {
        let (sender,  reciever) = mpsc::channel(8);
        let mut actor = Actor::new(reciever);
        tokio::spawn( async move { actor.run().await });
        Self { sender }
    }
    pub async  fn get_unique_id(&self) -> i32 {
        let (send, recv) = oneshot::channel();
        let msg = Msg::GetUniqueId { respond_to: send };
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
    pub async  fn trim_text(&self, text: String) -> String {
        let (send, recv) = oneshot::channel();
        let msg = Msg::TrimText { text: text, respont_to: send };
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
}