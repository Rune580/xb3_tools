use std::{env, path::{PathBuf, Path}, fs};

const THIRD_PARTY_DIR: &'static str = "third-party";

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let out = PathBuf::from(format!("target/{}/{}", profile, THIRD_PARTY_DIR));

    if out.exists() {
        fs::remove_dir_all(&out).unwrap();
    }

    fs::create_dir(&out).unwrap();

    copy_dir(THIRD_PARTY_DIR, out);
}

fn copy_dir<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>
{
    let to = to.as_ref().to_path_buf();

    for path in fs::read_dir(from).unwrap() {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
            fs::copy(&path, to).unwrap();
        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }

            copy_dir(&path, to);
        }
    }
}