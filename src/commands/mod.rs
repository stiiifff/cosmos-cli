use crate::types::Commands;

mod info;
mod list;

pub(crate) async fn execute_cmd(cmd: Commands) -> Result<(), anyhow::Error> {
    match cmd {
        Commands::Info { subcmd } => info::execute_subcmd(subcmd).await,
        Commands::List { subcmd } => list::execute_subcmd(subcmd).await,
    }
}
