use crate::{
    error::Result,
    models::{NewTag, Tag},
    Pool,
};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use diesel::RunQueryDsl;

pub async fn list_tags(State(pool): State<Pool>) -> Result<Json<Vec<Tag>>> {
    let mut conn = pool.get()?;

    let payload = Tag::all().load(&mut conn).expect("Unable to load tags");
    Ok(Json(payload))
}

pub async fn get_tag(Path(id): Path<i32>, State(pool): State<Pool>) -> Result<impl IntoResponse> {
    let mut conn = pool.get()?;

    let query_result = Tag::by_id(id).get_result(&mut conn);

    if matches!(query_result, Err(diesel::NotFound)) {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    let tag = query_result?;

    Ok(Json(tag).into_response())
}

pub async fn create_tag(
    State(pool): State<Pool>,
    Json(new_tag): Json<NewTag>,
) -> Result<impl IntoResponse> {
    let mut conn = pool.get()?;

    let tag_exists = Tag::by_label(&new_tag.label).first(&mut conn).is_ok();

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
