// use std::env;
use std::path::PathBuf;

fn main() -> miette::Result<()> {
    let include_path = PathBuf::from("vg/src/");
    let deps_path = PathBuf::from("vg/deps/libhandlegraph/src/include/handlegraph");

    miette::ensure!(include_path.exists(), "src path missing");
    miette::ensure!(deps_path.exists(), "deps path missing");

    let mut builder =
        autocxx_build::Builder::new("src/lib.rs", &[&include_path, &deps_path]).build()?;
    builder
        .flag_if_supported("-std=c++14")
        // .file("f5c/src/index.c");
        ;

    // let include = env::var_os("DEP_HDF5_INCLUDE").unwrap();
    // println!("cargo:include={}", include.to_string_lossy());
    // builder.include(include);
    // builder.define("HAVE_HDF5_H", "1");

    builder.compile("libvg");
    println!("cargo:rerun-if-changed=src/lib.rs");
    // println!("cargo:rustc-link-lib=hdf5");
    Ok(())
}
