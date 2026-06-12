use crate::Database;
use crate::email::QuoteEmail;
use crate::models::DateId;
use chrono::{Datelike, Timelike, Utc};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tracing::{error, info};

pub async fn init_email_scheduler(db: Database) -> Result<(), JobSchedulerError> {
    let scheduler = JobScheduler::new().await.unwrap();

    scheduler
        .add(Job::new_async("0 0,15,30,45 * * * *", move |_uuid, _l| {
            let db = db.clone();
            Box::pin(async move {
                let now = Utc::now();
                let send_time = format!("{:02}:{:02}", now.hour(), (now.minute() / 15) * 15);
                let today = format!("{:02}-{:02}", now.month(), now.day());
                let date_id = match DateId::new(&today) {
                    Ok(id) => id,
                    Err(e) => {
                        error!("Invalid date id: {}", e);
                        return;
                    }
                };
                let Some(quote) = db.quotes.get(&date_id).await else {
                    return;
                };
                let recipients = db.users.get_scheduled_users(&send_time).await;
                info!("Sending emails to {} recipients", recipients.len());
                if let Err(e) = QuoteEmail::send_batch(recipients, &quote).await {
                    error!("Failed to send scheduled emails: {}", e);
                }
            })
        })?)
        .await?;

    scheduler.start().await?;

    Ok(())
}
