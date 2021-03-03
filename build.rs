use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=rtmidi");
    println!("cargo:rerun-if-changed=wrapper.h");

    let (version, include_args) = match pkg_config::Config::new()
        .statik(false)
        .atleast_version("3.0.0")
        .probe("rtmidi")
    {
        Err(_) => ("4.0.0".to_string(), vec![]),
        Ok(library) => (
            library.version,
            library
                .include_paths
                .iter()
                .map(|include_path| {
                    format!(
                        "-I{}",
                        include_path.to_str().expect("include path was not UTF-8")
                    )
                })
                .collect::<Vec<_>>(),
        ),
    };

    let feature = match version.as_ref() {
        "4.0.0" => "v4_0_0",
        "3.0.0" => "v3_0_0",
        version => panic!("Unsupported RtMidi version '{}'", version),
    };
    println!("cargo:rustc-cfg=rtmidi_version=\"{}\"", feature);

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
