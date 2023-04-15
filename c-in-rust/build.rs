extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main()
{
	let bindings = bindgen::Builder::default()
		.header("src/cuda_foo/foo.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("cuda_foo.rs"))
        .expect("Couldn't write bindings!");

	cc::Build::new()
		.file("src/cuda_foo/foo.cu")
		.include("src/cuda_foo")
		.cuda(true)
		.cudart("static")
		.static_crt(true)
		.compile("cuda_foo");
	println!("cargo:rerun-if-changed=src/cuda_foo/foo.cu");
}
