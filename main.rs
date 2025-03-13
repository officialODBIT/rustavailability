use tokio::task;
use std::time::Duration;

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

async fn fetch_user_data(user_id: u32) -> Result<User, String> {
    // Simulate fetching data (this could be a database query, for example)
    if user_id == 0 {
        return Err("Invalid user ID".to_string());
    }
    
    tokio::time::sleep(Duration::from_secs(2)).await; // Simulate delay
    Ok(User {
        id: user_id,
        name: format!("User{}", user_id),
    })
}

async fn process_data(user_id: u32) -> Result<String, String> {
    let user_data = fetch_user_data(user_id).await?;
    Ok(format!("Processed data for: {}", user_data.name))
}

#[tokio::main]
async fn main() {
    let user_id = 42;
    
    let handle = task::spawn(async move {
        match process_data(user_id).await {
            Ok(result) => println!("{}", result),
            Err(e) => println!("Error: {}", e),
        }
    });
    
    // Simulate some other work in main while the task runs
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Main thread doing other work...");

    // Wait for the spawned task to complete
    handle.await.unwrap();
}
