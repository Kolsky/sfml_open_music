use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let parent = Path::parent;
    let out_dir = env::var_os("OUT_DIR");
    let out_dir = out_dir
        .as_ref()
        .map(|os_str| Path::new(os_str))
        .and_then(parent)
        .and_then(parent)
        .and_then(parent)
        .unwrap();

    fs::read_dir("./dll/")
        .unwrap()
        .filter_map(Result::ok)
        .for_each(|f| {
            let from = f.path();
            let to = out_dir.join(from.file_name().unwrap());
            fs::copy(&from, &to).unwrap();
        });
}
