mod libs;

use clap::{Parser, ValueEnum};
use libs::wilay;

fn main() {
    let args = Args::parse();

    match args.task {
        ToolTask::WilayDecode => wilay::decode_wilay(args.input_file, args.output_file),
        ToolTask::WilayEncode => todo!(),
    }
}

#[derive(ValueEnum, Debug, Clone)]
enum ToolTask {
    WilayDecode,
    WilayEncode
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, value_enum)]
    task: ToolTask,

    #[arg(short, long)]
    input_file: String,

    #[arg(short, long)]
    output_file: String,
}