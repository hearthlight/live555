//! Build-script shim exposing the location of the live555 source tree.
//! Consumers depend on this crate as a git build-dependency and copy
//! `source_dir()` somewhere writable to build with live555's own makefiles;
//! this crate does not build anything itself.

/// The repository root, i.e. the directory containing `genMakefiles`.
pub fn source_dir() -> &'static std::path::Path {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
}
