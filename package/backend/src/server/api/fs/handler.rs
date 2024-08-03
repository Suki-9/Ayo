use crate::modules::{
  auth::UserCredential, database::postgres::DB_POOL, file_system::{File, Group},
};
use axum::{
  body::Body,
  extract,
  extract::Path,
  http::{header, StatusCode},
  response::{AppendHeaders, IntoResponse, Json},
};
use serde::Deserialize;
use serde_json::json;
use tokio_util::io::ReaderStream;

#[derive(Deserialize)]
pub struct CreateReq {
  pub parent_id: Option<String>,
  pub name: Option<String>,
  pub summary: Option<String>,

  pub mime_type: Option<String>,
  pub data: Option<Vec<u8>>,
}

#[derive(Deserialize)]
pub struct UpdateReq {
  summary: Option<String>,
  name: Option<String>,
  chmod: Option<[u8; 3]>,
}

#[derive(Deserialize)]
pub struct CommonPathParams {
  id: String,
}
pub type ReadPathParams = CommonPathParams;
pub type UpdatePathParams = CommonPathParams;
pub type DeletePathParams = CommonPathParams;
pub type InfoPathParams = CommonPathParams;
pub type ThumbnailPathParams = CommonPathParams;

pub struct FsHandlers {}
impl FsHandlers {
  pub async fn create(
    credential: UserCredential,
    extract::Json(payload): extract::Json<CreateReq>,
  ) -> impl IntoResponse {
    match (payload.data, payload.mime_type) {
      (Some(data), Some(mime_type)) => {
        let Ok(file) = File::new(
          &credential.user_id,
          payload.name.as_deref(),
          payload.summary.as_deref(),
          &mime_type,
          None,
          payload.parent_id.as_deref(),
          data,
          None,
        )
        .await
        else {
          return Err(
            (
              StatusCode::INTERNAL_SERVER_ERROR,
              Json(json!({
                "Err": "File creation failed.",
                "hint": "Wait a while and try again.",
              })),
            )
              .into_response(),
          );
        };

        Ok((StatusCode::OK, Json(json!(file))).into_response())
      }
      (None, None) => {
        let Ok(group) = Group::new(
          &credential.user_id,
          payload.name.as_deref(),
          payload.summary.as_deref(),
          None,
          payload.parent_id.as_deref(),
          None,
        )
        .await
        else {
          return Err(
            (
              StatusCode::INTERNAL_SERVER_ERROR,
              Json(json!({
                "Err": "Directory creation failed.",
                "hint": "Wait a while and try again.",
              })),
            )
              .into_response(),
          );
        };
        Ok((StatusCode::OK, Json(json!(group))).into_response())
      }
      _ => Err(StatusCode::BAD_REQUEST.into_response()),
    }
  }

