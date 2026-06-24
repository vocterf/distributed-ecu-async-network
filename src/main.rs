use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

struct TyrePayload {
    id: String,
    pressure: f32,
}

#[tokio::main]
async fn main() {
    let (tx_gateway, mut rx_dashboard) = mpsc::channel::<TyrePayload>(4);

    let tx_fl_sensor = tx_gateway.clone();
    let tx_fr_sensor = tx_gateway.clone();
    let tx_rl_sensor = tx_gateway.clone();
    let tx_rr_sensor = tx_gateway.clone();

    tokio::spawn(async move {
        loop {
            let frame = TyrePayload {
                id: String::from("Front Left"),
                pressure: 2.2,
            };
            if tx_fl_sensor.send(frame).await.is_err() {
                println!("fl channel is closed");
                break;
            }
            sleep(Duration::from_millis(500)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            let frame = TyrePayload {
                id: String::from("Front Right"),
                pressure: 2.2,
            };
            if tx_fr_sensor.send(frame).await.is_err() {
                println!("fr channel is closed");
                break;
            }
            sleep(Duration::from_millis(500)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            let frame = TyrePayload {
                id: String::from("Rear Left"),
                pressure: 2.2,
            };
            if tx_rl_sensor.send(frame).await.is_err() {
                println!("rl channel is closed");
                break;
            }
            sleep(Duration::from_millis(500)).await;
        }
    });

    tokio::spawn(async move {
        let mut pressure = 2.2;
        loop {
            let frame = TyrePayload {
                id: String::from("Rear Right"),
                pressure,
            };
            if tx_rr_sensor.send(frame).await.is_err() {
                println!("rr channel is closed");
                break;
            }
            pressure -= 0.1;
            if pressure <= 0.1 {
                pressure = 2.2
            }
            sleep(Duration::from_millis(500)).await;
        }
    });

    drop(tx_gateway);

    while let Some(frame) = rx_dashboard.recv().await {
        println!("{}: {:.2}", frame.id, frame.pressure);
    }
}
