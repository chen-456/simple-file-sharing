use std::time::{Duration, Instant};

use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Path as WebPath},
    HttpRequest, HttpResponse, ResponseError,
};
use actix_ws::{Item, Message, MessageStream, Session as WsSession};
use anyhow::Context;
use async_std::{
    fs::File,
    io::{prelude::SeekExt, WriteExt},
};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    safe_path::normalize_web_path_as_file,
    state::{AppState, UploadInfo},
};

struct Session {
    file: File,
    size: u64,
}

#[derive(Deserialize)]
#[serde(tag = "cmd")]
enum Request {
    Seek { pos: u64 },
    Finish {},
}

#[derive(Serialize)]
#[serde(untagged)]
enum Response {
    Empty {},
    BlockReceived { len: u64, cur_pos: u64 },
}

#[derive(Serialize)]
struct JsonResponse {
    err: Option<String>,
    #[serde(flatten)]
    inner: Response,
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

impl Session {
    pub async fn execute_req(&mut self, req: Request) -> anyhow::Result<Response> {
        match req {
            Request::Seek { pos } => {
                anyhow::ensure!(pos <= self.size, "cannot seek past end of file");
                self.file.seek(std::io::SeekFrom::Start(pos)).await?;
                Ok(Response::Empty {})
            }
            Request::Finish {} => {
                self.file.flush().await?;
                Ok(Response::Empty {})
            }
        }
    }

    pub async fn write_data(&mut self, data: &[u8]) -> anyhow::Result<Response> {
        let cur = self.file.seek(std::io::SeekFrom::Current(0)).await?;
        let write_len = data.len().min((self.size - cur) as usize);
        if write_len < data.len() {
            log::warn!(
                "An uploader sent {} stray bytes! Ignoring",
                data.len() - write_len,
            );
        }
        let data = &data[..write_len];
        if data.is_empty() {
            anyhow::bail!("already reached end of file");
        }

        self.file.write_all(data).await?;
        Ok(Response::BlockReceived {
            len: write_len as u64,
            cur_pos: cur + write_len as u64,
        })
    }
}

async fn worker(file: File, size: u64, mut ws_session: WsSession, mut msg_stream: MessageStream) {
    let mut session = Session { file, size };

    while let Some(msg) = msg_stream.next().await {
        match msg {
            // Dirty fix: we assume all Continuation packets are caused by
            // binary packets, and we do not track each packet's preceding
            // packet.
            // FIXME: better implementation
            Ok(Message::Binary(data)) | Ok(Message::Continuation(Item::Last(data))) => {
                let result = session.write_data(&data).await;
                if let Err(err) = &result {
                    log::debug!("Failed to process incoming data: {err:#}");
                }
                if let Err(err) = ws_session
                    .text(serde_json::to_string(&JsonResponse::from(result)).unwrap())
                    .await
                {
                    log::error!("Failed to send response to client: {err:#}");
                }
            }
            // Dirty fix: we assume all Continuation packets are caused by
            // binary packets, and we do not track each packet's preceding
            // packet.
            // FIXME: better implementation
            Ok(Message::Continuation(Item::FirstBinary(data)))
            | Ok(Message::Continuation(Item::Continue(data))) => {
                let result = session.write_data(&data).await;
                if let Err(err) = &result {
                    log::debug!("Failed to process incoming data: {err:#}");
                }
                // Don't send a response now, since the packet hasn't ended yet
            }
            Ok(Message::Text(text)) => {
                let result = serde_json::from_str(text.as_ref()).context("parse JSON");
                let result = match result {
                    Ok(req) => session.execute_req(req).await,
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

#[derive(thiserror::Error, Debug)]
pub enum UploadError {
    #[error("Parse UUID failed: {0}")]
    ParseUuid(uuid::Error),

    #[error("The upload link you specified does not exist, or has expired.")]
    LinkNotExists,

    #[error("File I/O error: {0}")]
    PrepareFile(std::io::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(actix_web::Error),
}

impl ResponseError for UploadError {
    fn status_code(&self) -> StatusCode {
        match self {
            UploadError::ParseUuid(_) => StatusCode::BAD_REQUEST,
            UploadError::LinkNotExists => StatusCode::NOT_FOUND,
            UploadError::PrepareFile(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UploadError::WebSocket(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/uploads/{uuid}")]
pub async fn upload(
    req: HttpRequest,
    stream: actix_web::web::Payload,
    uuid: WebPath<String>,
    state: Data<AppState>,
) -> Result<HttpResponse, UploadError> {
    let uuid = Uuid::parse_str(&uuid).map_err(UploadError::ParseUuid)?;
    let mut lock = state.uploads.lock().await;
    if let Some(upload_info) = lock.get(&uuid) {
        if Instant::now() < upload_info.expires {
            let file = File::create(&upload_info.file)
                .await
                .map_err(UploadError::PrepareFile)?;
            let size = upload_info.size;
            file.set_len(size).await.map_err(UploadError::PrepareFile)?;

            let (res, session, msg_stream) =
                actix_ws::handle(&req, stream).map_err(UploadError::WebSocket)?;
            actix_web::rt::spawn(worker(file, size, session, msg_stream));
            Ok(res)
        } else {
            log::debug!("Upload link {uuid} has expired");
            lock.remove(&uuid);
            Err(UploadError::LinkNotExists)
        }
    } else {
        Err(UploadError::LinkNotExists)
    }
}

pub async fn gen_upload_uuid(
    web_path: &str,
    size: u64,
    state: &Data<AppState>,
) -> anyhow::Result<String> {
    let normalized = normalize_web_path_as_file(web_path)?;
    let exists = async_std::path::Path::new(&normalized).exists().await;
    if !exists {
        let mut lock = state.uploads.lock().await;
        let mut num_tries = 0;
        while num_tries < 20 {
            num_tries += 1;
            let uuid = uuid::Builder::from_random_bytes(rand::random()).into_uuid();
            if lock.contains_key(&uuid) {
                continue;
            }
            let uuid_str = uuid.to_string();
            log::debug!("Generated UUID {uuid_str} for upload {normalized:?}");
            // TODO: flock the target file so that no two parallel uploads
            // could be created
            lock.insert(
                uuid,
                UploadInfo {
                    file: normalized,
                    size,
                    expires: Instant::now() + Duration::from_secs(24 * 60 * 60),
                },
            );
            return Ok(uuid_str);
        }
        anyhow::bail!("failed to allocate a UUID for this upload");
    } else {
        anyhow::bail!("the path specified already exists");
    }
}
