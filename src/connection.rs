use crate::Message as CrateMessage;
use async_tungstenite::tokio::{connect_async, ConnectStream};
use async_tungstenite::WebSocketStream;
use futures::stream::{SplitSink, SplitStream};
use iced::futures::{SinkExt, StreamExt};
use iced::Command;
use reciprocity_communication::messages::oauth2::RefreshToken;
use reciprocity_communication::messages::{Auth, AuthMessage, ClientRequest, Message, User};
use std::sync::Arc;
use tokio::sync::Mutex;
use tungstenite::Message as TungMessage;

#[derive(Debug, Clone)]
pub struct Connection {
    send: Arc<Mutex<SplitSink<WebSocketStream<ConnectStream>, TungMessage>>>,
    rec: Arc<Mutex<SplitStream<WebSocketStream<ConnectStream>>>>,
}

#[derive(Debug, Clone)]
pub enum ConnectionError {
    Tungstenite(Arc<tungstenite::Error>),
    RmpSerdeEncode(Arc<rmp_serde::encode::Error>),
    RmpSerdeDecode(Arc<rmp_serde::decode::Error>),
    NonAuthMessage(Box<Message>),
}

impl From<tungstenite::Error> for ConnectionError {
    fn from(e: tungstenite::Error) -> Self {
        ConnectionError::Tungstenite(Arc::new(e))
    }
}

impl From<rmp_serde::encode::Error> for ConnectionError {
    fn from(e: rmp_serde::encode::Error) -> Self {
        ConnectionError::RmpSerdeEncode(Arc::new(e))
    }
}

impl From<rmp_serde::decode::Error> for ConnectionError {
    fn from(e: rmp_serde::decode::Error) -> Self {
        ConnectionError::RmpSerdeDecode(Arc::new(e))
    }
}

impl From<Message> for ConnectionError {
    fn from(m: Message) -> Self {
        ConnectionError::NonAuthMessage(Box::new(m))
    }
}

impl Connection {
    pub async fn new(
        auth: Auth,
        bot_http: String,
    ) -> Result<(Self, (User, RefreshToken)), ConnectionError> {
        let (mut socket, _) = connect_async(bot_http).await?;
        let auth_msg = Message::ClientRequest(ClientRequest::Authenticate(auth)).generate()?;
        socket.send(TungMessage::Binary(auth_msg)).await?;
        let resp = socket.next().await.expect("None Error expected")?;
        let resp = resp.into_data();
        let msg = Message::parse(resp.as_slice())?;
        let (user, token) = if let Message::Auth(AuthMessage::AuthSuccess(user, token)) = msg {
            (user, token)
        } else {
            return Err(msg.into());
        };
        let (send, rec) = socket.split();

        Ok((
            Connection {
                send: Arc::new(Mutex::new(send)),
                rec: Arc::new(Mutex::new(rec)),
            },
            (user, token),
        ))
    }

    pub async fn receive(self) -> Result<Message, ConnectionError> {
        let mut rec_lock = self.rec.lock().await;
        let msg = rec_lock.next().await.expect("None Error expected")?;
        let msg = msg.into_data();
        Message::parse(msg.as_slice()).map_err(|e| e.into())
    }

    pub async fn send(self, req: ClientRequest) -> Result<(), ConnectionError> {
        let mut send_lock = self.send.lock().await;
        let msg = Message::ClientRequest(req);
        let bin = msg.generate()?;
        send_lock
            .send(TungMessage::Binary(bin))
            .await
            .map_err(|e| e.into())
    }

    pub fn get_rec_cmd(&self) -> Command<CrateMessage> {
        Command::perform(self.clone().receive(), |res| {
            CrateMessage::ReceiveBotMessage(res)
        })
    }

    pub fn control_request(
        &self,
        req: reciprocity_communication::messages::PlayerControl,
    ) -> Command<crate::Message> {
        println!("Request: {:?}", req);
        Command::perform(self.clone().send(ClientRequest::Control(uuid::Uuid::new_v4().to_string(), req)), |res| {
            if let Err(e) = res {
                panic!("Error sending request. {:?}", e)
            }
            crate::Message::None()
        })
    }
}
