use dotenvy::dotenv;
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    let api_key = env::var("RESEND_API_KEY")
        .expect("RESEND_API_KEY must be set in environment");
    let resend = Resend::new(&api_key);

    let from = "onboarding@resend.dev";
    let to = ["your@email.com"]; // TODO: change to your onboarded email
    let subject = "Hello from Resend";

    let email = CreateEmailBaseOptions::new(from, to, subject)
        .with_html("<p>Congrats on sending your <strong>first email</strong>!</p>");

    let _email = resend.emails.send(email).await?;
    println!("{:?}", _email);

    Ok(())
}
