use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[derive(Debug)]
struct CanFrame {
    id: u32,
    payload: u32
}

#[tokio::main]
async fn main() {
    let (tx_gateway, mut rx_dashboard) = mpsc::channel::<CanFrame>(32);

    let tx_oil_sensor = tx_gateway.clone();
    let tx_rpm_sensor = tx_gateway;

    tokio::spawn(async move {
        loop {
            let frame = CanFrame {id: 0x120, payload: 4};

            if tx_oil_sensor.send(frame).await.is_err() {
                println!("[ERR]: Oil sensor channel closed!");
                break;
            }
            sleep(Duration::from_millis(800)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            let frame = CanFrame {id: 0x2A0, payload: 2200};

            if tx_rpm_sensor.send(frame).await.is_err() {
                println!("[ERR]: RPM sensor channel closed!");
                break;
            }
            sleep(Duration::from_millis(300)).await;
        }
    });

    println!("[DASHBOARD]: Booting interface... Listening for frames...");

    while let Some(frame) = rx_dashboard.recv().await {
        match frame.id {
            0x120 => println!("[DISPLAY] Oil Pressure: {} bar", frame.payload),
            0x2A0 => println!("[DISPLAY] Engine Speed: {} RPM", frame.payload),
            _ => println!("[DISPLAY] Unknown frame ID: {:#X}", frame.id),
        }
    }
}