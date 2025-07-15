// 性能对比示例
use std::time::Instant;
use wim_parser::WimParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    #[cfg(feature = "logging")]
    tracing_subscriber::fmt::init();

    // 创建测试数据
    let test_xml = create_test_xml_data();

    println!("WIM Parser 性能优化对比测试");
    println!("========================================");

    // 测试当前实现
    println!("\n🔴 测试当前实现（字符串匹配解析）");
    let start = Instant::now();

    let mut parser_current = WimParser::new_for_test(std::fs::File::open("/dev/null")?);
    for i in 0..100 {
        parser_current.parse_xml_data_for_bench(&test_xml)?;
        if i % 20 == 0 {
            print!(".");
        }
    }

    let current_duration = start.elapsed();
    println!("\n当前实现 100次解析用时: {:?}", current_duration);

    // 测试优化实现
    println!("\n🟢 测试优化实现（quick-xml + encoding_rs）");
    let start = Instant::now();

    let mut parser_optimized = WimParser::new_for_test(std::fs::File::open("/dev/null")?);
    for i in 0..100 {
        parser_optimized.parse_xml_data_optimized_for_bench(&test_xml)?;
        if i % 20 == 0 {
            print!(".");
        }
    }

    let optimized_duration = start.elapsed();
    println!("\n优化实现 100次解析用时: {:?}", optimized_duration);

    // 计算性能提升
    let speedup = current_duration.as_nanos() as f64 / optimized_duration.as_nanos() as f64;
    println!("\n📊 性能对比结果:");
    println!("   当前实现: {:?}", current_duration);
    println!("   优化实现: {:?}", optimized_duration);
    println!("   性能提升: {:.2}x", speedup);
    println!("   提升百分比: {:.1}%", (speedup - 1.0) * 100.0);

    // 内存使用对比
    println!("\n💾 内存优化特性:");
    println!("   ✓ 64KB 缓冲I/O");
    println!("   ✓ 字符串池减少分配");
    println!("   ✓ 预分配Vector容量");
    println!("   ✓ encoding_rs零拷贝UTF-16解码");
    println!("   ✓ quick-xml事件驱动解析");

    Ok(())
}

/// 创建测试用的XML数据
fn create_test_xml_data() -> Vec<u8> {
    let mut result = vec![0xFF, 0xFE]; // UTF-16 LE BOM

    let xml_content = r#"<?xml version="1.0" encoding="utf-16"?>
<WIM>
    <TOTALBYTES>22577165103</TOTALBYTES>
    <IMAGE INDEX="1">
        <DIRCOUNT>30978</DIRCOUNT>
        <FILECOUNT>136042</FILECOUNT>
        <TOTALBYTES>22577165103</TOTALBYTES>
        <WINDOWS>
            <ARCH>9</ARCH>
            <PRODUCTNAME>Microsoft® Windows® Operating System</PRODUCTNAME>
            <EDITIONID>Education</EDITIONID>
            <INSTALLATIONTYPE>Client</INSTALLATIONTYPE>
            <PRODUCTTYPE>WinNT</PRODUCTTYPE>
            <PRODUCTSUITE></PRODUCTSUITE>
            <LANGUAGES>
                <LANGUAGE>zh-CN</LANGUAGE>
                <DEFAULT>zh-CN</DEFAULT>
            </LANGUAGES>
            <VERSION>
                <MAJOR>10</MAJOR>
                <MINOR>0</MINOR>
                <BUILD>22621</BUILD>
                <SPBUILD>2428</SPBUILD>
                <SPLEVEL>0</SPLEVEL>
            </VERSION>
        </WINDOWS>
        <DISPLAYNAME>Windows 11 教育版</DISPLAYNAME>
        <DISPLAYDESCRIPTION>Windows 11 教育版</DISPLAYDESCRIPTION>
        <NAME>Windows 11 Education</NAME>
        <DESCRIPTION>Windows 11 Education</DESCRIPTION>
        <FLAGS>Education</FLAGS>
    </IMAGE>
    <IMAGE INDEX="2">
        <DIRCOUNT>25000</DIRCOUNT>
        <FILECOUNT>120000</FILECOUNT>
        <TOTALBYTES>20000000000</TOTALBYTES>
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
</WIM>"#;

    // 转换为UTF-16 LE字节
    for ch in xml_content.encode_utf16() {
        result.extend_from_slice(&ch.to_le_bytes());
    }

    result
}
