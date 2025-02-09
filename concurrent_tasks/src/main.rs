use concurrent_tasks::async_task;
use std::error::Error;
use tokio::{
    join,
    time::{sleep, Duration},
};
use tracing::{error, info};

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    let (res1, res2, res3, res4, res5) = join!(
        async_task(1),
        async_task(2),
        async_task(3),
        async_task(4),
        async_task(5)
    );

    let results = vec![res1, res2, res3, res4, res5];
    for res in results {
        if let Err(e) = res {
            error!("A task failed: {}", e);
            error!("Cancelling all tasks...");
            // We can close DB connections and IO processes here
            return;
        }
    }

    info!("All tasks completed successfully.");
}
