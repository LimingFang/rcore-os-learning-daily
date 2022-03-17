// 1.对 src/bin 下的每个 rs 文件设置不同的 BASE_ADDRESS
// 2.通过 Command 生成子进程编译单个 binary crate
use std::fs::{read_dir, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::Command;

const BASE_ADDRESS: usize = 0x8040_0000;
const STEP: usize = 2 << 20; // 2MB
const LINKER_FILE: &str = "src/linker.ld";
const BIN_DIR: &str = "src/bin/";

fn main() {
    let file = OpenOptions::new().read(true).open(LINKER_FILE).unwrap();
    let mut raw_linker_content = String::new();
    let mut buf_reader = BufReader::new(&file);
    loop {
        let mut line = String::new();
        let bytes_read = buf_reader.read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }
        raw_linker_content.push_str(&line);
    }
    let raw_linker_content_backup = raw_linker_content.clone();

    let mut current_addr: usize = BASE_ADDRESS;
    let mut old_address = format!("0x{:x}", current_addr);
    let mut new_address = format!("0x{:x}", current_addr);
    let iter = read_dir(BIN_DIR).unwrap();
    for entry in iter {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(LINKER_FILE)
            .unwrap();
        let entry = entry.unwrap();
        let mut bin_file_name = entry.file_name().into_string().unwrap();
        bin_file_name = bin_file_name.replace(".rs", "");
        if let Some(index) = raw_linker_content.find(&old_address) {
            raw_linker_content.replace_range(index..(index + old_address.len()), &new_address);
            let mut buf_writer = BufWriter::new(&mut file);
            buf_writer.write(raw_linker_content.as_bytes()).unwrap();
            buf_writer.flush().unwrap();
            current_addr += STEP;
            old_address = new_address;
            new_address = format!("0x{:x}", current_addr);
        } else {
            panic!("..");
        }
        println!("{}", &bin_file_name);
        Command::new(format!("cargo"))
            .arg("build")
            .args(["--release", &format!("--bin {}", bin_file_name.as_str())])
            .output()
            .unwrap();
    }
    let mut file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(LINKER_FILE)
        .unwrap();
    let mut buf_writer = BufWriter::new(&mut file);
    buf_writer
        .write(raw_linker_content_backup.as_bytes())
        .unwrap();
    buf_writer.flush().unwrap();
}
