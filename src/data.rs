use std::{
  ffi::CString,
  fs,
  path::{Path, PathBuf}
};
use anyhow::{anyhow, Context, Result};
use rglua::prelude::*;
use crate::resource;

fn append_dat_extension(path: &Path) -> PathBuf {
  if path.extension().is_some() {
    let mut new_name = path.file_name().unwrap().to_os_string();
    new_name.push(".dat");
    path.with_file_name(new_name)
  } else {
    path.with_extension("dat")
  }
}

fn strip_garrysmod_prefix(path: &Path) -> PathBuf {
  path.strip_prefix("garrysmod/")
    .or_else(|_| path.strip_prefix("garrysmod"))
    .unwrap_or(path)
    .to_path_buf()
}

fn add_file(lua: &LuaState, entry: &Path, save_path: &Path) -> Result<()> {
  if entry.is_dir() {
    fs::create_dir_all(save_path)
      .with_context(|| format!("Failed to create directory {:?}", save_path))?;

    for item in fs::read_dir(entry)
      .with_context(|| format!("Failed to read directory {:?}", entry))?
    {
      let item = item?;
      let src_path = item.path();
      let dst_path = save_path.join(item.file_name());
      add_file(lua, &src_path, &dst_path)?;
    }
  } else if entry.is_file() {
    if let Some(parent) = save_path.parent() {
      fs::create_dir_all(parent)
        .with_context(|| format!("Failed to create parent directory {:?}", parent))?;
    }

    let dst_with_dat = append_dat_extension(save_path);
    fs::copy(entry, &dst_with_dat)
      .with_context(|| format!("Failed to copy {:?} to {:?}", entry, dst_with_dat))?;

    let clean_path = strip_garrysmod_prefix(&dst_with_dat);

    let path_str = clean_path.to_str()
      .ok_or_else(|| anyhow!("Failed to convert path to string: {:?}", clean_path))?
      .replace("\\", "/");

    let c_path = CString::new(path_str)?;
    resource::add_single_file(*lua, c_path.as_ptr());
  }

  Ok(())
}

pub fn scan(lua: LuaState) -> Result<()> {
  let base_dir = Path::new("garrysmod/shared");
  let save_dir = Path::new("garrysmod/resource/shared");

  if !base_dir.exists() {
    fs::create_dir_all(base_dir)
      .with_context(|| format!("Unable to create base directory: {:?}", base_dir))?;
    return Ok(());
  }

  for entry in fs::read_dir(base_dir)
    .with_context(|| format!("Failed to read base directory {:?}", base_dir))?
  {
    let entry = entry?;
    let path = entry.path();
    let rel_path = path.strip_prefix(base_dir)
      .with_context(|| format!("Failed to get relative path for {:?}", path))?;
    let target_path = save_dir.join(rel_path);
    add_file(&lua, &path, &target_path)?;
  }

  Ok(())
}
