use tokio::sync::{oneshot, mpsc};

struct Actor {
    receiver: mpsc::Receiver<ActorMsg>,
    next_id: i32
}

enum ActorMsg {
    GetUniqueId {
        respond_to: oneshot::Sender<i32>,
    }
}

impl Actor {
    fn new(receiver: mpsc::Receiver<ActorMsg>) -> Self {
        Self {
            receiver,
            next_id: 0,
        }
    }
    fn handle_msg(&mut self, msg: ActorMsg ) {
        match msg {
            ActorMsg::GetUniqueId { respond_to } => {
                self.next_id += 1;
                let _ = respond_to.send(self.next_id);
            },
        }
    }
}

async fn run_my_actor(mut actor: Actor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_msg(msg);
    }
}

#[derive(Clone)]
pub struct ActorHandle {
    sender: mpsc::Sender<ActorMsg>,
}

impl ActorHandle {
    pub fn new() -> Self {
        let (sender,  reciever) = mpsc::channel(8);
        let actor = Actor::new(reciever);
        tokio::spawn(run_my_actor(actor));
        Self { sender }
    }
    pub async  fn get_unique_id(&self) -> i32 {
        let (send, recv) = oneshot::channel();
        let msg = ActorMsg::GetUniqueId { respond_to: send };
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
}