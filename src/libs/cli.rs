use clap::{command, arg, Parser, Args, Subcommand};

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
    #[arg(short, long)]
    pub input_file: String,

//    #[arg(short, long, conflicts_with(""))]
//    pub output
}