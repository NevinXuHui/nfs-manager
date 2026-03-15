# NFS Manager - 快速参考

## 一键启动

```bash
./dev.sh
```

## 常用命令

```bash
# 开发模式（热重载）
pnpm tauri dev

# 构建生产版本
pnpm tauri build

# 仅编译 Rust
cargo build --manifest-path=src-tauri/Cargo.toml

# 仅构建前端
pnpm build
```

## 配置文件位置

```
~/.nfs-manager.conf
```

## 配置格式

```
名称|服务器地址|挂载点|选项
```

示例：
```
company|192.168.1.100:/data|company-data|rw,sync,hard,intr
backup|192.168.1.200:/backup|backup|ro,sync
```

## 平台差异

| 平台 | 挂载命令 | 挂载点格式 | 权限要求 |
|------|---------|-----------|---------|
| macOS | `mount -t nfs` | `~/nfs-mounts/xxx` | 可能需要 sudo |
| Linux | `mount -t nfs` | `~/nfs-mounts/xxx` | 可能需要 sudo |
| Windows | `net use` | `Z:` (驱动器号) | 需要管理员 |

## 常见挂载选项

| 选项 | 说明 |
|------|------|
| `rw` | 读写模式 |
| `ro` | 只读模式 |
| `sync` | 同步写入 |
| `async` | 异步写入 |
| `hard` | 硬挂载（推荐） |
| `soft` | 软挂载 |
| `intr` | 允许中断 |
| `noexec` | 禁止执行 |
| `nosuid` | 禁止 suid |

## 故障排查

### 挂载失败
1. 检查网络连接：`ping <服务器IP>`
2. 检查 NFS 服务：`showmount -e <服务器IP>`
3. 检查权限：确保有 sudo/管理员权限
4. 查看日志：应用会显示详细错误信息

### 卸载失败
1. 关闭所有使用挂载点的程序
2. 关闭文件管理器窗口
3. 应用会自动尝试强制卸载

## 开发调试

### 查看 Rust 日志
```bash
RUST_LOG=debug pnpm tauri dev
```

### 查看前端控制台
在应用中按 `Cmd+Option+I` (macOS) 或 `F12` (Windows/Linux)

### 重新编译
```bash
# 清理构建缓存
rm -rf src-tauri/target
cargo clean --manifest-path=src-tauri/Cargo.toml

# 重新构建
pnpm tauri build
```

## 项目结构速查

```
src/App.tsx          # 主界面
src/api.ts           # API 封装
src-tauri/src/nfs.rs # NFS 核心逻辑
src-tauri/src/config.rs # 配置管理
src-tauri/src/lib.rs # Tauri 命令
```

## 构建产物位置

```
src-tauri/target/release/bundle/
├── macos/           # macOS .app
├── deb/             # Linux .deb
├── appimage/        # Linux AppImage
├── msi/             # Windows .msi
└── nsis/            # Windows NSIS 安装包
```

## 性能指标

- 启动时间: ~1-2 秒
- 内存占用: ~30-50 MB
- 安装包大小: ~3-5 MB
- 前端包大小: ~50 KB (gzipped)

## 技术栈版本

- Tauri: 2.x
- Rust: 1.70+
- SolidJS: 1.9+
- TypeScript: 5.x
- Vite: 6.x
