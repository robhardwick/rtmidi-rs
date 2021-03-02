use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=rtmidi");
    println!("cargo:rerun-if-changed=wrapper.h");

    let include_args = match pkg_config::Config::new()
        .statik(false)
        .atleast_version("4.0.0")
        .probe("rtmidi")
    {
        Err(_) => vec![],
        Ok(library) => library
            .include_paths
            .iter()
            .map(|include_path| {
                format!(
                    "-I{}",
                    include_path.to_str().expect("include path was not UTF-8")
                )
            })
            .collect::<Vec<_>>(),
    };

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(include_args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
