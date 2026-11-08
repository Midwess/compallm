use super::event::RequestEvent;
use crate::{codec::EncodeStep, error::CodecError};

pub trait RequestEncoder {
    /// Called only after NeedEvent. Retain at most one bounded event.
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError>;

    /// Write into the supplied slice without blocking. Wrote must report a
    /// nonzero length within the slice. NeedEvent must remain repeatable until
    /// accept/finish. Finished is valid only after finish and all output drains.
    fn advance(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError>;

    /// Called once after source EOF validation, while the encoder needs an event.
    /// Subsequent advance calls must drain final output and then return Finished.
    fn finish(&mut self) -> Result<(), CodecError>;
}

impl<E: RequestEncoder + ?Sized> RequestEncoder for Box<E> {
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        (**self).accept(event)
    }
    fn advance(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        (**self).advance(output)
    }
    fn finish(&mut self) -> Result<(), CodecError> {
        (**self).finish()
    }
}
