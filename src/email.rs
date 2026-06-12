use crate::models::{LoginCode, Quote};

use askama::Template;
use resend_rs::{
    types::CreateEmailBaseOptions,
    {Resend, Result},
};
use tracing::info;

const EMOJIS: [&str; 10] = ["📚", "📖", "✨", "💭", "🧠", "🎯", "🔥", "🌟", "🏛️", "🌊"];

#[derive(Template)]
#[template(path = "email/quote.html")]
struct QuoteEmailTemplate<'a> {
    quote: &'a Quote,
}

pub struct QuoteEmail;

impl QuoteEmail {
    // to: Vec<String> because same email can have multiple recipients
    pub async fn send(to: Vec<String>, quote: &Quote) -> Result<()> {
        let from = get_from();
        let resend = Resend::new(&std::env::var("RESEND_API_KEY").unwrap());

        let subject = format!("{} {}", get_random_emoji(), &quote.title);
        let html = QuoteEmailTemplate { quote }.render().unwrap();

        let email = CreateEmailBaseOptions::new(from, to, subject).with_html(&html);

        resend.emails.send(email).await?;

        Ok(())
    }

    pub async fn send_batch(recipients: Vec<String>, quote: &Quote) -> Result<()> {
        let from = get_from();
        let resend = Resend::new(&std::env::var("RESEND_API_KEY").unwrap());

        let subject = format!("{} {}", get_random_emoji(), &quote.title);
        let html = QuoteEmailTemplate { quote }.render().unwrap();

        let emails: Vec<_> = recipients
            .into_iter()
            .map(|recipient| {
                let to = vec![recipient];
                CreateEmailBaseOptions::new(&from, to, &subject).with_html(&html)
            })
            .collect();

        resend.batch.send(emails).await?;

        Ok(())
    }
}

#[derive(Template)]
#[template(path = "email/login_code.html")]
struct LoginCodeEmailTemplate<'a> {
    code: &'a str,
}

pub struct LoginCodeEmail;

impl LoginCodeEmail {
    pub async fn send(login_code: &LoginCode) -> Result<()> {
        let from = get_from();
        let resend = Resend::new(&std::env::var("RESEND_API_KEY").unwrap());

        let html = LoginCodeEmailTemplate {
            code: &login_code.code,
        }
        .render()
        .unwrap();
        let email = CreateEmailBaseOptions::new(
            &from,
            vec![&login_code.email],
            "Your Daily Stoic login code",
        )
        .with_html(&html);

        resend.emails.send(email).await?;

        Ok(())
    }
}

fn get_from() -> String {
    let s = std::env::var("RESEND_EMAIL")
        .expect("Required env var RESEND_EMAIL")
        .to_string();
    info!("from {}", s);
    s
}

fn get_random_emoji() -> &'static str {
    let rand = rand::random_range(0..EMOJIS.len());
    EMOJIS[rand]
}

pub fn check_env_vars() {
    let key = std::env::var("RESEND_API_KEY").expect("Missing required env var RESEND_API_KEY");
    std::env::var("RESEND_EMAIL").expect("Missing required env var RESEND_EMAIL");
    info!("key={}", key);
}
