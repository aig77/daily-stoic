use crate::AppState;
use crate::email::QuoteEmail;
use crate::models::DateId;
use chrono::{Datelike, Timelike, Utc};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tracing::{error, info};

pub async fn init_email_scheduler(state: AppState) -> Result<(), JobSchedulerError> {
    let scheduler = JobScheduler::new().await?;

    scheduler
        .add(Job::new_async("0 0,15,30,45 * * * *", move |_uuid, _l| {
            let state = state.clone();
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

                let quote = match state.db.quotes.get(&date_id).await {
                    Ok(Some(q)) => q,
                    Ok(None) => {
                        error!("no quote found for {}", &date_id);
                        return;
                    }
                    Err(e) => {
                        error!("failed to get quote on {}: {}", &date_id, e);
                        return;
                    }
                };

                let recipients = match state.db.users.get_scheduled_users(&send_time).await {
                    Ok(r) => r,
                    Err(e) => {
                        error!("failed to get scheduled users: {}", e);
                        return;
                    }
                };

                let recipients_count = recipients.len();

                if recipients_count == 0 {
                    info!("No recipients scheduled at this time.");
                    return;
                }

                info!("Sending emails to {} recipients", recipients_count);

                if let Err(e) =
                    QuoteEmail::send_batch(recipients, &quote, &state.config.base_url).await
                {
                    error!("Failed to send scheduled emails: {}", e);
                }
            })
        })?)
        .await?;

    scheduler.start().await?;

    Ok(())
}
