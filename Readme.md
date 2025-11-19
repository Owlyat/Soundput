### Config
Windows : `%Userprofile%/.config/Soundput/config.toml` (Copy Soundput folder to this directory)

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
