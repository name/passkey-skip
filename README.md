# Passkey Skip

A simple Windows utility that automatically handles Windows Security prompts by selecting the security key option, making the FIDO2 authentication process smoother.

## Features

- Runs silently in the system tray
- Automatically detects Windows Security prompts
- Automatically selects the security key option
- Starts automatically with Windows (optional)
- Minimal resource usage
- Optional PIN skip mode for different authentication flows

### Standard Mode

![Standard Mode Demo](https://i.imgur.com/wY5a9xi.gif)
*Standard mode automatically selects the security key option*

### PIN Skip Mode

![PIN Skip Mode Demo](https://i.imgur.com/5E5tI3z.gif)
*PIN skip mode bypasses the PIN entry step*

## Installation

### Manual Installation

1. Download the latest release from the [Releases page](https://github.com/name/passkey-skip/releases)
2. Create a folder at `C:\Program Files\PasskeySkip`
3. Extract `passkey-skip.exe` into this folder

### PowerShell Installation

```powershell
# Create directory and download files
New-Item -ItemType Directory -Path "C:\Program Files\PasskeySkip" -Force
Set-Location -Path "C:\Program Files\PasskeySkip"
Invoke-WebRequest -Uri "https://github.com/name/passkey-skip/releases/download/0.1.0/passkey-skip.exe" -OutFile "passkey-skip.exe"
```

### Auto-start Setup

#### Standard Mode

```powershell
# Create auto-start entry
$WshShell = New-Object -comObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\PasskeySkip.lnk")
$Shortcut.TargetPath = "C:\Program Files\PasskeySkip\passkey-skip.exe"
$Shortcut.Save()
```

#### PIN Skip Mode

```powershell
# Create auto-start entry with PIN skip option
$WshShell = New-Object -comObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\PasskeySkip.lnk")
$Shortcut.TargetPath = "C:\Program Files\PasskeySkip\passkey-skip.exe"
$Shortcut.Arguments = "--skip-pin"
$Shortcut.Save()
```

### Command Line Options

- `--skip-pin`: Run in PIN skip mode, which uses a different key sequence for security key selection. Use this if you want to bypass PIN entry in the authentication flow.

## How It Works

When a Windows Security prompt appears:

1. The app detects the window
2. Automatically selects the security key option
3. In PIN skip mode, performs additional key presses to bypass PIN entry
4. Confirms the selection

This eliminates the need to manually select the security key option each time.

## Requirements

- Windows 10 or later
- A FIDO2 security key

## Building from Source

1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Clone this repository
3. Run `cargo build --release`
4. The executable will be in `target/release/passkey-skip.exe`

To run in PIN skip mode after building:

```bash
./passkey-skip.exe --skip-pin
```

## License

[MIT License](LICENSE)
