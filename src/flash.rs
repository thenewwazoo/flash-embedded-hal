//! Flash-related traits

pub trait Latency {
    type Latency;
    type Error;

    fn set_latency(&mut self, latency: Self::Latency) -> Result<(), Self::Error>;
}

pub trait Read {
    type Error;

    fn read<WORD>(&self, addr: usize) -> Result<WORD, Self::Error>;
}

pub trait WriteErase {
    type Error;
    type Status;

    fn status(&self) -> Result<Self::Status, Self::Error>;

    fn erase_page(&mut self, address: usize) -> Result<(), Self::Error>;

    fn program_word(&mut self, address: usize, value: u32) -> Result<(), Self::Error>;
}

pub trait Locking {
    type Error;

    fn is_locked(&self) -> bool;

    fn lock(&mut self);

    fn unlock(&mut self);
}
