// 构建脚本
// 在../usr/src源码或者镜像变化的时候重新生成 link_app.asm
// 保证与内核链接的始终是最新的用户程序
use std::fs::{read_dir, File};
use std::io::{Result, Write};

fn main() {
    println!("cargo:rerun-if-changed=../usr/src");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_or_update().unwrap();
}

static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";
static ASM_FILE_NAME: &str = "src/link_app.asm";

fn insert_or_update() -> Result<()> {
    // 重新生成asm
    // 1.生成asm文件
    let mut f = File::create(ASM_FILE_NAME).unwrap();
    // 2.列举bin文件名字
    let mut apps: Vec<String> = read_dir("../user/src/bin")
        .unwrap()
        .into_iter()
        .map(|entry| {
            let mut entry = entry.unwrap().file_name().into_string().unwrap();
            entry.drain(entry.find('.').unwrap()..entry.len());
            entry
        })
        .collect();
    apps.sort();
    // 3.向文件写数据
    writeln!(
        &mut f,
        r#"
    .globl _num_app
    .section .data
    .align 3
_num_app:
    .quad {}"#,
        apps.len()
    )?;
    for i in 0..apps.len() {
        writeln!(&mut f, r#"   .quad app_{}_start"#, i)?;
    }
    writeln!(&mut f, r#"   .quad app_{}_end"#, apps.len() - 1)?;
    for (idx, app_name) in apps.into_iter().enumerate() {
        writeln!(
            &mut f,
            r#"
        .section .data
        .global app_{0}_start
        .global app_{0}_end
      app_{0}_start:
        .incbin "{2}{1}.bin"
      app_{0}_end:"#,
            idx, app_name, TARGET_PATH
        )?;
    }
    Ok(())
}
