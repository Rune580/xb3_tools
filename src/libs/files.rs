use std::path::{PathBuf, Path};

use clap::ValueEnum;

#[derive(Debug, Clone, Copy)]
pub enum InputOutputLayout {
    FileInFileOut,
    FileInDirOut,
    DirInDirOut
}

#[derive(Debug, Clone)]
pub struct InputOutputPair {
    pub input: PathBuf,
    pub output: PathBuf,
    pub layout: InputOutputLayout
}

impl InputOutputPair {
    pub fn new<P, Q>(
            input: P,
            output: Q,
            layout: InputOutputLayout
    ) -> InputOutputPair
    where P: AsRef<Path>, Q: AsRef<Path> {
        InputOutputPair { input: input.as_ref().to_path_buf(), output: output.as_ref().to_path_buf(), layout: layout }
    }
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum TextureOutputFormat {
    DDS,
    Png
}