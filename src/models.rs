use diesel::{Insertable, QueryDsl, Queryable, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};

use crate::schema::tags;

type All = diesel::dsl::Select<tags::table, diesel::dsl::AsSelect<Tag, diesel::sqlite::Sqlite>>;
type ById = diesel::dsl::Find<All, i32>;
type DeleteById = crate::helper_types::Delete<diesel::dsl::Find<tags::table, i32>>;
type Insert = crate::helper_types::Insert<tags::table, NewTag>;

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Tag {
    id: i32,
    label: String,
}

impl Tag {
    pub(crate) fn all() -> All {
        tags::table.select(Self::as_select())
    }

    pub(crate) fn by_id(id: i32) -> ById {
        Self::all().find(id)
    }

    pub(crate) fn delete_by_id(id: i32) -> DeleteById {
        diesel::delete(tags::table.find(id))
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct NewTag {
    label: String,
}

impl NewTag {
    pub(crate) fn insert(self) -> Insert {
        diesel::insert_into(tags::table).values(self)
    }
}
