use derive_more::*;
use derive_new::new as New;
use encoding_rs::WINDOWS_1252;
use itermore::IterNextChunk as _;

use crate::Error;
use crate::FormID;
use crate::IRef;
use crate::List;
use crate::Result;
use crate::SysTime;
use crate::result_helper::ResultHelper;

#[derive(New, From, Deref, DerefMut)]
pub struct Scanner<'a, I: Iterator<Item = u8>>(&'a mut I);

impl<'a, I: Iterator<Item = u8>> Scanner<'a, I> {
    /// Shorthand for WINDOWS_1252.decode(...)
    #[inline]
    fn decode_bytes(bytes: &[u8]) -> String {
        WINDOWS_1252.decode(bytes).0.into()
    }

    /// Shorthand for WINDOWS_1252.decode(...)
    #[inline]
    fn decode_iter<J: Iterator<Item = u8>>(bytes: J) -> String {
        let bytes: List<u8> = bytes.collect();
        Self::decode_bytes(&bytes)
    }

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

    pub fn bool(&mut self) -> Result<bool> {
        let res = self.u8()?;
        debug_assert!(res == 0 || res == 1);
        Ok(res != 0)
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

    /// Read a byte-length-prefaced string.
    pub fn bstring(&mut self) -> Result<String> {
        let len = self.u8()? as usize;
        let encoded_bytes = self.take(len);
        let res = Self::decode_iter(encoded_bytes);
        Ok(res)
    }

    /// Read a byte-length-prefaced, zero-terminated string.
    pub fn bzstring(&mut self) -> Result<String> {
        let len = self.u8()? as usize;
        let encoded_bytes = self.take(len).take_while(|b| *b != 0);
        let res = Self::decode_iter(encoded_bytes);
        Ok(res)
    }

    pub fn form_id(&mut self) -> Result<FormID> {
        let res = self.u32()?;
        Ok(FormID::new(res))
    }

    pub fn iref(&mut self) -> Result<IRef> {
        let res = self.u32()?;
        Ok(IRef::new(res))
    }
}
