use std::{io, thread, time::Duration};
use {
    agave_validator::admin_rpc_service,
    std::path::Path,
    solana_rpc_client::rpc_client::RpcClient,
    std::error::Error
};
#[tokio::main]
async fn main() {
    let ledger_path=Path::new("/mnt/ledger");
    let admin_client= admin_rpc_service::connect(ledger_path).await.unwrap();
    println!("connected to ledger");
    let pid=admin_client.pid().await.unwrap();
    let rpc_endpoint=admin_client.rpc_addr().await.unwrap();
    let rpc_client = RpcClient::new_socket(rpc_endpoint.unwrap());
    let mut identity =rpc_client.get_identity().unwrap();
    println!("Identity:{}", identity.to_string());
    let mut current_slot=rpc_client.get_slot().unwrap();
    println!("Current Slot:{}", current_slot);
    let target_slot=current_slot+1000;
    if target_slot < current_slot {
        panic!("Target slot must be greater than current slot");
    }
    while target_slot > current_slot {
        println!("{} slots left", target_slot-current_slot);
        current_slot=rpc_client.get_slot().unwrap();
    }
    // exit
    admin_client.exit().await.unwrap();
    poll_until_pid_terminates(pid);
}
fn poll_until_pid_terminates(pid: u32){
    let pid = i32::try_from(pid).unwrap();

    println!("Waiting for agave-validator process {pid} to terminate");
    loop {
        // From man kill(2)
        //
        // If sig is 0, then no signal is sent, but existence and permission
        // checks are still performed; this can be used to check for the
        // existence of a process ID or process group ID that the caller is
        // permitted to signal.
        let result = unsafe {
            libc::kill(pid, /*sig:*/ 0)
        };
        if result >= 0 {
            // Give the process some time to exit before checking again
            thread::sleep(Duration::from_millis(500));
        } else {
        println!("couldn't exit");
        }
    }
}