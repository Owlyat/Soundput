### Config
Windows : `%Userprofile%/.config/Soundput/config.toml` (Copy Soundput folder to this directory)

`break = true` (default) cuts off the currently playing key press audio when the next sound starts.
`break = false` keeps the press audio playing and layers the release audio on top of it.

## Installation
Build from cargo.
`Cargo build --release`
Put the binary in your PATH.

### Windows
if you want it to start on startup, add this script to `%AppData%/Microsoft/Windows/Start Menu/Programs/Startup/` and modify `path/to/Soundput` to your Soundput binary path:
```Soundput.vbs
Dim WinScriptHost
Set WinScriptHost = CreateObject("WScript.Shell")
WinScriptHost.Run Chr(34) & "path/to/Soundput.exe" & Chr(34), 0
Set WinScriptHost = Nothing
```
