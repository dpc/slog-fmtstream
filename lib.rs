//! `!#[no_std]` streamer for `slog-rs`
#![warn(missing_docs)]

#![no_std]

#[macro_use]
extern crate slog;

use slog::*;

use core::{fmt, result};

/// Formats `Record`-s into `fmt::Write`
pub trait Format: Send + Sync + Sized {
    /// Format one logging record and write into `io`
    fn format(&self,
              _io: &mut fmt::Write,
              _info: &Record,
              _logger_values: &OwnedKeyValueList)
              -> result::Result<(), fmt::Error>;
}

/// `!#[no_std]` streamer
pub struct Streamer<W, F> {
    _io : W,
    _format : F,
}

impl<W: 'static + fmt::Write + Send+Sync, F: Format + Send> Drain for Streamer<W, F> {
    type Error = fmt::Error;

    fn log(&self,
           _info: &Record,
           _logger_values: &OwnedKeyValueList)
        ->  result::Result<(), fmt::Error> {
            Ok(())
        }
}
