use concurrent_tasks::async_task;

#[tokio::test]
async fn test_single_task_success() {
    let result = async_task(2).await; // Task with ID 2 (should succeed)

    assert!(result.is_ok(), "Task should succeed but failed.");
}
