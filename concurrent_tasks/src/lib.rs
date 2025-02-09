use std::{error::Error, time::Duration};
use tokio::time::sleep;
use tracing::{error, info};

pub async fn async_task(id: usize) -> Result<(), Box<dyn Error + Send + Sync>> {
    let sleep_time = Duration::from_secs(2);
    sleep(sleep_time).await;

    if id % 3 == 0 {
        error!("Task number {} failed!", id);
        return Err(format!("Critical error in task {}", id).into());
    }

    info!("Task number {} has completed successfully!", id);
    Ok(())
}
