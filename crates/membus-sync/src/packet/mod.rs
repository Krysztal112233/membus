use std::{
    io::{BufReader, BufWriter, Cursor, Read, Write},
    ops::{Deref, DerefMut},
};

use bytemuck::{Pod, Zeroable};

use crate::Error;

mod signal;

#[derive(Debug, Clone, Default)]
pub struct PacketBuf(Vec<u8>);

impl PacketBuf {
    /// Constructing [`PacketBuf`] via [`Default::default`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructing new [`PacketBuf`] with provided capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl Deref for PacketBuf {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PacketBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for PacketBuf {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for PacketBuf {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<PacketBuf> for PacketBufReader {
    fn from(value: PacketBuf) -> Self {
        PacketBufReader(BufReader::new(Cursor::new(value.0)))
    }
}

impl From<PacketBuf> for PacketBufWriter {
    fn from(value: PacketBuf) -> Self {
        PacketBufWriter(BufWriter::new(Cursor::new(value.0)))
    }
}

/// Wrapper for [`BufWriter<Cursor<Vec<u8>>>`]
///
/// This wrapper provide [`Pod`] types writing
#[derive(Debug)]
pub struct PacketBufWriter(BufWriter<Cursor<Vec<u8>>>);

impl PacketBufWriter {
    /// Get [`PacketBuf`] from self and consume ownership
    ///
    /// # Panic
    ///
    /// [`BufWriter::into_inner`] might cause panic.
    pub fn into_inner(self) -> PacketBuf {
        self.try_into_inner().unwrap()
    }

    /// Safely get [`PacketBuf`] and consume ownership
    pub fn try_into_inner(self) -> Result<PacketBuf, Error> {
        Ok(PacketBuf(self.0.into_inner()?.into_inner()))
    }

    /// Safely write data with typed.
    pub fn write<T>(&mut self, value: T) -> Result<usize, Error>
    where
        T: Pod + Zeroable,
    {
        let bytes = bytemuck::bytes_of(&value);
        self.0.write_all(bytes)?;
        Ok(bytes.len())
    }

    /// Safely write slice with typed.
    pub fn write_slice<T>(&mut self, slice: &[T]) -> Result<usize, Error>
    where
        T: Pod + Zeroable,
    {
        let bytes = bytemuck::cast_slice(slice);
        self.write(slice.len())?;
        self.0.write_all(bytes)?;
        Ok(bytes.len())
    }
}

impl Write for PacketBufWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

/// Wrapper for [`BufReader<Cursor<Vec<u8>>>`]
///
/// This wrapper provide [`Pod`] type's reading.
#[derive(Debug)]
pub struct PacketBufReader(BufReader<Cursor<Vec<u8>>>);

impl PacketBufReader {
    /// Read and consume data into `T`
    pub fn read<T>(&mut self) -> Result<T, Error>
    where
        T: Pod + Zeroable,
    {
        let size = std::mem::size_of::<T>();
        let mut buf = vec![0u8; size];
        self.read_exact(&mut buf)?;

        // Safely casting
        let casted = bytemuck::try_from_bytes::<T>(&buf).copied()?;

        Ok(casted)
    }

    /// Read and consume data into `Vec<T>`
    pub fn read_slice<T>(&mut self, buf: &mut [T]) -> Result<usize, Error>
    where
        T: Pod + Zeroable,
    {
        let byte_buf = bytemuck::cast_slice_mut(buf);
        self.0.read_exact(byte_buf)?;
        Ok(buf.len())
    }

    /// This function will consume first element of [`BufReader`], then cast it into [`usize`],
    /// and read length of the casted [`usize`].
    pub fn read_sized<T>(&mut self) -> Result<Vec<T>, Error>
    where
        T: Pod + Zeroable,
    {
        let fixed = self.read::<usize>()?;
        let mut buf = Vec::with_capacity(fixed);
        self.read_slice(&mut buf)?;
        Ok(buf)
    }
}

impl Read for PacketBufReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

pub trait PacketBufCodec: Sized {
    /// Encoding struct into [`PacketBuf`]
    ///
    /// # Error
    ///
    /// Mostly are [`Error::IO`]
    fn encode(self) -> Result<PacketBuf, Error>;

    /// Decode from [`PacketBuf`] into target
    ///
    /// # Error
    ///
    /// Mostly are serializing failed.
    fn decode(buf: PacketBuf) -> Result<Self, Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // 定义一个测试用的 Pod 类型
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
    struct TestData {
        a: u32,
    }

    #[test]
    fn test_packet_buf_writer_write() {
        let cursor = Cursor::new(Vec::new());
        let buf_writer = BufWriter::new(cursor);
        let mut writer = PacketBufWriter(buf_writer);

        let test_data = TestData { a: 12345 };

        let bytes_written = writer.write(test_data).unwrap();
        let expected_size = std::mem::size_of::<TestData>();
        assert_eq!(bytes_written, expected_size);

        let packet_buf = writer.into_inner();

        let expected_bytes = bytemuck::bytes_of(&test_data);
        assert_eq!(packet_buf.0, expected_bytes);
    }
}
