# omni-mcp-app
omni ai mcp server dev  desktop app

## Tauri App
pnpm create omni-mcp-app

### Template created! To get started run:
  cd omni-mcp-app
  pnpm install
  pnpm tauri android init
  pnpm tauri ios init

### For Desktop development, run:
  pnpm tauri dev

### For Android development, run:
  pnpm tauri android dev

### For iOS development, run:
  pnpm tauri ios dev

### Force fix with uncommitted changes :
```bash
cargo fix --lib -p omni-mcp-app --allow-dirty
 ```
### delete model_config.db
rm ~/Library/Application\ Support/omni-mcp-app/model_config.db