#!/bin/bash
# NFS Manager 运行脚本

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# 检查依赖
check_dependencies() {
    print_info "检查依赖..."

    if ! command -v pnpm &> /dev/null; then
        print_error "pnpm 未安装"
        echo "请运行: npm install -g pnpm"
        exit 1
    fi

    if ! command -v cargo &> /dev/null; then
        print_error "Rust 未安装"
        echo "请访问: https://rustup.rs/"
        exit 1
    fi

    print_success "依赖检查通过"
}

# 安装依赖
install_deps() {
    if [ ! -d "node_modules" ]; then
        print_info "安装前端依赖..."
        pnpm install
        print_success "前端依赖安装完成"
    else
        print_info "前端依赖已安装"
    fi
}

# 清理构建产物
clean() {
    print_info "清理构建产物..."
    rm -rf dist/
    rm -rf src-tauri/target/
    print_success "清理完成"
}

# 开发模式
dev() {
    print_info "启动开发模式..."
    print_warning "按 Ctrl+C 停止"
    echo ""
    pnpm tauri dev
}

# 构建生产版本
build() {
    print_info "构建生产版本..."
    pnpm tauri build
    print_success "构建完成"
    echo ""
    print_info "构建产物位置:"
    echo "  macOS: src-tauri/target/release/bundle/macos/"
    echo "  Linux: src-tauri/target/release/bundle/deb/ 或 appimage/"
    echo "  Windows: src-tauri/target/release/bundle/msi/ 或 nsis/"
}

# 仅构建前端
build_frontend() {
    print_info "构建前端..."
    pnpm build
    print_success "前端构建完成: dist/"
}

# 仅构建后端
build_backend() {
    print_info "构建后端..."
    cd src-tauri
    cargo build --release
    cd ..
    print_success "后端构建完成: src-tauri/target/release/"
}

# 运行测试
test() {
    print_info "运行测试..."
    cd src-tauri
    cargo test
    cd ..
    print_success "测试通过"
}

# 代码检查
check() {
    print_info "运行代码检查..."
    cd src-tauri
    cargo clippy -- -D warnings
    cd ..
    print_success "代码检查通过"
}

# 格式化代码
format() {
    print_info "格式化代码..."
    cd src-tauri
    cargo fmt
    cd ..
    pnpm prettier --write "src/**/*.{ts,tsx,css}"
    print_success "代码格式化完成"
}

# 显示帮助信息
show_help() {
    cat << EOF
${BLUE}NFS Manager 运行脚本${NC}

用法: ./run.sh [命令]

命令:
  ${GREEN}dev${NC}              启动开发模式（热重载）
  ${GREEN}build${NC}            构建生产版本
  ${GREEN}build-frontend${NC}  仅构建前端
  ${GREEN}build-backend${NC}   仅构建后端
  ${GREEN}test${NC}             运行测试
  ${GREEN}check${NC}            运行代码检查（clippy）
  ${GREEN}format${NC}           格式化代码
  ${GREEN}clean${NC}            清理构建产物
  ${GREEN}help${NC}             显示此帮助信息

示例:
  ./run.sh dev          # 启动开发服务器
  ./run.sh build        # 构建应用
  ./run.sh clean build  # 清理后重新构建

EOF
}

# 主函数
main() {
    # 检查是否在项目根目录
    if [ ! -f "package.json" ] || [ ! -d "src-tauri" ]; then
        print_error "请在项目根目录运行此脚本"
        exit 1
    fi

    # 如果没有参数，显示帮助
    if [ $# -eq 0 ]; then
        show_help
        exit 0
    fi

    # 检查依赖
    check_dependencies

    # 安装依赖
    install_deps

    # 执行命令
    case "$1" in
        dev)
            dev
            ;;
        build)
            build
            ;;
        build-frontend)
            build_frontend
            ;;
        build-backend)
            build_backend
            ;;
        test)
            test
            ;;
        check)
            check
            ;;
        format)
            format
            ;;
        clean)
            clean
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "未知命令: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# 运行主函数
main "$@"
