use std::time::Duration;
use tokio::time::{sleep, timeout};


async fn request_ecu_status(processing_time_ms: u64) -> String {
    println!("[ECU]: Request received.");

    sleep(Duration::from_millis(processing_time_ms)).await;

    String::from("ECU_STATUS: ACTIVE, NO_ERRORS")
}



#[tokio::main]
async fn main() {

    let ecu_delay = 1500;

    println!("Sending request... It has 1000ms to respond.");

    let diagnostic_result = timeout(Duration::from_millis(1000), request_ecu_status(ecu_delay)).await ;

    match diagnostic_result {
        Ok(ecu_message) => {
            println!("\n[SUCCESS]: Response in time.");
            println!("[DASHBOARD]: Data: -> {}", ecu_message);
        }
        Err(_) => {
            println!("\n[TIMEOUT CRITICAL]: Timeout.");
        }
    }
}