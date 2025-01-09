use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::{OptionalExtension, RunQueryDsl};

use crate::{
    error::Result,
    models::{NewTag, Tag},
    Pool,
};

pub async fn list_tags(State(pool): State<Pool>) -> Result<Json<Vec<Tag>>> {
    let mut conn = pool.get()?;

    let payload = Tag::all().load(&mut conn)?;
    Ok(Json(payload))
}

pub async fn get_tag(Path(id): Path<i32>, State(pool): State<Pool>) -> Result<impl IntoResponse> {
    let mut conn = pool.get()?;

    let tag_option = Tag::by_id(id).get_result(&mut conn).optional()?;

    match tag_option {
        Some(tag) => Ok(Json(tag).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

pub async fn create_tag(
    State(pool): State<Pool>,
    Json(new_tag): Json<NewTag>,
) -> Result<impl IntoResponse> {
    let mut conn = pool.get()?;

    let tag_exists = Tag::by_label(&new_tag.label)
        .first(&mut conn)
        .optional()?
        .is_some();

    if tag_exists {
        return Ok(StatusCode::CONFLICT.into_response());
    }

    let tag: Tag = new_tag.insert().get_result(&mut conn)?;
    Ok((StatusCode::CREATED, Json(tag)).into_response())
}

pub async fn delete_tag(Path(id): Path<i32>, State(pool): State<Pool>) -> Result<StatusCode> {
    let mut conn = pool.get()?;

    let rows_affected = Tag::delete_by_id(id).execute(&mut conn)?;

    if rows_affected == 1 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
