#!/bin/bash

# 版本验证脚本 - 验证Git标签与Cargo.toml版本一致性
# 用于release工作流中的版本验证

set -euo pipefail

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 获取Git标签版本
get_git_tag_version() {
    if [ -n "${GITHUB_REF:-}" ]; then
        # 从GitHub Actions环境变量获取
        echo "${GITHUB_REF#refs/tags/}"
    elif [ -n "${1:-}" ]; then
        # 从命令行参数获取
        echo "$1"
    else
        # 从当前Git标签获取
        git describe --tags --exact-match 2>/dev/null || {
            log_error "无法获取当前Git标签"
            return 1
        }
    fi
}

# 获取Cargo.toml中的版本
get_cargo_version() {
    if [ ! -f "Cargo.toml" ]; then
        log_error "Cargo.toml文件不存在"
        return 1
    fi

    grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'
}

# 验证版本格式
validate_version_format() {
    local version="$1"

    # 检查语义化版本格式 (vX.Y.Z 或 X.Y.Z)
    if [[ ! "$version" =~ ^v?[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9\.-]+)?(\+[a-zA-Z0-9\.-]+)?$ ]]; then
        log_error "版本格式无效: $version"
        log_error "期望格式: vX.Y.Z 或 X.Y.Z (支持预发布和构建元数据)"
        return 1
    fi

    log_info "版本格式验证通过: $version"
    return 0
}

# 标准化版本号（移除v前缀）
normalize_version() {
    local version="$1"
    echo "${version#v}"
}

# 主验证函数
main() {
    log_info "开始版本验证..."

    # 获取Git标签版本
    local git_tag_version
    git_tag_version=$(get_git_tag_version "${1:-}")
    log_info "Git标签版本: $git_tag_version"

    # 获取Cargo.toml版本
    local cargo_version
    cargo_version=$(get_cargo_version)
    log_info "Cargo.toml版本: $cargo_version"

    # 验证版本格式
    validate_version_format "$git_tag_version"
    validate_version_format "$cargo_version"

    # 标准化版本号进行比较
    local normalized_git_version
    local normalized_cargo_version
    normalized_git_version=$(normalize_version "$git_tag_version")
    normalized_cargo_version=$(normalize_version "$cargo_version")

    # 比较版本
    if [ "$normalized_git_version" = "$normalized_cargo_version" ]; then
        log_info "✅ 版本验证成功！"
        log_info "Git标签: $git_tag_version"
        log_info "Cargo.toml: $cargo_version"

        # 输出GitHub Actions环境变量
        if [ -n "${GITHUB_ENV:-}" ]; then
            echo "RELEASE_VERSION=$git_tag_version" >> "$GITHUB_ENV"
            echo "CARGO_VERSION=$cargo_version" >> "$GITHUB_ENV"
            log_info "已设置GitHub Actions环境变量"
        fi

        return 0
    else
        log_error "❌ 版本不匹配！"
        log_error "Git标签版本: $git_tag_version (标准化: $normalized_git_version)"
        log_error "Cargo.toml版本: $cargo_version (标准化: $normalized_cargo_version)"
        log_error "请确保Git标签与Cargo.toml中的版本一致"
        return 1
    fi
}

# 显示帮助信息
show_help() {
    cat << EOF
版本验证脚本

用法:
    $0 [版本号]

参数:
    版本号    可选，指定要验证的版本号。如果未提供，将从Git标签或GITHUB_REF获取

示例:
    $0                  # 从当前Git标签获取版本
    $0 v0.1.0          # 验证指定版本
    GITHUB_REF=refs/tags/v0.1.0 $0  # 从GitHub Actions环境变量获取

退出代码:
    0    验证成功
    1    验证失败或错误
EOF
}

# 处理命令行参数
case "${1:-}" in
    -h|--help)
        show_help
        exit 0
        ;;
    *)
        main "$@"
        ;;
esac