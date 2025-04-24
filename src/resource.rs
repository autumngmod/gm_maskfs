use rglua::prelude::*;

pub fn add_single_file(lua: LuaState, path: LuaString) {
  lua_getglobal(lua, cstr!("resource"));

  lua_getfield(lua, -1, cstr!("AddSingleFile"));
  lua_pushstring(lua, path);

  lua_call(lua, 1, 0);
}