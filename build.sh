#!/bin/bash
# NFS Manager 多平台构建脚本

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

print_header() {
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

# 检查依赖
check_dependencies() {
    print_info "检查构建依赖..."

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

# 安装前端依赖
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
    print_info "清理旧的构建产物..."
    rm -rf dist/
    rm -rf src-tauri/target/release/bundle/
    print_success "清理完成"
}

# 构建当前平台
build_current() {
    print_header "构建当前平台"

    print_info "构建中..."
    pnpm tauri build

    print_success "构建完成"
    show_artifacts
}

# 构建 macOS
build_macos() {
    print_header "构建 macOS"

    if [[ "$OSTYPE" != "darwin"* ]]; then
        print_error "macOS 构建只能在 macOS 系统上进行"
        return 1
    fi

    print_info "构建 macOS 应用..."
    pnpm tauri build --target universal-apple-darwin

    print_success "macOS 构建完成"

    if [ -d "src-tauri/target/universal-apple-darwin/release/bundle/macos" ]; then
        print_info "产物位置: src-tauri/target/universal-apple-darwin/release/bundle/macos/"
        ls -lh src-tauri/target/universal-apple-darwin/release/bundle/macos/
    elif [ -d "src-tauri/target/release/bundle/macos" ]; then
        print_info "产物位置: src-tauri/target/release/bundle/macos/"
        ls -lh src-tauri/target/release/bundle/macos/
    fi
}

# 构建 Linux
build_linux() {
    print_header "构建 Linux"

    if [[ "$OSTYPE" != "linux-gnu"* ]]; then
        print_warning "Linux 构建建议在 Linux 系统上进行"
        print_info "尝试交叉编译..."
    fi

    # 检查是否安装了交叉编译工具
    if ! command -v cross &> /dev/null; then
        print_warning "未安装 cross 工具，尝试使用 cargo"
        print_info "如需交叉编译，请安装: cargo install cross"
    fi

    print_info "构建 Linux 应用..."
    pnpm tauri build --target x86_64-unknown-linux-gnu

    print_success "Linux 构建完成"

    if [ -d "src-tauri/target/x86_64-unknown-linux-gnu/release/bundle" ]; then
        print_info "产物位置: src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/"
        ls -lh src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/
    fi
}

# 构建 Windows
build_windows() {
    print_header "构建 Windows"

    if [[ "$OSTYPE" != "msys" && "$OSTYPE" != "win32" ]]; then
        print_warning "Windows 构建建议在 Windows 系统上进行"
        print_info "尝试交叉编译..."
    fi

    # 检查是否安装了交叉编译工具
    if ! command -v cross &> /dev/null; then
        print_warning "未安装 cross 工具"
        print_info "如需交叉编译，请安装: cargo install cross"
    fi

    print_info "构建 Windows 应用..."
    pnpm tauri build --target x86_64-pc-windows-msvc

    print_success "Windows 构建完成"

    if [ -d "src-tauri/target/x86_64-pc-windows-msvc/release/bundle" ]; then
        print_info "产物位置: src-tauri/target/x86_64-pc-windows-msvc/release/bundle/"
        ls -lh src-tauri/target/x86_64-pc-windows-msvc/release/bundle/
    fi
}

# 构建所有平台
build_all() {
    print_header "构建所有平台"

    print_warning "注意: 交叉编译可能需要额外的工具和配置"
    print_info "建议在各自平台上进行原生编译以获得最佳结果"
    echo ""

    # 根据当前平台决定构建顺序
    case "$OSTYPE" in
        darwin*)
            build_macos
            print_info "跳过 Linux 和 Windows（需要交叉编译）"
            ;;
        linux-gnu*)
            build_linux
            print_info "跳过 macOS 和 Windows（需要交叉编译）"
            ;;
        msys*|win32*)
            build_windows
            print_info "跳过 macOS 和 Linux（需要交叉编译）"
            ;;
        *)
            print_error "未知的操作系统: $OSTYPE"
            exit 1
            ;;
    esac

    print_success "构建完成"
}

# 显示构建产物
show_artifacts() {
    print_header "构建产物"

    if [ -d "src-tauri/target/release/bundle" ]; then
        echo "📦 构建产物位置: src-tauri/target/release/bundle/"
        echo ""

        # macOS
        if [ -d "src-tauri/target/release/bundle/macos" ]; then
            print_info "macOS:"
            ls -lh src-tauri/target/release/bundle/macos/ | grep -v "^total" | awk '{print "  " $9 " (" $5 ")"}'
        fi

        # Linux
        if [ -d "src-tauri/target/release/bundle/deb" ]; then
            print_info "Linux (DEB):"
            ls -lh src-tauri/target/release/bundle/deb/*.deb 2>/dev/null | awk '{print "  " $9 " (" $5 ")"}'
        fi

        if [ -d "src-tauri/target/release/bundle/appimage" ]; then
            print_info "Linux (AppImage):"
            ls -lh src-tauri/target/release/bundle/appimage/*.AppImage 2>/dev/null | awk '{print "  " $9 " (" $5 ")"}'
        fi

        # Windows
        if [ -d "src-tauri/target/release/bundle/msi" ]; then
            print_info "Windows (MSI):"
            ls -lh src-tauri/target/release/bundle/msi/*.msi 2>/dev/null | awk '{print "  " $9 " (" $5 ")"}'
        fi

        if [ -d "src-tauri/target/release/bundle/nsis" ]; then
            print_info "Windows (NSIS):"
            ls -lh src-tauri/target/release/bundle/nsis/*.exe 2>/dev/null | awk '{print "  " $9 " (" $5 ")"}'
        fi
    else
        print_warning "未找到构建产物"
    fi
}

# 显示帮助信息
show_help() {
    cat << EOF
${BLUE}NFS Manager 构建脚本${NC}

用法: ./build.sh [选项]

选项:
  ${GREEN}current${NC}        构建当前平台（默认）
  ${GREEN}macos${NC}          构建 macOS 应用（需要在 macOS 上运行）
  ${GREEN}linux${NC}          构建 Linux 应用
  ${GREEN}windows${NC}        构建 Windows 应用
  ${GREEN}all${NC}            构建所有平台（当前平台原生构建）
  ${GREEN}clean${NC}          清理构建产物
  ${GREEN}artifacts${NC}      显示构建产物
  ${GREEN}help${NC}           显示此帮助信息

示例:
  ./build.sh              # 构建当前平台
  ./build.sh macos        # 构建 macOS
  ./build.sh clean        # 清理构建产物
  ./build.sh all          # 构建所有平台

注意:
  - 交叉编译需要额外的工具和配置
  - 建议在目标平台上进行原生编译
  - macOS 可以构建 Universal Binary（支持 Intel 和 Apple Silicon）

EOF
}

# 主函数
main() {
    # 检查是否在项目根目录
    if [ ! -f "package.json" ] || [ ! -d "src-tauri" ]; then
        print_error "请在项目根目录运行此脚本"
        exit 1
    fi

    # 如果没有参数，默认构建当前平台
    if [ $# -eq 0 ]; then
        check_dependencies
        install_deps
        build_current
        exit 0
    fi

    # 执行命令
    case "$1" in
        current)
            check_dependencies
            install_deps
            build_current
            ;;
        macos)
            check_dependencies
            install_deps
            build_macos
            ;;
        linux)
            check_dependencies
            install_deps
            build_linux
            ;;
        windows)
            check_dependencies
            install_deps
            build_windows
            ;;
        all)
            check_dependencies
            install_deps
            build_all
            ;;
        clean)
            clean
            ;;
        artifacts)
            show_artifacts
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
