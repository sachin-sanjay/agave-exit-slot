use std::{io, thread, time::Duration};
use {
    agave_validator::admin_rpc_service,
    clap::{Arg, ArgMatches, command},
    solana_rpc_client::rpc_client::RpcClient,
    std::path::Path,
};
#[tokio::main]
async fn main() {
    let results: ArgMatches = command!()
        .arg(
            Arg::new("target-slot")
                .short('t')
                .long("target-slot")
                .required(true),
        )
        .arg(
            Arg::new("ledger-path")
                .short('l')
                .long("ledger-path")
                .required(true),
        )
        .get_matches();
    let p = results.get_one::<String>("ledger-path").unwrap();
    println!("value {:?}", p);
    let ledger_path = Path::new(p);
    let admin_client = admin_rpc_service::connect(ledger_path).await.unwrap();
    println!("connected to ledger");
    let pid = admin_client.pid().await.unwrap();
    let rpc_endpoint = admin_client.rpc_addr().await.unwrap();
    let rpc_client = RpcClient::new_socket(rpc_endpoint.unwrap());
    let identity = rpc_client.get_identity().unwrap();
    println!("Identity:{}", identity.to_string());
    let mut current_slot = rpc_client.get_slot().unwrap();
    println!("Current Slot:{}", current_slot);
    let target_slot_str = results.get_one::<String>("target-slot").unwrap();
    let target_slot: u64 = target_slot_str.parse().unwrap();
    if target_slot < current_slot {
        panic!("Target slot must be greater than current slot");
    }
    let mut diff = target_slot - current_slot;
    while target_slot > current_slot {
        //add a sleep to avoid spamming the rpc server
        println!("{} slots left", diff);
        diff = target_slot - current_slot;
        tokio::time::sleep(Duration::from_millis(400)).await;
        current_slot = rpc_client.get_slot().unwrap();
    }
    // check for the snapshot change
    let mut highest_snapshot= rpc_client.get_highest_snapshot_slot().unwrap();
    let full_snapshot_slot=highest_snapshot.full;
    let incremental_snapshot_slot=highest_snapshot.incremental.unwrap();
    highest_snapshot= rpc_client.get_highest_snapshot_slot().unwrap();
    let mut new_full = highest_snapshot.full;
    let mut new_incremental = highest_snapshot.incremental.unwrap();
    let mut flag=false;
    println!("Waiting for new snapshot after reaching target slot {}", target_slot);
    while new_full == full_snapshot_slot && new_incremental == incremental_snapshot_slot {
        highest_snapshot= rpc_client.get_highest_snapshot_slot().unwrap();
        new_full = highest_snapshot.full;
        new_incremental = highest_snapshot.incremental.unwrap(); 
        println!{"old snapshot {} vs new snapshot {}", incremental_snapshot_slot, new_incremental};   
        tokio::time::sleep(Duration::from_millis(2000)).await;
    }

    // exit
    admin_client.exit().await.unwrap();
    poll_until_pid_terminates(pid);
}
fn poll_until_pid_terminates(pid: u32) {
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
            let errno = io::Error::last_os_error().raw_os_error().unwrap();
            match errno {
                libc::ESRCH => {
                    println!("Done, agave-validator process {pid} has terminated");
                    break;
                }
                _ => {
                    println!("waiting");
                }
            }
        }
    }
}
