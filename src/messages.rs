use actix::Message;
use diesel::QueryResult;
use serde::Serialize;
use crate:: types::User;


#[derive(Message,Serialize)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchAllUsers;
