use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::user_schema::User;

pub type UserDb = Arc<Mutex<HashMap<u32, User>>>;
