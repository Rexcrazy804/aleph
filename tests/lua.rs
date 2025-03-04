use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string};

/// ``buckets``: ``HashMap<BucketName, BucketUrl>``
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    description: String,
    buckets: HashMap<String, String>,
    packages: Vec<String>,
}

impl LuaUserData for Config {}

#[test]
fn lua_test() {
    let lua_config = read_to_string("./tests/sample_data/config.lua").unwrap();
    let lua = Lua::new();
    let local_config: Config = lua
        .from_value(lua.load(lua_config).eval().unwrap())
        .unwrap();

    dbg!(&local_config);
}
