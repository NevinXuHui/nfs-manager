# NFS Manager 项目结构

```
nfs-manager/
├── src/                      # 前端源码 (SolidJS)
│   ├── App.tsx              # 主应用组件
│   ├── App.css              # 样式文件
│   ├── api.ts               # Tauri API 调用封装
│   └── main.tsx             # 入口文件
│
├── src-tauri/               # Rust 后端
│   ├── src/
│   │   ├── main.rs          # 程序入口
│   │   ├── lib.rs           # Tauri 命令处理
│   │   ├── nfs.rs           # NFS 核心逻辑
│   │   └── config.rs        # 配置管理
│   ├── Cargo.toml           # Rust 依赖配置
│   └── tauri.conf.json      # Tauri 配置
│
├── public/                  # 静态资源
├── .backup/                 # 原 shell 脚本备份
├── README.md                # 项目说明
├── dev.sh                   # 开发启动脚本
└── package.json             # 前端依赖配置
```

## 核心模块说明

### 前端 (src/)

- **App.tsx**: 主界面，包含配置列表、添加对话框、挂载/卸载操作
- **api.ts**: 封装所有 Tauri 命令调用，提供类型安全的 API
- **App.css**: 响应式样式，支持亮色/暗色主题

### 后端 (src-tauri/src/)

- **nfs.rs**:
  - `Platform` 枚举：识别当前操作系统
  - `NfsConfig` 结构：NFS 配置数据模型
  - `MountStatus` 结构：挂载状态
  - 跨平台挂载/卸载实现

- **config.rs**:
  - `ConfigManager`: 配置文件读写管理
  - 配置增删改查操作

- **lib.rs**:
  - Tauri 命令定义
  - 应用状态管理
  - 前后端通信桥梁

## 数据流

```
用户操作 (UI)
    ↓
SolidJS 组件 (App.tsx)
    ↓
API 调用 (api.ts)
    ↓
Tauri IPC
    ↓
Rust 命令处理 (lib.rs)
    ↓
业务逻辑 (nfs.rs, config.rs)
    ↓
系统调用 (mount/umount)
```

## 配置文件

位置: `~/.nfs-manager.conf`

格式: `name|server:/path|mount_point|options`

示例:
```
company|192.168.1.100:/data|company-data|rw,sync,hard,intr
```

## 开发命令

```bash
# 安装依赖
pnpm install

# 开发模式（热重载）
pnpm tauri dev
# 或
./dev.sh

# 构建生产版本
pnpm tauri build

# 仅构建前端
pnpm build

# 仅构建后端
cd src-tauri && cargo build --release
```

## 构建产物

执行 `pnpm tauri build` 后，产物位于：

- **macOS**: `src-tauri/target/release/bundle/macos/NFS Manager.app`
- **Linux**: `src-tauri/target/release/bundle/deb/` 或 `appimage/`
- **Windows**: `src-tauri/target/release/bundle/msi/` 或 `nsis/`

## 体积优化

当前配置已经很小：

- SolidJS: ~7KB (gzipped)
- Tauri 运行时: ~600KB
- 总安装包: ~3-5MB (取决于平台)

进一步优化：

1. 生产构建时自动启用 `strip` 和 `lto`
2. 前端使用 Vite 的代码分割
3. 图标资源已压缩

## 跨平台注意事项

### macOS
- 需要在"系统偏好设置"中授权网络访问
- 可能需要 `sudo` 权限

### Linux
- 确保安装了 `nfs-common` 包
- 某些发行版需要配置 `/etc/fstab`

### Windows
- 需要启用 "NFS 客户端" 功能
- 必须以管理员身份运行
- 使用驱动器号而非路径
