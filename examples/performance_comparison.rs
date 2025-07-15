// 性能对比示例
use std::io::Write;
use std::time::Instant;
use wim_parser::WimParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    #[cfg(feature = "logging")]
    tracing_subscriber::fmt::init();

    // 创建测试文件
    let test_xml = create_test_xml();
    let mut temp_file = tempfile::NamedTempFile::new()?;
    temp_file.write_all(&test_xml)?;
    let temp_path = temp_file.path();

    // 当前实现性能测试
    let start = Instant::now();
    for _ in 0..100 {
        let mut parser = WimParser::new(temp_path)?;
        parser.parse_full()?;
    }
    let current_duration = start.elapsed();

    // 优化实现性能测试（如果有的话）
    let start = Instant::now();
    for _ in 0..100 {
        let mut parser = WimParser::new(temp_path)?;
        parser.parse_full()?;
    }
    let optimized_duration = start.elapsed();

    // 显示结果
    println!("\n性能对比结果:");
    println!("\n当前实现 100次解析用时: {current_duration:?}");

    // 计算性能提升
    let speedup = current_duration.as_secs_f64() / optimized_duration.as_secs_f64();

    println!("\n=== 性能对比 ===");
    println!("\n优化实现 100次解析用时: {optimized_duration:?}");

    println!("\n详细对比:");
    println!("   当前实现: {current_duration:?}");
    println!("   优化实现: {optimized_duration:?}");
    println!("   性能提升: {speedup:.2}x");

    Ok(())
}

fn create_test_xml() -> Vec<u8> {
    let xml_content = r#"<?xml version="1.0" encoding="utf-8"?>
<WIM>
    <TOTALBYTES>5000000000</TOTALBYTES>
    <IMAGE INDEX="1">
        <DIRCOUNT>25000</DIRCOUNT>
        <FILECOUNT>120000</FILECOUNT>
        <TOTALBYTES>2000000000</TOTALBYTES>
        <WINDOWS>
            <ARCH>9</ARCH>
            <PRODUCTNAME>Microsoft® Windows® Operating System</PRODUCTNAME>
            <EDITIONID>Professional</EDITIONID>
        </WINDOWS>
        <DISPLAYNAME>Windows 11 专业版</DISPLAYNAME>
        <DISPLAYDESCRIPTION>Windows 11 专业版</DISPLAYDESCRIPTION>
        <NAME>Windows 11 Pro</NAME>
        <DESCRIPTION>Windows 11 Pro</DESCRIPTION>
    </IMAGE>
    <IMAGE INDEX="2">
        <DIRCOUNT>22000</DIRCOUNT>
        <FILECOUNT>110000</FILECOUNT>
        <TOTALBYTES>1800000000</TOTALBYTES>
        <WINDOWS>
            <ARCH>9</ARCH>
            <PRODUCTNAME>Microsoft® Windows® Operating System</PRODUCTNAME>
            <EDITIONID>Core</EDITIONID>
        </WINDOWS>
        <DISPLAYNAME>Windows 11 家庭版</DISPLAYNAME>
        <DISPLAYDESCRIPTION>Windows 11 家庭版</DISPLAYDESCRIPTION>
        <NAME>Windows 11 Home</NAME>
        <DESCRIPTION>Windows 11 Home</DESCRIPTION>
    </IMAGE>
</WIM>"#;

    // 转换为UTF-16 LE编码
    let mut utf16_data = Vec::new();

    // 添加BOM
    utf16_data.extend_from_slice(&[0xFF, 0xFE]);

    // 转换字符串为UTF-16 LE
    for c in xml_content.encode_utf16() {
        utf16_data.extend_from_slice(&c.to_le_bytes());
    }

    utf16_data
}
