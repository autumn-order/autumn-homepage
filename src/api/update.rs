use tokio::signal;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use super::task::stats::task_update_corporation_stats;

pub async fn schedule_tasks() -> Result<(), JobSchedulerError> {
    let sched = JobScheduler::new().await?;

    sched
        .add(Job::new_async("0 37 * * * *", |_uuid, _l| {
            Box::pin(async move {
                task_update_corporation_stats().await;
            })
        })?)
        .await?;

    sched.start().await?;

    signal::ctrl_c()
        .await
        .expect("failed to listen for termination event");

    Ok(())
}
