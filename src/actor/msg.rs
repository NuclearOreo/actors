use tokio::sync::oneshot;

pub enum Msg {
    GetUniqueId {
        respond_to: oneshot::Sender<i32>,
    }
}