  pub async fn read(
    credential: UserCredential,
    Path(params): Path<ReadPathParams>,
  ) -> impl IntoResponse {
    match params.id.chars().next() {
      Some('F') => {
        let file = match File::read(&params.id).await {
          Ok(file) if file.owner_id == credential.user_id => file,
          Ok(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
          _ => return Err(StatusCode::UNAUTHORIZED.into_response()),
        };

        let stream_body = match tokio::fs::File::open(&file.obj_path).await {
          Ok(file) => ReaderStream::new(file),
          _ => return Err(StatusCode::NOT_FOUND.into_response()),
        };

        Ok(
          (
            StatusCode::OK,
            AppendHeaders([
              (header::CONTENT_TYPE, file.mime_type),
              (
                header::CONTENT_DISPOSITION,
                match file.summary {
                  Some(v) => v,
                  None => "".to_string(),
                },
              ),
            ]),
            Body::from_stream(stream_body),
          )
            .into_response(),
        )
      }
      Some('D') => {
        let Ok(dir) = Group::read(&params.id).await else {
          return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        };

        let mut dir = json!(dir);

        match Group::get_child(&params.id).await {
          Ok(child) => dir
            .as_object_mut()
            .unwrap()
            .insert("child".to_string(), json!(child)),
          Err(e) => {
            println!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
          }
        };

        Ok((StatusCode::OK, Json(dir)).into_response())
      }
      _ => Err(StatusCode::BAD_REQUEST.into_response()),
    }
  }

  pub async fn update(
    credential: UserCredential,
    Path(params): Path<UpdatePathParams>,
    extract::Json(payload): extract::Json<UpdateReq>,
  ) -> impl IntoResponse {
    let Ok(mut tx) = DB_POOL.begin().await else {
      return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    match params.id.chars().next() {
      Some('F') => {
        let Ok(file) = File::read(&params.id).await else {
          return StatusCode::NOT_FOUND.into_response();
        };

        if match payload.name {
          Some(name) => File::rename(&params.id, &name, Some(&mut tx))
            .await
            .is_err(),
          _ => false,
        } {
          tx.rollback().await.unwrap();
          return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        if match payload.summary {
          Some(summary) => File::update_summary(&params.id, &summary, Some(&mut tx))
            .await
            .is_err(),
          _ => false,
        } {
          tx.rollback().await.unwrap();
          return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        if match payload.chmod {
          Some(chmod) => File::chmod(&params.id, &chmod, Some(&mut tx))
            .await
            .is_err(),
          _ => false,
        } {
          tx.rollback().await.unwrap();
          return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        if file.owner_id == credential.user_id {
          match (tx.commit().await, File::read(&params.id).await) {
            (Ok(_), Ok(file)) => (StatusCode::OK, Json(json!(file))).into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
          }
        } else {
          tx.rollback().await.unwrap();
          StatusCode::UNAUTHORIZED.into_response()
        }
      }
      Some('D') => {
        let Ok(dir) = Group::read(&params.id).await else {
          return StatusCode::NOT_FOUND.into_response();
        };

        if match payload.name {
          Some(name) => Group::rename(&params.id, &name, Some(&mut tx))
            .await
            .is_err(),
          _ => false,
        } {
          tx.rollback().await.unwrap();
          return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        if match payload.summary {
          Some(summary) => Group::update_summary(&params.id, &summary, Some(&mut tx))
            .await
            .is_err(),
          _ => false,
        } {
          tx.rollback().await.unwrap();
          return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        if match payload.chmod {
          Some(chmod) => Group::chmod(&params.id, &chmod, Some(&mut tx))
            .await
            .is_err(),
          _ => false,
        } {
          tx.rollback().await.unwrap();
          return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        if dir.owner_id == credential.user_id {
          match (tx.commit().await, File::read(&params.id).await) {
            (Ok(_), Ok(file)) => (StatusCode::OK, Json(json!(file))).into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
          }
        } else {
          tx.rollback().await.unwrap();
          StatusCode::UNAUTHORIZED.into_response()
        }
      }
      _ => StatusCode::BAD_REQUEST.into_response(),
    }
  }

  pub async fn delete(
    credential: UserCredential,
    Path(params): Path<DeletePathParams>,
  ) -> impl IntoResponse {
    match match params.id.chars().next() {
      Some('F') => File::delete(&params.id, None).await,
      Some('D') => Group::delete(&params.id, None).await,
      _ => Err(anyhow::anyhow!("")),
    } {
      Ok(()) => StatusCode::OK,
      Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
    .into_response()
  }

  pub async fn info(
    credential: UserCredential,
    Path(params): Path<InfoPathParams>,
  ) -> impl IntoResponse {
    match params.id.chars().next() {
      Some('F') => {
        let Ok(file) = File::read(&params.id).await else {
          return (StatusCode::NOT_FOUND, Json(json!({}))).into_response();
        };

        if file.owner_id != credential.user_id {
          return StatusCode::UNAUTHORIZED.into_response();
        }

        (StatusCode::OK, Json(json!(file))).into_response()
      }
      Some('D') => {
        let Ok(dir) = Group::read(&params.id).await else {
          return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        if dir.owner_id != credential.user_id {
          return StatusCode::UNAUTHORIZED.into_response();
        }

        let mut dir = json!(dir);

        match Group::get_child(&params.id).await {
          Ok(child) => dir
            .as_object_mut()
            .unwrap()
            .insert("child".to_string(), json!(child)),
          _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        (StatusCode::OK, Json(dir)).into_response().into_response()
      }
      _ => (StatusCode::NOT_FOUND, Json(json!({}))).into_response(),
    }
  }

  pub async fn thumbnail(
    credential: UserCredential,
    Path(params): Path<ThumbnailPathParams>,
  ) -> impl IntoResponse {
    let file = match File::read(&params.id).await {
      Ok(file) => file,
      Err(e) => {
        println!("{:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    };

    if file.owner_id != credential.user_id {
      return StatusCode::UNAUTHORIZED.into_response();
    }

    let stream_body = match tokio::fs::File::open(&file.obj_path).await {
      Ok(file) => ReaderStream::new(file),
      _ => return StatusCode::NOT_FOUND.into_response(),
    };

    (
      StatusCode::OK,
      AppendHeaders([(header::CONTENT_TYPE, file.mime_type)]),
      Body::from_stream(stream_body),
    )
      .into_response()
  }
}
