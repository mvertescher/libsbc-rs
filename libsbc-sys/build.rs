
extern crate pkg_config;

fn main() {
    if let Err(e) = pkg_config::probe_library("sbc") {
        match e {
            pkg_config::Error::Failure { .. } => panic! (
                "Pkg-config failed - usually this is because sbc development headers are not installed.\n\n\
                pkg_config details:\n{}",
                e
            ),
            _ => panic!("{}", e)
        }
    }
}
