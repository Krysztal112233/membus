use crate::{packet::PacketBufReader, signal::SyncSignal, Error};

use super::{PacketBuf, PacketBufCodec, PacketBufWriter};

#[doc = include_str!("signal.md")]
#[derive(Debug, Default, Clone)]
pub struct SyncSignalPacket {
    /// Signal type
    pub signal: SyncSignal,

    /// Payload of this packet
    pub payload: Vec<u8>,
}

#[allow(unused)]
impl SyncSignalPacket {
    /// Constructing  [`SignalPacket`] with empty [`SignalPacket::payload`]
    pub fn new(signal: SyncSignal) -> Self {
        Self {
            signal,
            ..Default::default()
        }
    }
}

impl PacketBufCodec for SyncSignalPacket {
    fn encode(self) -> Result<PacketBuf, Error> {
        let mut buffer: PacketBufWriter = PacketBuf::with_capacity(self.payload.len() + 1).into();

        buffer.write(self.signal as u8)?;
        buffer.write_slice(&self.payload)?;

        Ok(buffer.into_inner())
    }

    fn decode(buf: PacketBuf) -> Result<SyncSignalPacket, Error> {
        let mut buf: PacketBufReader = buf.into();

        let signal = SyncSignal::from_repr(buf.read::<u8>()?).unwrap_or_default();
        let payload = buf.read_sized::<u8>()?;

        Ok(Self { signal, payload })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codec() {
        let packet = SyncSignalPacket::default();

        let packet = packet.encode().expect("SyncSignalPacket::encode");

        SyncSignalPacket::decode(packet).expect("SyncSignalPacket::decode");
    }
}
