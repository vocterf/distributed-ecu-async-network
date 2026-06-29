use socketcan::tokio::CanSocket;
use socketcan::{CanFrame, EmbeddedFrame, StandardId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = CanSocket::open("vcan0")?;
    
    let id = StandardId::new(0x120).unwrap();
    let frame = CanFrame::new(id, &[1, 2, 3, 4]).unwrap();
    
    socket.write_frame(frame).await?;
    
    let recv_frame = socket.read_frame().await?;
    println!("Recv: {:?}", recv_frame);
    
    Ok(())
}
