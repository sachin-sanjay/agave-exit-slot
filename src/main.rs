use {
    agave_validator::admin_rpc_service,
    std::path::Path,
    solana_rpc_client::rpc_client::RpcClient
};
#[tokio::main]
async fn main() {
    let ledger_path=Path::new("/mnt/ledger");
    let admin_client= admin_rpc_service::connect(ledger_path).await.unwrap();
    println!("connected to ledger");
    let pid=admin_client.pid().await.unwrap();
    let rpc_endpoint=admin_client.rpc_addr().await.unwrap();
    let rpc_client = RpcClient::new_socket(rpc_addr);
    let mut identity = match rpc_client.get_identity() {
        Ok(identity) => identity,
        Err(err) => {
            println!("Failed to get validator identity over RPC: {err}");
            continue;
        }
    };
    println!("Identity:{}", identity.to_string());
    let current_slot=rpc_client.get_slot().unwrap();
    println!("Current Slot:{}", current_slot);

}
