mod actor;

use actor::ActorHandle;

#[tokio::main]
async fn main() {
    let handle = ActorHandle::new();
    let val = handle.get_unique_id().await;
    let trimmed_text = handle.trim_text("Hello     ".to_string()).await;
    println!("{:?}", val);
    println!("{:?}", trimmed_text);
}
