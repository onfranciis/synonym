use std::error::Error;
use tokio::{
    join,
    time::{sleep, Duration},
};
use tracing::{error, info};

async fn async_task(id: usize) -> Result<(), Box<dyn Error + Send + Sync>> {
    let sleep_time = Duration::from_secs(2);
    sleep(sleep_time).await;

    if id % 3 == 0 {
        error!("Task number {} failed!", id);
        return Err(format!("Critical error in task {}", id).into());
    }

    info!("Task number {} has completed successfully!", id);
    Ok(())
}

#[tokio::main]
async fn main() {
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
