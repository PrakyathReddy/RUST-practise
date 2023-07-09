fn main() {
    use std::io::Result;

    pub trait Read {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    }

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
    }
}
