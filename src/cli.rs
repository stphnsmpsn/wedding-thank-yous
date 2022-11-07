use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(long)]
    pub input_file: std::path::PathBuf,
    #[arg(long)]
    pub smtp_username: String,
    #[arg(long)]
    pub smtp_password: String,
    #[arg(long)]
    pub smtp_server: String,
    #[arg(long)]
    pub from: String,
    #[arg(long)]
    pub image_path: Option<String>,
    #[arg(long)]
    pub attachment: Option<Vec<String>>,
}
