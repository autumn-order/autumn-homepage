use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tokio_cron_scheduler::{Job, JobScheduler};

use super::task::stats::task_update_corporation_stats;

pub async fn schedule_tasks(sched: &JobScheduler, db: Arc<DatabaseConnection>) {
    let db_clone = Arc::clone(&db);
    task_update_corporation_stats(&db_clone).await;

    sched
        .add(
            Job::new_async("0 37 * * * *", move |_uuid, _l| {
                let db_clone = Arc::clone(&db_clone);
                Box::pin(async move {
                    task_update_corporation_stats(&db_clone).await;
                })
            })
            .expect("Failed to create job"),
        )
        .await
        .expect("Failed to add job to scheduler");
}
