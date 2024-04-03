use crate::types::ListSubcommand;

mod chains;

pub(crate) async fn execute_subcmd(subcmd: ListSubcommand) -> Result<(), anyhow::Error> {
    match subcmd {
        ListSubcommand::Chains => chains::execute().await,
    }
}
