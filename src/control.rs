use actix_web::{HttpRequest, HttpResponse};
use actix_ws::{Message, MessageStream, Session as WsSession};
use anyhow::Context;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "cmd")]
enum Request {
    LoginPwd { username: String },
    Logout {},
}

struct Session {
    user_id: Option<u32>,
}

impl Session {
    pub fn new() -> Session {
        Session { user_id: None }
    }

    pub async fn execute(&mut self, req: Request) -> anyhow::Result<Response> {
        match req {
            Request::LoginPwd { username } => {
                anyhow::ensure!(self.user_id.is_none(), "already logged in");
                log::info!("User {username:?} logged in");
                self.user_id = Some(1);
                Ok(Response::Empty {})
            }
            Request::Logout {} => {
                anyhow::ensure!(self.user_id.is_some(), "not logged in yet");
                log::info!("User ID {:?} logged out", self.user_id);
                self.user_id = None;
                Ok(Response::Empty {})
            }
        }
    }
}

#[derive(Serialize)]
struct JsonResponse {
    err: Option<String>,
    #[serde(flatten)]
    inner: Response,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Response {
    Empty {},
}

impl From<anyhow::Result<Response>> for JsonResponse {
    fn from(value: anyhow::Result<Response>) -> Self {
        match value {
            Ok(resp) => JsonResponse {
                err: None,
                inner: resp,
            },
            Err(err) => JsonResponse {
                err: Some(format!("{err:#}")),
                inner: Response::Empty {},
            },
        }
    }
}

async fn worker(mut ws_session: WsSession, mut msg_stream: MessageStream) {
    let mut session = Session::new();
    while let Some(msg) = msg_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let result = serde_json::from_str(text.as_ref()).context("parse JSON");
                let result = match result {
                    Ok(req) => session.execute(req).await,
                    Err(err) => Err(err),
                };
                if let Err(err) = &result {
                    log::debug!("Failed to process request: {err:#}");
                }
                if let Err(err) = ws_session
                    .text(serde_json::to_string(&JsonResponse::from(result)).unwrap())
                    .await
                {
                    log::error!("Failed to send response to client: {err:#}");
                }
            }
            Ok(Message::Pong(_)) => (),
            Ok(other) => {
                log::debug!("Ignoring unknown WebSocket message: {other:?}");
            }
            Err(err) => {
                log::error!("WebSocket error: {err}");
                break;
            }
        }
    }
    log::debug!("Client closed connection");
    let _ = ws_session.close(None).await;
}

pub async fn websocket(
    req: HttpRequest,
    stream: actix_web::web::Payload,
) -> actix_web::Result<HttpResponse> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;
    actix_web::rt::spawn(worker(session, msg_stream));
    Ok(res)
}
