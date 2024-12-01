pub mod cli;
pub mod days;

fn main() -> anyhow::Result<()> {
    cli::Cli::run()
}
