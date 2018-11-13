//! libsbc build file

extern crate cc;
extern crate pkg_config;

fn main() {
    #[cfg(feature = "source-build")]
    source_build();
    #[cfg(not(feature = "source-build"))]
    use_pkg_config();
}

#[cfg(feature = "source-build")]
fn source_build() {
    let mut build = cc::Build::new();
    build.include("sbc/sbc");

    #[cfg(target_os = "linux")]
    build.flag("-Wno-unused-parameter");

    let files = ["sbc/sbc/sbc.c", "sbc/sbc/sbc_primitives.c"];
    build.files(files.iter()).compile("sbc");

    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static=sbc");
}

#[cfg(not(feature = "source-build"))]
fn use_pkg_config() {
    if let Err(e) = pkg_config::probe_library("sbc") {
        match e {
            pkg_config::Error::Failure { .. } => panic!(
                "Pkg-config failed - usually this is because sbc development headers are not installed.\n\n\
                pkg_config details:\n{}",
                e
                ),
            _ => panic!("{}", e)
        }
    }
}
