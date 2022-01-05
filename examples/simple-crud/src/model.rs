use crate::schema::*;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}
