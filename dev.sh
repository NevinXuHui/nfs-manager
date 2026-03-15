#!/bin/bash
# NFS Manager 开发启动脚本

echo "🚀 启动 NFS Manager 开发环境..."

# 检查依赖
if ! command -v pnpm &> /dev/null; then
    echo "❌ pnpm 未安装，请先安装 pnpm"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust 未安装，请先安装 Rust"
    exit 1
fi

# 安装依赖（如果需要）
if [ ! -d "node_modules" ]; then
    echo "📦 安装前端依赖..."
    pnpm install
fi

# 启动开发服务器
echo "✨ 启动 Tauri 开发服务器..."
pnpm tauri dev
