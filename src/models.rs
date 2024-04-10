// use uuid::Uuid;
// use crate::schema::users;

// #[derive(Debug,Queryable, Selectable)]
// #[diesel(table_name = crate::schema::users)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct User {
//     pub id: Uuid,
//     pub email: String,
//     pub name: String,
//     pub hashed_password: String,
// }

// #[derive(Debug, Insertable)]
// #[table_name = "users"]
// pub struct NewUser {
//     pub email: String,
//     pub name: String,
//     pub hashed_password: String,
// }
