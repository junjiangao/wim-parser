use std::env;
use wim_parser::WimParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    #[cfg(feature = "logging")]
    tracing_subscriber::fmt::init();

    // 从命令行参数获取 WIM 文件路径
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("用法: {} <wim_file_path>", args[0]);
        eprintln!("示例: {} /path/to/install.wim", args[0]);
        std::process::exit(1);
    }

    let wim_path = &args[1];
    println!("正在解析 WIM 文件: {}", wim_path);

    // 创建解析器并解析文件
    let mut parser = WimParser::new(wim_path)?;
    parser.parse_full()?;

    // 显示基本信息
    println!("\n=== WIM 文件信息 ===");
    println!("镜像数量: {}", parser.get_image_count());
    println!(
        "是否压缩: {}",
        if parser.is_compressed() { "是" } else { "否" }
    );

    if let Some(compression_type) = parser.get_compression_type() {
        println!("压缩类型: {}", compression_type);
    }

    // 显示文件头信息
    if let Some(header) = parser.get_header() {
        println!("\n=== 文件头信息 ===");
        println!("{}", header);
    }

    // 显示所有镜像信息
    println!("\n=== 镜像详情 ===");
    for (i, image) in parser.get_images().iter().enumerate() {
        println!("镜像 #{}: {}", i + 1, image.name);
        println!("  描述: {}", image.description);
        println!("  文件数: {}", image.file_count);
        println!("  目录数: {}", image.dir_count);
        println!("  总大小: {} MB", image.total_bytes / (1024 * 1024));

        if let Some(version) = &image.version {
            println!("  版本: {}", version);
        }

        if let Some(arch) = &image.architecture {
            println!("  架构: {}", arch);
        }

        if let Some(creation_time) = image.creation_time {
            println!("  创建时间: {}", creation_time);
        }

        println!();
    }

    // 显示 Windows 特定信息
    if let Some(windows_info) = parser.get_windows_info() {
        println!("=== Windows 信息摘要 ===");
        println!("{}", windows_info);
        println!("版本列表:");
        for edition in &windows_info.editions {
            println!("  - {}", edition);
        }
    }

    // 显示版本摘要
    let versions = parser.get_version_summary();
    if !versions.is_empty() {
        println!("\n=== 检测到的版本 ===");
        for version in versions {
            println!("  - {}", version);
        }
    }

    // 显示主要版本和架构
    if let Some(primary_version) = parser.get_primary_version() {
        println!("\n主要版本: {}", primary_version);
    }

    if let Some(primary_arch) = parser.get_primary_architecture() {
        println!("主要架构: {}", primary_arch);
    }

    // 检查特定版本和架构
    println!("\n=== 兼容性检查 ===");
    let check_versions = ["Windows 10", "Windows 11", "Windows Server"];
    for version in check_versions {
        if parser.has_version(version) {
            println!("✓ 包含 {}", version);
        }
    }

    let check_archs = ["x64", "x86", "ARM64"];
    for arch in check_archs {
        if parser.has_architecture(arch) {
            println!("✓ 支持 {} 架构", arch);
        }
    }

    println!("\n解析完成！");
    Ok(())
}
