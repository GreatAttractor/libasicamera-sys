use std::env;
use std::path::Path;

fn main() {
    let target = env::var("TARGET").expect("getting target");

    let libdir = match env::var("ASICAMERA_LIBDIR") {
        Ok(dir) => dir,
        Err(_) => {
            match target.as_ref() {
                "x86_64-pc-windows-msvc"
				| "x86_64-pc-windows-gnu"
				| "i686-pc-windows-msvc"
				| "i686-pc-windows-gnu" => panic!("Missing env. variable ASICAMERA_LIBDIR. Set it to ASICamera2.DLL's location."),

                _ => "/usr/lib".to_string(),
            }
        }
    };

    let libdir = Path::new(&libdir);
    let libname = "ASICamera2";

    println!("cargo:rustc-link-search=native={}", libdir.display());
    println!("cargo:rustc-link-lib={}", libname);
}
