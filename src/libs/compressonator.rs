use std::{path::Path, process::Command};

#[cfg(target_os = "linux")]
const CLI_PATH: &'static str = "third-party/compressonatorcli/linux/compressonatorcli";
#[cfg(target_os = "windows")]
const CLI_PATH: &'static str = "third-party/compressonatorcli/win32/CompressonatorCLI_x64_4.2.5185.exe";

pub fn decompress_dds<P, Q>(input: P, out_dir: Q)
where P: AsRef<Path>, Q: AsRef<Path> {
    let input = input.as_ref().to_path_buf();
    let out_dir = out_dir.as_ref().to_path_buf();

    let mut output_file = input.file_stem().unwrap().to_str().unwrap().to_string();
    output_file.push_str(".png");
    let output = out_dir.clone().join(output_file);

    let input_arg = format!("\"{}\"", input.to_str().unwrap());
    let output_arg = format!("\"{}\"", output.to_str().unwrap());
    let command = format!("{} {} {}", CLI_PATH, input_arg, output_arg);

    run_command(command);
}

#[cfg(target_os = "linux")]
fn run_command(
        command: String
) {
    match Command::new("sh")
        .arg("-c").arg(command)
        .output() {
        Ok(_) => {},
        Err(out) => panic!("failed to decompress dds, error: {}", out),
    };
}

#[cfg(target_os = "windows")]
fn run_command(
        command: String
) {
    match Command::new("sh")
        .arg("-c").arg(command)
        .output() {
        Ok(out) => println!("out: {}\nerr: {}", String::from_utf8(out.stdout).unwrap(), String::from_utf8(out.stderr).unwrap()),
        Err(out) => panic!("failed to decompress dds, error: {}", out),
    };
}