mod actor;

use actor::actor::ActorHandle;

#[tokio::main]
async fn main() {
    let handle = ActorHandle::new();
    let val = handle.get_unique_id().await;
    println!("{:?}", val);
}
