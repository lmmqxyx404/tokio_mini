use std::io;

#[derive(Debug)]
pub(crate) struct Driver {}

#[derive(Debug)]
pub(crate) struct Handle {}

pub(crate) struct Cfg {}

impl Driver {
    pub(crate) fn new(cfg: Cfg) -> io::Result<(Self, Handle)> {
        todo!()
    }
}
