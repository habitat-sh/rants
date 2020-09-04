#[cfg(feature = "native-tls")]
use native_tls_crate::{self, TlsConnector};
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    io::{self, AsyncRead, AsyncWrite},
    net::TcpStream,
};
#[cfg(feature = "native-tls")]
use tokio_native_tls::{TlsConnector as TokioTlsConnector, TlsStream};

/// A simple wrapper type that can either be a raw TCP stream or a TCP stream with TLS enabled.
#[pin_project(project = TlsOrTcpStreamProj)]
#[derive(Debug)]
pub enum TlsOrTcpStream {
    TcpStream(#[pin] TcpStream),
    #[cfg(feature = "native-tls")]
    TlsStream(#[pin] TlsStream<TcpStream>),
}

impl TlsOrTcpStream {
    pub fn new(stream: TcpStream) -> Self {
        Self::TcpStream(stream)
    }

    #[cfg(feature = "native-tls")]
    pub async fn upgrade(
        self,
        tls_connector: TlsConnector,
        domain: &str,
    ) -> Result<Self, native_tls_crate::Error> {
        Ok(match self {
            Self::TcpStream(stream) => {
                let tokio_tls_connector = TokioTlsConnector::from(tls_connector);
                let tls_stream = tokio_tls_connector.connect(domain, stream).await?;
                Self::TlsStream(tls_stream)
            }
            Self::TlsStream(stream) => Self::TlsStream(stream),
        })
    }
}

impl AsyncRead for TlsOrTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match self.project() {
            TlsOrTcpStreamProj::TcpStream(stream) => stream.poll_read(cx, buf),
            #[cfg(feature = "native-tls")]
            TlsOrTcpStreamProj::TlsStream(stream) => stream.poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for TlsOrTcpStream {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        match self.project() {
            TlsOrTcpStreamProj::TcpStream(stream) => stream.poll_write(cx, buf),
            #[cfg(feature = "native-tls")]
            TlsOrTcpStreamProj::TlsStream(stream) => stream.poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        match self.project() {
            TlsOrTcpStreamProj::TcpStream(stream) => stream.poll_flush(cx),
            #[cfg(feature = "native-tls")]
            TlsOrTcpStreamProj::TlsStream(stream) => stream.poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        match self.project() {
            TlsOrTcpStreamProj::TcpStream(stream) => stream.poll_shutdown(cx),
            #[cfg(feature = "native-tls")]
            TlsOrTcpStreamProj::TlsStream(stream) => stream.poll_shutdown(cx),
        }
    }
}
