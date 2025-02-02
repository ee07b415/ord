use {
  super::*,
  ord::subcommand::wallet::{cardinals::Cardinal, outputs::Output},
};

#[test]
fn cardinals() {
  let rpc_server = test_bitcoincore_rpc::spawn();
  create_wallet(&rpc_server);

  // this creates 2 more cardinal outputs and one inscribed output
  inscribe(&rpc_server);

  let all_outputs = CommandBuilder::new("wallet outputs")
    .rpc_server(&rpc_server)
    .output::<Vec<Output>>();

  let cardinal_outputs = CommandBuilder::new("wallet cardinals")
    .rpc_server(&rpc_server)
    .output::<Vec<Cardinal>>();

  assert_eq!(all_outputs.len() - cardinal_outputs.len(), 1);
}
