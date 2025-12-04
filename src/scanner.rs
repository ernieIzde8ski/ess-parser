use derive_more::*;
use derive_new::new as New;
use encoding_rs::WINDOWS_1252;
use itermore::IterNextChunk as _;

use crate::Error;
use crate::Result;
use crate::SysTime;
use crate::result_helper::ResultHelper;

#[derive(New, From, Deref, DerefMut)]
pub struct Scanner<'a, I: Iterator<Item = u8>>(&'a mut I);

impl<'a, I: Iterator<Item = u8>> Scanner<'a, I> {
    pub fn u8(&mut self) -> Result<u8> {
        self.next().map(u8::from_le).ok_or(Error::UnexpectedEOF)
    }

    // Note: When MS says "WORD", this is what they mean.
    pub fn u16(&mut self) -> Result<u16>
    where
        Self: Sized,
    {
        self.next_array()
            .map(u16::from_le_bytes)
            .replace_err(Error::UnexpectedEOF)
    }

    pub fn u32(&mut self) -> Result<u32>
    where
        Self: Sized,
    {
        self.next_array()
            .map(u32::from_le_bytes)
            .replace_err(Error::UnexpectedEOF)
    }

    pub fn f32(&mut self) -> Result<f32>
    where
        Self: Sized,
    {
        self.next_array()
            .map(f32::from_le_bytes)
            .replace_err(Error::UnexpectedEOF)
    }

    /// Read a SysTime.
    ///
    /// See SYSTEMTIME: <https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-systemtime>
    pub fn systime(&mut self) -> Result<SysTime> {
        let year = self.u16()?;
        let month = self.u16()?;
        let weekday = self.u16()?;
        let day = self.u16()?;
        let hour = self.u16()?;
        let minute = self.u16()?;
        let seconds = self.u16()?;
        let milliseconds = self.u16()?;
        Ok(SysTime {
            year,
            month,
            weekday,
            day,
            hour,
            minute,
            seconds,
            milliseconds,
        })
    }

    /// Read a length-prefaced, zero-terminated string.
    pub fn bzstring(&mut self) -> Result<String> {
        let len = self.u8()? as usize;
        let encoded_bytes: Vec<u8> = self.take(len).take_while(|b| *b != 0).collect();
        let (cowstr, _, _) = WINDOWS_1252.decode(&encoded_bytes);
        Ok(cowstr.into())
    }
}
