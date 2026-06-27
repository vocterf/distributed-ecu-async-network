use socketcan::tokio::CanSocket;
use socketcan::{CanFrame, EmbeddedFrame, StandardId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = CanSocket::open("vcan0")?;
    
    // Construct a standard CAN frame with ID 0x120 and payload [1, 2, 3, 4]
    let id = StandardId::new(0x120).unwrap();
    let frame = CanFrame::new(id, &[1, 2, 3, 4]).unwrap();
    
    // Test write
    socket.write_frame(frame).await?;
    
    // Test read
    let recv_frame = socket.read_frame().await?;
    println!("Recv: {:?}", recv_frame);
    
    Ok(())
}
