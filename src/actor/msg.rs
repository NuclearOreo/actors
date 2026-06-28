use tokio::sync::oneshot;

pub enum Msg {
    GetUniqueId {
        respond_to: oneshot::Sender<i32>,
    },
    TrimText {
        text: String,
        respont_to: oneshot::Sender<String>
    }
}