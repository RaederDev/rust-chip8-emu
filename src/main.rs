use chip8::{cpu::CPU, memory::Memory};
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

fn main() {
    let mem = Memory::new();
    let mut cpu = CPU::new(mem);
    let path = get_file_path(&"./div.ch8".to_string()).unwrap();
    let file_bytes = read_file(&path);
    println!("{}", file_bytes.len());
    cpu.memory.load_program(&file_bytes);
    println!("{}", cpu.memory);
}

fn read_file(path: &PathBuf) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf);
    buf
}

fn get_file_path(file_or_path: &String) -> Result<PathBuf, String> {
    let passed_path = PathBuf::from(file_or_path);

    if passed_path.is_file() {
        return Ok(passed_path);
    }

    Err("Invalid path".to_string())
}

