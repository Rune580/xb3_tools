mod libs;

use clap::Parser;
use libs::{cli::{Cli, ToolTask}, tasks};

fn main() {
    let args = Cli::parse();

    match args.task {
        ToolTask::WilayDecode(decode_args) => {
            tasks::execute_decode_wilay(decode_args.io, decode_args.format, args.max_threads);
//            wilay::decode_wilay(decode_args.input_file, decode_args.output_file)
        },
        ToolTask::WilayEncode => todo!(),
    }
}