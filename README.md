# NFS Manager

一个现代化的 NFS 自动挂载/卸载工具，使用 Tauri + Rust + SolidJS 构建。

## 特性

- 🚀 跨平台支持 (macOS, Linux, Windows)
- 📝 可视化配置管理
- 📊 实时挂载状态监控
- 🎨 现代化 UI 界面
- 💪 批量挂载/卸载操作
- ⚡ 体积小巧，性能优异

## 技术栈

- **后端**: Rust + Tauri
- **前端**: SolidJS + TypeScript
- **构建工具**: Vite + pnpm

## 开发

### 前置要求

- Rust (1.70+)
- Node.js (18+)
- pnpm

### 安装依赖

```bash
pnpm install
```

### 快速开始（推荐）

使用 `run.sh` 脚本：

```bash
# 启动开发模式
./run.sh dev

# 构建生产版本
./run.sh build

# 查看所有命令
./run.sh help
```

### 开发模式

```bash
# 使用 run.sh（推荐）
./run.sh dev

# 或直接使用 pnpm
pnpm tauri dev
```

### 构建

```bash
# 使用 run.sh（推荐）
./run.sh build

# 或直接使用 pnpm
pnpm tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`

### 其他命令

```bash
./run.sh test              # 运行测试
./run.sh check             # 代码检查
./run.sh format            # 格式化代码
./run.sh clean             # 清理构建产物
./run.sh build-frontend    # 仅构建前端
./run.sh build-backend     # 仅构建后端
```

## 使用说明

### 添加 NFS 配置

1. 点击 "添加配置" 按钮
2. 填写以下信息：
   - **名称**: 配置的唯一标识（如 `company`）
   - **服务器地址**: NFS 服务器路径（如 `192.168.1.100:/data`）
   - **挂载点**:
     - macOS/Linux: 目录名（如 `company-data`，实际挂载到 `~/nfs-mounts/company-data`）
     - Windows: 驱动器号（如 `Z`）
   - **挂载选项**: NFS 挂载参数（如 `rw,sync,hard,intr`）

### 挂载/卸载

- **单个挂载**: 点击配置卡片上的 "挂载" 按钮
- **单个卸载**: 点击配置卡片上的 "卸载" 按钮
- **批量挂载**: 点击底部的 "挂载全部" 按钮
- **批量卸载**: 点击底部的 "卸载全部" 按钮

**注意**：首次挂载时，macOS 会弹出权限确认对话框，请点击"确定"并输入密码。

### 配置文件

配置文件位于 `~/.nfs-manager.conf`，格式：

```
name|server:/path|mount_point|options
```

**挂载点说明**：
- 相对路径（推荐）：如 `company-data`，会自动挂载到 `~/nfs-mounts/company-data`
- 绝对路径：如 `/Users/username/my-mount`，直接使用该路径

示例：

```
# 使用相对路径（推荐）
company|192.168.1.100:/data|company-data|rw,sync,hard,intr
backup|192.168.1.200:/backup|backup|ro,sync

# 使用绝对路径
dev|dev.example.com:/projects|/Users/username/projects|rw,async
dev|dev.example.com:/projects|projects|rw,async
```

## 平台差异

### macOS / Linux

- 使用 `mount` 和 `umount` 命令
- 挂载点位于 `~/nfs-mounts/` 目录
- 需要 root 权限（某些系统）

### Windows

- 使用 `net use` 命令
- 挂载到驱动器号（如 `Z:`）
- 需要管理员权限

## 权限说明

NFS 挂载通常需要管理员/root 权限：

- **macOS**: 可能需要在"系统偏好设置 > 安全性与隐私"中授权
- **Linux**: 使用 `sudo` 运行或配置 `/etc/fstab`
- **Windows**: 以管理员身份运行

## 故障排查

### 挂载失败

1. 检查网络连接
2. 确认 NFS 服务器地址正确
3. 验证 NFS 服务器权限配置
4. 检查防火墙设置
5. 确认有足够的系统权限

### 卸载失败

- 确保没有程序正在使用挂载点
- 尝试关闭访问挂载点的文件管理器窗口
- 使用强制卸载（应用会自动尝试）

## 许可证

MIT License

## 从 Shell 脚本迁移

如果你之前使用 `nfs-manager.sh`，配置文件格式完全兼容，可以直接使用现有的 `~/.nfs-manager.conf` 文件。
