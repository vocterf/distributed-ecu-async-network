use embedded_can::{Frame, Id, StandardId, ExtendedId};
use bitvec::prelude::*;
use core::convert::TryFrom;
#[derive(Debug, Clone)]
pub enum CanError {
    UnknownFrameId,
    UnknownMuxValue,
    InvalidPayloadSize,
    InvalidFrameId,
    ValueOutOfRange,
    InvalidEnumValue,
}
pub trait GeneratedCanMessage<const LEN: usize>: Sized {
    fn try_from_frame(frame: &impl Frame) -> Result<Self, CanError>;
    fn encode(&self) -> [u8; LEN];
}
#[derive(Debug, Clone)]
pub enum Msg {
    RpmData(RpmDataMsg),
    OilData(OilDataMsg),
}
impl Msg {
    fn try_from(frame: &impl Frame) -> Result<Self, CanError> {
        let result = match frame.id() {
            RpmDataMsg::ID => Msg::RpmData(RpmDataMsg::try_from_frame(frame)?),
            OilDataMsg::ID => Msg::OilData(OilDataMsg::try_from_frame(frame)?),
            _ => return Err(CanError::UnknownFrameId),
        };
        Ok(result)
    }
}
///RPM_DATA
///- ID: Standard 256 (0x100)
///- Size: 2 bytes
///- Transmitter: RPM_SENSOR
#[derive(Debug, Clone)]
pub struct RpmDataMsg {
    data: [u8; 2usize],
}
impl RpmDataMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(256u16) });
    pub const LEN: usize = 2usize;
    pub fn new(engine_rpm: u16) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_engine_rpm(engine_rpm)?;
        Ok(msg)
    }
    ///EngineRPM
    ///- Min: 0
    ///- Max: 8000
    ///- Unit: RPM
    ///- Receivers: DASHBOARD
    ///- Start bit: 0
    ///- Size: 16 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn engine_rpm(&self) -> u16 {
        let raw_engine_rpm = self
            .data
            .view_bits::<Lsb0>()[0usize..16usize]
            .load_le::<u16>();
        (raw_engine_rpm as u16).saturating_mul(1u16).saturating_add(0u16)
    }
    ///Set value of EngineRPM
    ///- Min: 0
    ///- Max: 8000
    pub fn set_engine_rpm(&mut self, value: u16) -> Result<(), CanError> {
        if value > 8000u16 {
            return Err(CanError::ValueOutOfRange);
        }
        let raw_value = value
            .checked_sub(0u16)
            .and_then(|v| v.checked_div(1u16))
            .ok_or(CanError::ValueOutOfRange)? as u16;
        let storage_value = raw_value as u16;
        self.data.view_bits_mut::<Lsb0>()[0usize..16usize].store_le(storage_value);
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for RpmDataMsg {
    fn try_from_frame(frame: &impl Frame) -> Result<Self, CanError> {
        if frame.data().len() < Self::LEN {
            return Err(CanError::InvalidPayloadSize);
        }
        if frame.id() != Self::ID {
            return Err(CanError::InvalidFrameId);
        }
        let mut buf = [0u8; Self::LEN];
        buf.copy_from_slice(&frame.data()[..Self::LEN]);
        Ok(Self { data: buf })
    }
    fn encode(&self) -> [u8; Self::LEN] {
        self.data
    }
}
///OIL_DATA
///- ID: Standard 512 (0x200)
///- Size: 3 bytes
///- Transmitter: OIL_SENSOR
#[derive(Debug, Clone)]
pub struct OilDataMsg {
    data: [u8; 3usize],
}
impl OilDataMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(512u16) });
    pub const LEN: usize = 3usize;
    pub fn new(oil_temperature: i16, oil_pressure: f32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_oil_temperature(oil_temperature)?;
        msg.set_oil_pressure(oil_pressure)?;
        Ok(msg)
    }
    ///OilTemperature
    ///- Min: -40
    ///- Max: 215
    ///- Unit: C
    ///- Receivers: DASHBOARD
    ///- Start bit: 0
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -40
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn oil_temperature(&self) -> i16 {
        let raw_oil_temperature = self
            .data
            .view_bits::<Lsb0>()[0usize..8usize]
            .load_le::<u8>();
        (raw_oil_temperature as i16).saturating_mul(1i16).saturating_add(-40i16)
    }
    ///OilPressure
    ///- Min: 0
    ///- Max: 10
    ///- Unit: bar
    ///- Receivers: DASHBOARD
    ///- Start bit: 8
    ///- Size: 16 bits
    ///- Factor: 0.01
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn oil_pressure(&self) -> f32 {
        let raw_oil_pressure = self
            .data
            .view_bits::<Lsb0>()[8usize..24usize]
            .load_le::<u16>();
        (raw_oil_pressure as f32) * (0.01f32) + (0f32)
    }
    ///Set value of OilTemperature
    ///- Min: -40
    ///- Max: 215
    pub fn set_oil_temperature(&mut self, value: i16) -> Result<(), CanError> {
        if value < -40i16 || value > 215i16 {
            return Err(CanError::ValueOutOfRange);
        }
        let raw_value = value
            .checked_sub(-40i16)
            .and_then(|v| v.checked_div(1i16))
            .ok_or(CanError::ValueOutOfRange)? as u8;
        let storage_value = raw_value as u8;
        self.data.view_bits_mut::<Lsb0>()[0usize..8usize].store_le(storage_value);
        Ok(())
    }
    ///Set value of OilPressure
    ///- Min: 0
    ///- Max: 10
    pub fn set_oil_pressure(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0f32 || value > 10f32 {
            return Err(CanError::ValueOutOfRange);
        }
        let raw_value = ((value - (0f32)) / (0.01f32)) as u16;
        let storage_value = raw_value as u16;
        self.data.view_bits_mut::<Lsb0>()[8usize..24usize].store_le(storage_value);
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for OilDataMsg {
    fn try_from_frame(frame: &impl Frame) -> Result<Self, CanError> {
        if frame.data().len() < Self::LEN {
            return Err(CanError::InvalidPayloadSize);
        }
        if frame.id() != Self::ID {
            return Err(CanError::InvalidFrameId);
        }
        let mut buf = [0u8; Self::LEN];
        buf.copy_from_slice(&frame.data()[..Self::LEN]);
        Ok(Self { data: buf })
    }
    fn encode(&self) -> [u8; Self::LEN] {
        self.data
    }
}
