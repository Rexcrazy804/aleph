use aleph::luaconfig::LuaConfig;
use mlua::prelude::*;
use std::fs::read_to_string;

#[test]
fn lua_test() {
    let lua_config = read_to_string("./tests/sample_data/config.lua").unwrap();
    let lua = Lua::new();
    let local_config: LuaConfig = lua
        .from_value(lua.load(lua_config).eval().unwrap())
        .unwrap();

    dbg!(&local_config);
}
