mod actor;

use actor::ActorHandle;

#[tokio::main]
async fn main() {
    let handle = ActorHandle::new();
    { 
        let another_handle = handle.clone();
        let id = another_handle.get_unique_id().await;
        println!("{id}");
    }
    let id = handle.get_unique_id().await;
    let trimmed_text = handle.trim_text("Hello     ".to_string()).await;
    println!("{id}, {trimmed_text}");
}
