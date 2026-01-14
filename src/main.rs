use {
    agave_validator::admin_rpc_service,
    std::path::Path
};
#[tokio::main]
async fn main() {
    let ledger_path=Path::new("/mnt/ledger");
    let admin_client= admin_rpc_service::connect(ledger_path).await.unwrap();
    println!("connected to ledger");
}
