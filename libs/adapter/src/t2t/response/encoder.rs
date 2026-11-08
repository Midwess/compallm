use super::event::ResponseEvent;
use crate::{codec::EncodeStep, error::CodecError};

pub trait ResponseEncoder {
    /// Called after NeedEvent; retain at most one bounded event.
    fn accept(&mut self, event: ResponseEvent) -> Result<(), CodecError>;

    /// Write a nonzero prefix within output, request another event, or finish.
    /// NeedEvent remains repeatable until accept/finish. Each call must do bounded
    /// work; target JSON/SSE framing and escaping belong to this encoder.
    fn advance(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError>;

    /// Called once after source EOF validation. Drain final output with advance,
    /// then return Finished; returning NeedEvent after finish is a contract error.
    fn finish(&mut self) -> Result<(), CodecError>;
}

impl<E: ResponseEncoder + ?Sized> ResponseEncoder for Box<E> {
    fn accept(&mut self, event: ResponseEvent) -> Result<(), CodecError> {
        (**self).accept(event)
    }
    fn advance(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        (**self).advance(output)
    }
    fn finish(&mut self) -> Result<(), CodecError> {
        (**self).finish()
    }
}
