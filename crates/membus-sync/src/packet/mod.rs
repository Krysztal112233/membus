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

#[derive(Debug)]
pub struct PacketBufWriter(BufWriter<Cursor<Vec<u8>>>);

impl PacketBufWriter {
    /// Get [`PacketBuf`] from self and consume ownership
    ///
    /// # Panic
    ///
    /// [`BufWriter::into_inner`] will cause panic.
    pub fn into_inner(self) -> PacketBuf {
        self.try_into_inner().unwrap()
    }

    /// Safely get [`PacketBuf`] and consume ownership
    pub fn try_into_inner(self) -> Result<PacketBuf, Error> {
        Ok(PacketBuf(self.0.into_inner()?.into_inner()))
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

#[derive(Debug)]
pub struct PacketBufReader(BufReader<Cursor<Vec<u8>>>);

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
    fn decode(buf: &[u8]) -> Result<Self, Error>;
}

/// Typed reading from packet
pub trait PacketReader: Read {
    /// Read and consume data into `T`
    fn read_into<T>(&mut self) -> Result<T, Error>
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
    fn read_into_slice<T>(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::with_capacity(len);

        for _ in 0..len {
            buf.push(self.read_into()?)
        }

        Ok(buf)
    }
}

impl PacketReader for PacketBufReader {}
