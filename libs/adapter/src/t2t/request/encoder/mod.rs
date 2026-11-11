mod output_format;
mod stop;
mod writer;

use crate::{codec::EncodeStep, error::CodecError, t2t::request::event::RequestEvent};

pub(crate) use output_format::FormatWriter;
pub(crate) use stop::{StopWriter, Stops};
pub(crate) use writer::Encoder;

pub trait RequestEncoder {
    /// Called only after NeedEvent. Retain metadata, never payload bytes.
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError>;

    /// Write into the supplied slice without blocking.
    fn advance(&mut self, text: &mut &[u8], output: &mut [u8]) -> Result<EncodeStep, CodecError>;

    /// Called once after source EOF validation, while the encoder needs an event.
    fn finish(&mut self) -> Result<(), CodecError>;
}

impl<E: RequestEncoder + ?Sized> RequestEncoder for Box<E> {
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        (**self).accept(event)
    }

    fn advance(&mut self, text: &mut &[u8], output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        (**self).advance(text, output)
    }

    fn finish(&mut self) -> Result<(), CodecError> {
        (**self).finish()
    }
}
