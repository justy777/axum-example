use crate::{
    models::{NewTag, Tag},
    Pool,
};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use diesel::RunQueryDsl;

pub(crate) async fn root() -> &'static str {
    "Hello, world!"
}

pub(crate) async fn list_tags(State(pool): State<Pool>) -> Json<Vec<Tag>> {
    let mut conn = pool.get().unwrap();

    let payload = Tag::all().load(&mut conn).expect("Unable to load tags");
    Json(payload)
}

pub(crate) async fn get_tag(Path(id): Path<i32>, State(pool): State<Pool>) -> Json<Tag> {
    let mut conn = pool.get().unwrap();

    let tag = Tag::by_id(id).get_result(&mut conn).unwrap();

    Json(tag)
}

pub(crate) async fn create_tag(
    State(pool): State<Pool>,
    Json(new_tag): Json<NewTag>,
) -> (StatusCode, Json<Tag>) {
    let mut conn = pool.get().unwrap();

    let result = new_tag.insert().get_result(&mut conn).unwrap();
    (StatusCode::CREATED, Json(result))
}

pub(crate) async fn delete_tag(Path(id): Path<i32>, State(pool): State<Pool>) -> StatusCode {
    let mut conn = pool.get().unwrap();

    let rows_affected = Tag::delete_by_id(id).execute(&mut conn).unwrap();

    if rows_affected == 1 {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
