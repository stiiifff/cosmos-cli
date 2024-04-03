use crate::types::InfoSubcommand;

pub(crate) async fn execute_subcmd(subcmd: InfoSubcommand) -> Result<(), anyhow::Error> {
    match subcmd {
        InfoSubcommand::Chain { chain_id } => {
            println!("Getting information about chain {} ...", chain_id);
        }
    }
    Ok(())
}
