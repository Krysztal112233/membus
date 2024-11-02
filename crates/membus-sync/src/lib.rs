use std::io::{self, BufWriter, Cursor, IntoInnerError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Input/Output error
    #[error("{0}")]
    IO(#[from] io::Error),

    /// When cannot decode packet layout
    #[error("unknown packet layout at: {0}")]
    UnknownPacketLayout(usize),

    #[error("{0}")]
    IntoInner(#[from] IntoInnerError<BufWriter<Cursor<Vec<u8>>>>),

    /// Cast failed. Mostly are size mismatched
    #[error("{0}")]
    CastFailed(#[from] bytemuck::PodCastError),

    /// Codec failed
    #[error("{0}")]
    Codec(Box<dyn ::std::error::Error>),
}

/// Socket management
pub mod sock;

/// Packet definition
pub mod packet;

/// Sync signals
pub mod signal;
