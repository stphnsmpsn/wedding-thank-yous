use clap::Parser;
use std::io::Write;

use crate::mailer::Mailer;
use crate::{
    cli::Args,
    context::Context,
    error::Error,
    mailer::template::make_message_body,
    models::{GiftType, Party},
};

pub(crate) mod cli;
pub(crate) mod context;
pub(crate) mod error;
pub(crate) mod mailer;
pub(crate) mod models;

fn main() -> Result<(), Error> {
    let args: Args = Args::parse();

    let context = Context::new(args.smtp_username, args.smtp_password, args.smtp_server)?;

    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(args.input_file)?;

    let mailer = Mailer::new(context, args.image_path, args.attachment, args.from)?;

    let mut errors: Vec<String> = vec![];

    for result in reader.deserialize::<Party>() {
        match result {
            Ok(party) => {
                print!("Thanking: {}... ", party.name);
                std::io::stdout().flush().unwrap();
                match mailer.send_email(&party) {
                    Ok(()) => println!("[DONE]"),
                    Err(_) => {
                        println!("[ERROR]");
                        errors.push(format!("Failed to thank: {}", party.name))
                    }
                };
            }
            Err(e) => {
                println!("Failed to deserialize party: {:?}", e);
            }
        }
    }

    errors.iter().for_each(|error| println!("{error}"));

    Ok(())
}
