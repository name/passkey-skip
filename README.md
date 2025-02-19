# Passkey Skip

A simple Windows utility that automatically handles Windows Security prompts by selecting the security key option, making the FIDO2 authentication process smoother.

## Features

- Runs silently in the system tray
- Automatically detects Windows Security prompts
- Automatically selects the security key option
- Starts automatically with Windows (optional)
- Minimal resource usage

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

### Command Prompt Installation

```bash
mkdir "C:\Program Files\PasskeySkip"
cd /d "C:\Program Files\PasskeySkip"
curl -L -o passkey-skip.exe "https://github.com/name/passkey-skip/releases/download/0.1.0/passkey-skip.exe/passkey-skip.exe"
```

## Setting Up Auto-Start

### Manual Setup

1. Press `Win + R` to open Run
2. Type `shell:startup` and press Enter
3. Create a shortcut to `passkey-skip.exe` in this folder:
   - Right-click in the Startup folder
   - Select "New" → "Shortcut"
   - Browse to `C:\Program Files\PasskeySkip\passkey-skip.exe`
   - Click "Next" and "Finish"

### PowerShell Setup

```powershell
$WshShell = New-Object -comObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\PasskeySkip.lnk")
$Shortcut.TargetPath = "C:\Program Files\PasskeySkip\passkey-skip.exe"
$Shortcut.Save()
```

### Command Prompt Setup

```bash
powershell -Command "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\PasskeySkip.lnk'); $Shortcut.TargetPath = 'C:\Program Files\PasskeySkip\passkey-skip.exe'; $Shortcut.Save()"
```

## Usage

- The app runs in the system tray (look for the icon near the clock)
- Right-click the tray icon to see options:
  - Quit: Closes the application

## How It Works

When a Windows Security prompt appears:

1. The app detects the window
2. Automatically selects the security key option
3. Confirms the selection

This eliminates the need to manually select the security key option each time.

## Requirements

- Windows 10 or later
- A FIDO2 security key

## Building from Source

1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Clone this repository
3. Run `cargo build --release`
4. The executable will be in `target/release/passkey-skip.exe`

## License

[MIT License](LICENSE)
