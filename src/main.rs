mod libs;

use clap::Parser;
use libs::{wilay, cli::{Cli, ToolTask}};

fn main() {
    let args = Cli::parse();

    match args.task {
        ToolTask::WilayDecode(decode_args) => {
//            wilay::decode_wilay(decode_args.input_file, decode_args.output_file)
        },
        ToolTask::WilayEncode => todo!(),
    }
}