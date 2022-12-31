use std::fs;
use std::path::{Path, PathBuf};

use workerpool::Pool;
use workerpool::thunk::{ThunkWorker, Thunk};

use super::cli::InputOutputArgs;
use super::files::{TextureOutputFormat, InputOutputPair, InputOutputLayout};
use super::{wilay, compressonator};

pub fn execute_decode_wilay(
        io_args: InputOutputArgs,
        format: TextureOutputFormat,
        max_threads: usize
) {
    io_args.assert_valid_args();

    let io = io_args.to_input_output_pair();
    let input = io.input.clone();
    let output = io.output.clone();
    let layout = io.layout;

    let out_dir = get_output_dir(io.output.clone());

    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).unwrap();
    }

    let need_decompress = matches!(format, TextureOutputFormat::Png);
    let mut temp_dir = out_dir.clone();
    if need_decompress {
        temp_dir = temp_dir.join("temp/");
    }

    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir).unwrap();
    }

    match layout {
        InputOutputLayout::FileInFileOut => {
            let output = temp_dir.join(output.file_stem().unwrap()).with_extension("dds");
            decode_wilay_internal(&input, &output, &out_dir, layout, format);
        },
        InputOutputLayout::FileInDirOut => {
            let output = temp_dir.join(input.file_stem().unwrap()).with_extension("dds");
            decode_wilay_internal(&input, &output, &out_dir, layout, format);
        },
        InputOutputLayout::DirInDirOut => {
            let pool = Pool::<ThunkWorker<()>>::new(max_threads);

            for path in fs::read_dir(&input).unwrap() {
                let path = path.unwrap().path();
                if !path.is_file() {
                    continue;
                }
                if path.extension().unwrap() != "wilay" {
                    continue;
                }

                let output = temp_dir.clone().join(path.file_stem().unwrap()).with_extension("dds");
                let out_dir_static = out_dir.clone();
                let layout_static = layout.clone();
                let format_static = format.clone();
                pool.execute(Thunk::of(move || decode_wilay_internal(path, output, out_dir_static, layout_static, format_static)));
            }

            pool.join();
        },
    }
}

fn get_output_dir<P>(
        output: P
) -> PathBuf
where P: AsRef<Path> {
    let output = output.as_ref().to_path_buf();
    let output = if output.is_dir() {
        output
    } else if output.is_file() {
        output.parent().unwrap().to_path_buf()
    } else {
      panic!();
    };
    output
}

fn decode_wilay_internal<P, Q, R>(
        input: P,
        output: Q,
        out_dir: R,
        layout: InputOutputLayout,
        format: TextureOutputFormat
)
where P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path> {
    let input = input.as_ref().to_path_buf();

    let io = InputOutputPair::new(&input, &output, layout);
    let result = wilay::decode_wilay(io);

    if result.is_err() {
        println!("failed to decode {} with err: {}", input.to_str().unwrap(), result.err().unwrap());
        return;
    }

    if matches!(format, TextureOutputFormat::Png) {
        compressonator::decompress_dds(output, out_dir)
    }
}