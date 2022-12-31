use std::{path::PathBuf, fs};

use clap::{command, arg, Parser, Args, Subcommand};

use super::files::{TextureOutputFormat, InputOutputPair, InputOutputLayout};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub task: ToolTask,

    #[arg(long, default_value("4"))]
    pub max_threads: usize,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ToolTask {
    WilayDecode(WilayDecodeArgs),
    WilayEncode
}

#[derive(Args, Debug, Clone)]
pub struct WilayDecodeArgs {
    #[command(flatten)]
    pub io: InputOutputArgs,

    #[arg(short, value_enum)]
    pub format: TextureOutputFormat
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct InputOutputArgs {
    #[arg(short, long, required(true))]
    pub input: String,

    #[arg(short, long, required(true))]
    pub output: String,
}

impl InputOutputArgs {
    pub fn assert_valid_args(&self) {
        let layout = self.get_layout();
        assert!(layout.is_ok());
    }

    pub fn to_input_output_pair(&self) -> InputOutputPair {
        let layout = self.get_layout().expect("Failed to determine layout of input output arguments!");
        InputOutputPair::new(&self.input, &self.output, layout)
    }

    fn get_layout(&self) -> Result<InputOutputLayout, &'static str> {
        let input = PathBuf::from(&self.input);
        let output = PathBuf::from(&self.output);

        if !output.is_file() {
            if !output.exists() {
                fs::create_dir_all(&output).unwrap();
            }
        }

        if input.is_file() && output.is_file() {
            Ok(InputOutputLayout::FileInFileOut)
        } else if input.is_file() && output.is_dir() {
            Ok(InputOutputLayout::FileInDirOut)
        } else if input.is_dir() && output.is_dir() {
            Ok(InputOutputLayout::DirInDirOut)
        } else {
            Err("Invalid arguments!")
        }
    }
}