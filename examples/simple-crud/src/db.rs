use crate::model::*;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::MysqlConnection;
use transaction::prelude::*;
use transaction_diesel_mysql::with_conn;
use transaction_diesel_mysql::DieselContext;

type Ctx<'a> = DieselContext<'a, MysqlConnection>;
// Until Rust supports `impl Trait`, we need to box `Transaction`s when returning from functions.
type BoxTx<'a, T> = Box<dyn Transaction<Ctx = Ctx<'a>, Item = T, Err = Error> + 'a>;

pub fn create_user<'a>(name: &'a str) -> BoxTx<'a, User> {
    use crate::schema::users::table;
    // Connections are injected via transaction.
    // Get it using `with_conn`
    with_conn(move |cn| {
        diesel::insert_into(table)
            .values(&NewUser { name: name })
            .execute(cn)?;
        table
            .order(crate::schema::users::id.desc())
            .limit(1)
            .first(cn)
    })
    // box it
    .boxed()
}

pub fn find_user<'a>(id: i32) -> BoxTx<'a, Option<User>> {
    use crate::schema::users::dsl::users;
    with_conn(move |cn| users.find(id).get_result(cn).optional()).boxed()
}

pub fn update_user<'a>(id: i32, name: &'a str) -> BoxTx<'a, Option<()>> {
    use crate::schema::users::dsl;
    with_conn(move |cn| {
        diesel::update(dsl::users.find(id))
            .set(dsl::name.eq(name))
            .execute(cn)
            .map(|_| ())
            .optional()
    })
    .boxed()
}

pub fn delete_user<'a>(id: i32) -> BoxTx<'a, Option<()>> {
    use crate::schema::users::dsl::users;
    with_conn(move |cn| {
        diesel::delete(users.find(id))
            .execute(cn)
            .map(|_| ())
            .optional()
    })
    .boxed()
}
