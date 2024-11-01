use crate::{signal::SyncSignal, Error};

use super::{PacketBuf, PacketBufCodec};

#[doc = include_str!("signal.md")]
#[derive(Debug, Default)]
pub struct SignalPacket {
    /// Signal type
    pub signal: SyncSignal,

    /// Payload of this packet
    pub payload: PacketBuf,
}

impl SignalPacket {
    /// Constructing  [`SignalPacket`] with empty [`SignalPacket::payload`]
    pub fn new(signal: SyncSignal) -> Self {
        Self {
            signal,
            ..Default::default()
        }
    }
}

impl PacketBufCodec for SignalPacket {
    fn encode(self) -> Result<PacketBuf, Error> {
        let mut buffer = PacketBuf::with_capacity(self.payload.len() + 1);

        buffer.push(self.signal as u8);

        #[cfg(target_endian = "little")]
        buffer.extend_from_slice(&self.payload.len().to_le_bytes());
        #[cfg(target_endian = "big")]
        buffer.extend_from_slice(&self.payload.len().to_be_bytes());

        buffer.extend_from_slice(&self.payload);

        Ok(buffer)
    }

    fn decode(buf: &[u8]) -> Result<SignalPacket, Error> {
        todo!()
    }
}
