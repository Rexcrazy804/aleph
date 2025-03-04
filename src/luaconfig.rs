use mlua::UserData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// ``buckets``: ``HashMap<BucketName, BucketUrl>``
#[derive(Serialize, Deserialize, Debug)]
pub struct LuaConfig {
    pub description: String,
    pub buckets: HashMap<String, String>,
    pub packages: Vec<String>,
}

impl UserData for LuaConfig {}
