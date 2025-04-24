mod data;
mod resource;

use anyhow::Result;
use rglua::prelude::*;

#[gmod_open]
fn gmod_open(lua: LuaState) -> Result<i32> {
  data::scan(lua)?;

  Ok(0)
}

#[gmod_close]
fn gmod_close(_lua: LuaState) -> i32 {
  0
}