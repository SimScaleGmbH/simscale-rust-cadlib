//! Link the prebuilt cadlib native library.
//!
//! The library is linked **by name** (no hardcoded path) so this crate stays
//! relocatable. Install `libcadlib.so` to a system location (e.g. `/usr/local/lib` + `ldconfig`)
//! or point the linker at a custom directory via `CADLIB_LIB_DIR` (the analogue
//! of cgo's `CGO_LDFLAGS=-L<dir>`). See <https://github.com/SimScaleGmbH/simscale-rust-cadlib#install>.

use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rustc-link-lib=dylib=cadlib");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // -lm/-ldl/-lpthread are ELF/Linux-only; on macOS and Windows the C runtime
    // provides those symbols.
    if target_os == "linux" {
        for lib in ["m", "dl", "pthread"] {
            println!("cargo:rustc-link-lib=dylib={lib}");
        }
    }

    let lib_dir = env::var_os("CADLIB_LIB_DIR").map(PathBuf::from);
    if let Some(dir) = &lib_dir {
        println!("cargo:rustc-link-search=native={}", dir.display());
    }
    println!("cargo:rerun-if-env-changed=CADLIB_LIB_DIR");

    warn_if_lib_missing(&target_os, lib_dir);
}

/// Warns without failing when the native lib is in none of the linker's
/// usual places. Not an error: `cargo check`, `cargo publish`'s verify
/// build and docs.rs run this script without the lib.
fn warn_if_lib_missing(target_os: &str, lib_dir: Option<PathBuf>) {
    let (file, search_var, defaults): (&str, &str, &[&str]) = match target_os {
        "linux" => (
            "libcadlib.so",
            "LIBRARY_PATH",
            &["/usr/local/lib", "/usr/lib", "/lib", "/usr/lib64", "/lib64"],
        ),
        "macos" => (
            "libcadlib.dylib",
            "LIBRARY_PATH",
            &["/usr/local/lib", "/opt/homebrew/lib"],
        ),
        // MSVC links against the import lib, searched along LIB.
        "windows" => ("cadlib.lib", "LIB", &[]),
        _ => return,
    };
    println!("cargo:rerun-if-env-changed={search_var}");

    let mut dirs: Vec<PathBuf> = lib_dir.into_iter().collect();
    if let Some(paths) = env::var_os(search_var) {
        dirs.extend(env::split_paths(&paths));
    }
    dirs.extend(defaults.iter().map(PathBuf::from));
    if target_os == "linux" {
        if let Ok(arch) = env::var("CARGO_CFG_TARGET_ARCH") {
            dirs.push(Path::new("/usr/lib").join(format!("{arch}-linux-gnu")));
        }
    }
    if dirs.iter().any(|dir| dir.join(file).is_file()) {
        return;
    }

    let version = env::var("CARGO_PKG_VERSION").unwrap_or_default();
    let script = if target_os == "windows" {
        "install-cadlib.ps1"
    } else {
        "install-cadlib.sh"
    };
    println!(
        "cargo:warning=native {file} v{version} not found in CADLIB_LIB_DIR, {search_var} or the \
         default library paths; linking will fail. Install it with {script}: \
         https://github.com/SimScaleGmbH/simscale-rust-cadlib/blob/v{version}/{script}"
    );
    println!("cargo:warning=for a non-default prefix, set CADLIB_LIB_DIR to its lib directory");
}
