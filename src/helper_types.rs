use diesel::{
    associations::HasTable,
    query_builder::{DeleteStatement, InsertStatement, IntoUpdateTarget},
    Insertable,
};

/// Represents the return type of [`diesel::insert_into().values()`](diesel::insert_into)
pub type Insert<Target, Values> =
    InsertStatement<<Target as HasTable>::Table, <Values as Insertable<Target>>::Values>;

/// Represents the return type of [`diesel::delete()`](diesel::delete)
pub type Delete<Target> =
    DeleteStatement<<Target as HasTable>::Table, <Target as IntoUpdateTarget>::WhereClause>;
