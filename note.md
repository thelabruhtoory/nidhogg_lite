### Building on Windows

```powershell

powershell
$CARGO_TARGET_DIR="$HOME\Documents\"
cargo build
Remove-Item "$HOME\Documents\debug" -Recurse
```

### Building on Linux

```bash

bash
export CARGO_TARGET_DIR="${HOME}/Documents/"
cargo build
rm -rf ${HOME}/Documents/debug/
```