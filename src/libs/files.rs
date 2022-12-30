use clap::ValueEnum;

#[derive(Debug, Clone)]
pub struct InputOutputPair {
    pub input: String,
    pub output: String
}

#[derive(ValueEnum, Debug, Clone)]
pub enum TextureOutputFormat {
    DDS,
    Png
}