use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs::File;
use wim_parser::WimParser;

/// 创建测试用的XML数据（模拟真实的WIM XML内容）
fn create_test_xml_data(image_count: usize) -> Vec<u8> {
    let mut xml_content = String::new();

    // UTF-16 LE BOM
    let mut result = vec![0xFF, 0xFE];

    xml_content.push_str(
        r#"<?xml version="1.0" encoding="utf-16"?>
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
        <SERVICINGDATA>
            <GDRDUREVISION>2428</GDRDUREVISION>
            <PKEYCONFIGVERSION>10.0.22621.1</PKEYCONFIGVERSION>
        </SERVICINGDATA>
    </IMAGE>"#,
    );

    // 为大型测试创建多个镜像
    for i in 2..=image_count {
        xml_content.push_str(&format!(
            r#"
    <IMAGE INDEX="{i}">
        <DIRCOUNT>25000</DIRCOUNT>
        <FILECOUNT>120000</FILECOUNT>
        <TOTALBYTES>20000000000</TOTALBYTES>
        <WINDOWS>
            <ARCH>9</ARCH>
            <PRODUCTNAME>Microsoft® Windows® Operating System</PRODUCTNAME>
            <EDITIONID>Professional</EDITIONID>
        </WINDOWS>
        <DISPLAYNAME>Windows 11 专业版 {i}</DISPLAYNAME>
        <DISPLAYDESCRIPTION>Windows 11 专业版 {i}</DISPLAYDESCRIPTION>
        <NAME>Windows 11 Pro {i}</NAME>
        <DESCRIPTION>Windows 11 Pro {i}</DESCRIPTION>
    </IMAGE>"#
        ));
    }

    xml_content.push_str("\n</WIM>");

    // 转换为UTF-16 LE字节
    for ch in xml_content.encode_utf16() {
        result.extend_from_slice(&ch.to_le_bytes());
    }

    result
}

/// 基准测试：当前的XML解析实现
fn bench_current_xml_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("xml_parsing_current");

    for image_count in [1, 5, 10, 20].iter() {
        let xml_data = create_test_xml_data(*image_count);

        group.bench_with_input(
            BenchmarkId::new("current_parser", image_count),
            &xml_data,
            |b, data| {
                b.iter(|| {
                    let mut parser = WimParser::new_for_test(File::open("/dev/null").unwrap());
                    parser.parse_xml_data_for_bench(black_box(data)).unwrap()
                })
            },
        );
    }

    group.finish();
}

/// 基准测试：优化的XML解析实现
fn bench_optimized_xml_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("xml_parsing_optimized");

    for image_count in [1, 5, 10, 20].iter() {
        let xml_data = create_test_xml_data(*image_count);

        group.bench_with_input(
            BenchmarkId::new("optimized_parser", image_count),
            &xml_data,
            |b, data| {
                b.iter(|| {
                    let mut parser = WimParser::new_for_test(File::open("/dev/null").unwrap());
                    parser
                        .parse_xml_data_optimized_for_bench(black_box(data))
                        .unwrap()
                })
            },
        );
    }

    group.finish();
}

/// 基准测试：UTF-16解码性能比较
fn bench_utf16_decoding(c: &mut Criterion) {
    let test_data = create_test_xml_data(10);
    let utf16_data = &test_data[2..]; // 跳过BOM

    let mut group = c.benchmark_group("utf16_decoding");

    // 当前实现：手动转换
    group.bench_function("manual_utf16_conversion", |b| {
        b.iter(|| {
            let mut utf16_chars = Vec::new();
            for chunk in utf16_data.chunks_exact(2) {
                let char_val = u16::from_le_bytes([chunk[0], chunk[1]]);
                utf16_chars.push(char_val);
            }
            String::from_utf16(black_box(&utf16_chars))
        })
    });

    // 优化实现：encoding_rs
    group.bench_function("encoding_rs_utf16", |b| {
        b.iter(|| {
            let (decoded, _, _) = encoding_rs::UTF_16LE.decode(black_box(utf16_data));
            decoded.into_owned()
        })
    });

    group.finish();
}

/// 内存使用基准测试
fn bench_memory_allocation(c: &mut Criterion) {
    let _xml_data = create_test_xml_data(20);

    let mut group = c.benchmark_group("memory_allocation");

    // 测试大量字符串分配的影响
    group.bench_function("string_allocations", |b| {
        b.iter(|| {
            let mut strings = Vec::new();
            for i in 0..1000 {
                strings.push(format!("test_string_{i}"));
            }
            black_box(strings)
        })
    });

    // 测试预分配Vector的性能
    group.bench_function("vector_preallocation", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(1000);
            for i in 0..1000 {
                vec.push(i);
            }
            black_box(vec)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_current_xml_parsing,
    bench_optimized_xml_parsing,
    bench_utf16_decoding,
    bench_memory_allocation
);
criterion_main!(benches);
