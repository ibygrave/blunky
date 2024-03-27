pub struct StrBuf<const N: usize> {
    buf: [u8; N],
    cursor: usize,
}

impl<const N: usize> Default for StrBuf<N> {
    fn default() -> Self {
        Self {
            buf: [0_u8; N],
            cursor: 0,
        }
    }
}

impl<const N: usize> StrBuf<N> {
    pub fn clear(&mut self) {
        self.cursor = 0;
    }
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.cursor]).unwrap()
    }
}

impl<const N: usize> core::fmt::Write for StrBuf<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let tail = &mut self.buf[self.cursor..];
        if tail.len() < bytes.len() {
            return Err(core::fmt::Error);
        }
        let tail = &mut tail[..bytes.len()];
        tail.copy_from_slice(bytes);
        self.cursor += bytes.len();
        Ok(())
    }
}
