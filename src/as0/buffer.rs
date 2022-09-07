use std::ops::Index;

pub struct Buffer<const CAP: usize> {
    data: [u8; CAP],
    index: usize,
    cap: usize
}


impl<const CAP: usize> Buffer<CAP> {
    pub fn new() -> Buffer<CAP> {
        Buffer {
            data: [0; CAP],
            index: 0,
            cap: CAP
        }
    }

    pub fn push(&mut self, byte: u8) {
        self.data[self.index] = byte;
        self.index += 1;
    }

    pub fn data(&self) -> &[u8; CAP] {
        &self.data
    }

    pub fn size(&self) -> usize {
        self.index
    }

    pub fn cap(&self) -> usize {
        self.cap
    }
}


impl<const CAP: usize> Index<usize> for Buffer<CAP> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_buffer() {
        let buf: Buffer<32> = Buffer::new();
        assert_eq!(buf.cap(), 32);
    }

    #[test]
    fn access_by_bracket() {
        let mut buf: Buffer<32> = Buffer::new();
        buf.push(0x4);
        assert_eq!(buf[0], 0x4);
    }

    #[test]
    fn push_byte_into_buffer() {
        let mut buf: Buffer<32> = Buffer::new();
        buf.push(0x3);
    }

    #[test]
    fn read_all_buffer() {
        let mut buf: Buffer<4> = Buffer::new();
        buf.push(0x1);
        buf.push(0x2);
        buf.push(0x3);
        buf.push(0x4);

        let data = buf.data();
        assert_eq!(*data, [0x1, 0x2, 0x3, 0x4])
    }

    #[test]
    #[should_panic]
    fn buffer_overfllow() {
        let mut buf: Buffer<4> = Buffer::new();
        buf.push(0x1);
        buf.push(0x1);
        buf.push(0x1);
        buf.push(0x1);
        buf.push(0x1);
    }

    #[test]
    fn buffer_size() {
        let mut buf: Buffer<4> = Buffer::new();
        buf.push(0x1);
        buf.push(0x1);

        assert_eq!(buf.size(), 2);
    }
}