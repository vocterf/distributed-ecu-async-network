use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("=== ASYNCHRONOUS CHANNELS SIMULATION START ===");

    let (tx, mut rx) = mpsc::channel::<u32>(5);

    let tx_sensor_a = tx.clone();
    let tx_sensor_b = tx; 

    tokio::spawn(async move {
        let simulated_speeds = [30, 32, 35];
        for speed in simulated_speeds {
            println!("[Sensor A: Wheel Speed] Emitting: {} km/h", speed);
            
            if tx_sensor_a.send(speed).await.is_err() {
                eprintln!("[Sensor A] Channel closed by receiver. Terminating task.");
                break;
            }
            sleep(Duration::from_millis(40)).await;
        }
    });

    tokio::spawn(async move {
        let simulated_pressures = [150, 200];
        for pressure in simulated_pressures {
            println!("  [Sensor B: Brake Fluid] Emitting: {} kPa", pressure);
            
            if tx_sensor_b.send(pressure).await.is_err() {
                eprintln!("  [Sensor B] Channel closed by receiver. Terminating task.");
                break;
            }
            sleep(Duration::from_millis(60)).await;
        }
    });

    println!("[ECU Core] Initializing consumer event loop...");

    while let Some(bus_packet) = rx.recv().await {
        println!("[ECU Core] Bus Intercept -> Processing data packet: {}", bus_packet);
    }

    println!("=== ASYNCHRONOUS CHANNELS SIMULATION END ===");
}