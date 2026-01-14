use {
    agave_validator::admin_rpc_service,
    std::path::Path
};
fn main() {
    let ledger_path=Path::new("/mnt/ledger");
    let admin_client= match admin_rpc_service::connect(ledger_path)?;
    println!("connected to ledger");
}
