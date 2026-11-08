use std::{
    io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use adapter::replay::ReplayStore;
use bytes::Bytes;
use futures_core::Stream;
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};
use tokio_util::io::ReaderStream;

const MAX_BODY: u64 = 8 * 1024 * 1024;

/// Private temporary file, removed when store and all readers have been dropped.
pub struct DiskReplayStore {
    path: Option<Arc<tempfile::TempPath>>,
    writer: Option<BufWriter<File>>,
    bytes: u64,
    sealed: bool,
}

impl DiskReplayStore {
    pub async fn new() -> io::Result<Self> {
        let temporary = tokio::task::spawn_blocking(tempfile::NamedTempFile::new).await??;
        let (file, path) = temporary.into_parts();
        Ok(Self {
            path: Some(Arc::new(path)),
            writer: Some(BufWriter::new(File::from_std(file))),
            bytes: 0,
            sealed: false,
        })
    }
}

pub struct DiskReader {
    stream: ReaderStream<File>,
    remaining: u64,
    _path: Arc<tempfile::TempPath>,
    done: bool,
}

impl Stream for DiskReader {
    type Item = io::Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.done {
            return Poll::Ready(None);
        }
        match Pin::new(&mut self.stream).poll_next(cx) {
            Poll::Ready(Some(Ok(bytes))) => {
                if bytes.len() as u64 > self.remaining {
                    self.done = true;
                    return Poll::Ready(Some(Err(io::Error::other("replay length changed"))));
                }
                self.remaining -= bytes.len() as u64;
                Poll::Ready(Some(Ok(bytes)))
            }
            Poll::Ready(None) => {
                self.done = true;
                if self.remaining != 0 {
                    Poll::Ready(Some(Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "truncated replay",
                    ))))
                } else {
                    Poll::Ready(None)
                }
            }
            Poll::Ready(Some(Err(error))) => {
                self.done = true;
                Poll::Ready(Some(Err(error)))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl ReplayStore for DiskReplayStore {
    type Error = io::Error;
    type Reader = DiskReader;

    async fn append(&mut self, chunk: Bytes) -> io::Result<()> {
        if self.sealed || self.bytes + chunk.len() as u64 > MAX_BODY {
            return Err(io::Error::other("replay closed or body exceeds 8 MiB"));
        }
        self.writer
            .as_mut()
            .ok_or_else(|| io::Error::other("store closed"))?
            .write_all(&chunk)
            .await?;
        self.bytes += chunk.len() as u64;
        Ok(())
    }

    async fn seal(&mut self) -> io::Result<()> {
        self.writer
            .as_mut()
            .ok_or_else(|| io::Error::other("store closed"))?
            .flush()
            .await?;
        self.sealed = true;
        Ok(())
    }

    async fn open_reader(&mut self) -> io::Result<Self::Reader> {
        if !self.sealed {
            return Err(io::Error::other("store not sealed"));
        }
        let path = self
            .path
            .as_ref()
            .ok_or_else(|| io::Error::other("store closed"))?
            .clone();
        let file = File::open(path.as_ref()).await?;
        Ok(DiskReader {
            stream: ReaderStream::new(file),
            remaining: self.bytes,
            _path: path,
            done: false,
        })
    }

    async fn abort(&mut self) -> io::Result<()> {
        self.writer = None;
        self.path = None;
        self.sealed = false;
        Ok(())
    }
}
