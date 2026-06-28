mod actor;

use actor::ActorHandle;

#[tokio::main]
async fn main() {
    let handle = ActorHandle::new();
    let mut val = handle.get_unique_id().await;
    println!("{:?}", val);
}
