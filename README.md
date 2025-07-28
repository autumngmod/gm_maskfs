# gm_maskfs — Bypass GMod File Extension Limits

**MaskFS** is a binary module for Garry's Mod that bypasses the game's restriction on sending certain file extensions from server to client. With it, you can now send *any* file type to clients — no more being limited by GMod's whitelist.

## 🔧 Installation

1. Download the appropriate `.dll` file for your server's architecture from the table below.
2. Place it in ``garrysmod/lua/bin``
3. Download [`maskfs.lua`](https://raw.githubusercontent.com/autumngmod/gm_maskfs/master/maskfs.lua) and place it in ``garrysmod/lua/autorun/server``


| Architecture | Download |
|--------------|----------|
| Windows x32  | [Download](https://github.com/autumngmod/gm_maskfs/releases/latest/download/gmsv_maskfs_win32.dll) |
| Windows x64  | [Download](https://github.com/autumngmod/gm_maskfs/releases/latest/download/gmsv_maskfs_win64.dll) |
| Linux x32    | [Download](https://github.com/autumngmod/gm_maskfs/releases/latest/download/gmsv_maskfs_linux.dll) |
| Linux x64    | [Download](https://github.com/autumngmod/gm_maskfs/releases/latest/download/gmsv_maskfs_linux64.dll) |

## 🚀 Usage

1. Install the module as described above.
2. Drop any files or folders you want to send to clients into ``garrysmod/shared``

3. Restart the server.

After that, clients will receive those files automatically on connect, but with a `.dat` extension.

## ✅ Pros

- **Fast**: Files are downloaded during client connection like regular content.
- **Flexible**: Send any file extension you want.

## ⚠️ Minor Drawbacks

- Files are saved on the client with an added `.dat` extension.
- Files may be duplicated.
