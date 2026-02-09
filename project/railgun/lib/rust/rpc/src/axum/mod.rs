use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use axum::body::Bytes;
use axum::extract::FromRequestParts;
use axum::extract::ws;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use futures_util::Sink;
use futures_util::SinkExt;
use futures_util::Stream;
use futures_util::StreamExt;
use serde::Serialize;
use serde::de::DeserializeOwned;
use typegen::Type;

use self::codec::Codec;
use self::codec::JsonCodec;

pub mod codec;

#[derive(Debug)]
pub struct TypedWebSocketUpgrade<TServer, TClient, TCodec = JsonCodec>
where
    TCodec: Codec,
{
    upgrade: ws::WebSocketUpgrade,
    _marker: PhantomData<fn() -> (TServer, TClient, TCodec)>,
}

// #[async_trait]
impl<TServer, TClient, TCodec, S> FromRequestParts<S>
    for TypedWebSocketUpgrade<TServer, TClient, TCodec>
where
    TServer: Type,
    TClient: Type,
    TCodec: Codec,
    S: Send + Sync,
{
    type Rejection = <ws::WebSocketUpgrade as FromRequestParts<S>>::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let upgrade =
            <ws::WebSocketUpgrade as FromRequestParts<S>>::from_request_parts(parts, state).await?;

        Ok(Self {
            upgrade,
            _marker: PhantomData,
        })
    }
}

impl<TServer, TClient, TCodec> TypedWebSocketUpgrade<TServer, TClient, TCodec>
where
    TServer: Send + Type,
    TClient: Send + Type,
    TCodec: Codec,
{
    pub fn on_upgrade<F, Fut>(self, callback: F) -> impl IntoResponse
    where
        F: FnOnce(TypedWebSocket<TServer, TClient, TCodec>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.upgrade
            .on_upgrade(|socket| async move {
                let socket = TypedWebSocket {
                    socket,
                    _marker: PhantomData,
                };

                callback(socket).await;
            })
            .into_response()
    }

    pub fn map<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ws::WebSocketUpgrade) -> ws::WebSocketUpgrade,
    {
        self.upgrade = f(self.upgrade);
        self
    }

    pub fn into_inner(self) -> ws::WebSocketUpgrade {
        self.upgrade
    }
}

/// A typesafe variant of [`axum::extract::ws::WebSocket`].
#[derive(Debug)]
pub struct TypedWebSocket<TServer, TClient, TCodec = JsonCodec>
where
    TServer: Type,
    TClient: Type,
    TCodec: Codec,
{
    socket: ws::WebSocket,
    _marker: PhantomData<fn() -> (TServer, TClient, TCodec)>,
}

impl<TServer, TClient, TCodec> TypedWebSocket<TServer, TClient, TCodec>
where
    TServer: Serialize + Type,
    TClient: DeserializeOwned + Type,
    TCodec: Codec,
{
    /// Receive another message.
    ///
    /// Returns `None` if the stream stream has closed.
    ///
    /// This is analagous to [`axum::extract::ws::WebSocket::recv`] but with a
    /// statically typed message.
    pub async fn recv(
        &mut self,
    ) -> Option<Result<TypedMessage<TClient>, SocketError<TCodec::Error>>>
    where
        TClient: DeserializeOwned,
        TCodec: Codec,
    {
        self.next().await
    }

    /// Send a message.
    ///
    /// This is analagous to [`axum::extract::ws::WebSocket::send`] but with a
    /// statically typed message.
    pub async fn send(
        &mut self,
        item: TypedMessage<TServer>,
    ) -> Result<(), SocketError<TCodec::Error>> {
        SinkExt::send(self, item).await
    }

    /// Gracefully close this WebSocket.
    ///
    /// This is analagous to [`axum::extract::ws::WebSocket::close`].
    pub async fn close(mut self) -> Result<(), SocketError<TCodec::Error>> {
        self.socket.close().await.map_err(SocketError::Ws)
    }

    /// Get the inner axum [`axum::extract::ws::WebSocket`].
    pub fn into_inner(self) -> ws::WebSocket {
        self.socket
    }
}

impl<TServer, TClient, TCodec> Stream for TypedWebSocket<TServer, TClient, TCodec>
where
    TServer: Serialize + Type,
    TClient: DeserializeOwned + Type,
    TCodec: Codec,
{
    type Item = Result<TypedMessage<TClient>, SocketError<TCodec::Error>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let msg = futures_util::ready!(
            Pin::new(&mut self.socket)
                .poll_next(cx)
                .map_err(SocketError::Ws)?
        );

        if let Some(msg) = msg {
            let msg = match msg {
                ws::Message::Text(msg) => msg.as_bytes().to_owned().into(),
                ws::Message::Binary(bytes) => bytes,
                ws::Message::Close(frame) => {
                    return Poll::Ready(Some(Ok(TypedMessage::Close(frame))));
                }
                ws::Message::Ping(buf) => {
                    return Poll::Ready(Some(Ok(TypedMessage::Ping(buf))));
                }
                ws::Message::Pong(buf) => {
                    return Poll::Ready(Some(Ok(TypedMessage::Pong(buf))));
                }
            };

            let msg = TCodec::decode(msg)
                .map(TypedMessage::Item)
                .map_err(SocketError::Codec);
            Poll::Ready(Some(msg))
        } else {
            Poll::Ready(None)
        }
    }
}

impl<TServer, TClient, TCodec> Sink<TypedMessage<TServer>>
    for TypedWebSocket<TServer, TClient, TCodec>
where
    TServer: Serialize + Type,
    TClient: DeserializeOwned + Type,
    TCodec: Codec,
{
    type Error = SocketError<TCodec::Error>;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.socket)
            .poll_ready(cx)
            .map_err(SocketError::Ws)
    }

    fn start_send(
        mut self: Pin<&mut Self>,
        item: TypedMessage<TServer>,
    ) -> Result<(), Self::Error> {
        let msg = match item {
            TypedMessage::Item(buf) => {
                ws::Message::Binary(TCodec::encode(buf).map_err(SocketError::Codec)?)
            }
            TypedMessage::Ping(buf) => ws::Message::Ping(buf),
            TypedMessage::Pong(buf) => ws::Message::Pong(buf),
            TypedMessage::Close(frame) => ws::Message::Close(frame),
        };

        Pin::new(&mut self.socket)
            .start_send(msg)
            .map_err(SocketError::Ws)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.socket)
            .poll_flush(cx)
            .map_err(SocketError::Ws)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.socket)
            .poll_close(cx)
            .map_err(SocketError::Ws)
    }
}

/// Errors that can happen when using this library.
#[derive(Debug)]
pub enum SocketError<E> {
    /// Something went wrong with the WebSocket.
    Ws(axum::Error),
    /// Something went wrong with the [`Codec`].
    Codec(E),
}

impl<E> fmt::Display for SocketError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SocketError::Ws(inner) => inner.fmt(f),
            SocketError::Codec(inner) => inner.fmt(f),
        }
    }
}

impl<E> StdError for SocketError<E>
where
    E: StdError + 'static,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            SocketError::Ws(inner) => Some(inner),
            SocketError::Codec(inner) => Some(inner),
        }
    }
}

/// A WebSocket message contain a value of a known type.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedMessage<T> {
    /// An item of type `T`.
    Item(T),
    /// A ping message with the specified payload
    ///
    /// The payload here must have a length less than 125 bytes
    Ping(Bytes),
    /// A pong message with the specified payload
    ///
    /// The payload here must have a length less than 125 bytes
    Pong(Bytes),
    /// A close message with the optional close frame.
    Close(Option<ws::CloseFrame>),
}
