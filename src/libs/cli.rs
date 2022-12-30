use std::{path::{Path, PathBuf}, fs};

use clap::{command, arg, Parser, Args, Subcommand, ArgGroup};

use super::files::TextureOutputFormat;

#[derive(Subcommand, Debug, Clone)]
pub enum ToolTask {
    WilayDecode(WilayDecodeArgs),
    WilayEncode
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub task: ToolTask,
}

#[derive(Args, Debug, Clone)]
pub struct WilayDecodeArgs {
    #[command(flatten)]
    pub io: InputOutputArgs,

    #[arg(short, value_enum)]
    pub format: TextureOutputFormat
}

#[derive(Args, Debug, Clone)]
#[command(group(ArgGroup::new("inputs")
        .required(true)
        .args(&["input_file", "input_dir"])
        ), group(ArgGroup::new("outputs")
        .required(true)
        .args(&["output_file", "output_dir"])
        ))]
pub struct InputOutputArgs {
    #[arg(short('i'), conflicts_with("input_dir"))]
    pub input_file: Option<String>,

    #[arg(long("in"), conflicts_with("input_file"), conflicts_with("output_file"))]
    pub input_dir: Option<String>,

    #[arg(short('o'), conflicts_with("input_dir"))]
    pub output_file: Option<String>,

    #[arg(long("out"), conflicts_with("output_file"))]
    pub output_dir: Option<String>
}

impl InputOutputArgs {
    pub fn assert_valid_args(&self) {
        let valid_inputs = (self.input_file.is_some() && self.input_dir.is_none()) ||
                           (self.input_file.is_none() && self.input_dir.is_some());
        let valid_outputs = (self.output_file.is_some() && self.output_dir.is_none()) ||
                            (self.output_file.is_none() && self.output_dir.is_some());

        let valid = (self.input_file.is_some() && self.input_dir.is_none() && valid_outputs) ||
                    (self.input_file.is_none() && self.input_dir.is_some() && self.output_file.is_none() && self.output_dir.is_some());

        assert!(valid_inputs && valid_outputs);
        assert!(valid);
    }

    pub fn get_output_file(&self, ext: String) -> Result<String, &'static str> {
        if self.input_dir.is_some() || self.input_file.is_none() {
            return Err("invalid input args for a single output file");
        }

        if self.output_file.is_some() {
            return Ok(self.output_file.as_ref().unwrap().to_owned());
        } else if self.output_dir.is_some() {
            let out_dir = PathBuf::from(self.output_dir.as_ref().unwrap());
            if !out_dir.exists() {
                fs::create_dir(&out_dir).unwrap();
            }

            let input_file_string = self.input_file.as_ref().unwrap();
            let input_file_path = Path::new(&input_file_string);
            let file_stem = input_file_path.file_stem().expect("failed to get input file stem!").to_str().expect("failed to get input file stem!");

            let mut output_file = file_stem.to_string();
            output_file.push_str(&ext);
            let out_dir = out_dir.join(output_file);
            return Ok(out_dir.to_str().unwrap().to_string());
        }

        return Err("invalid args for a single output file");
    }
}