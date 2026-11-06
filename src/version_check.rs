//! First-use compatibility check between these bindings and the native library.
//!
//! The bindings accept exactly one native `libcadlib`: the one released with
//! them. The expected version is the generated [`LIB_VERSION`]; the native lib's
//! comes from `cad_version()`. Any difference fails, and the error points at the
//! install scripts of the expected release.

use std::{ffi::CStr, sync::OnceLock};

use crate::{ffi, lib_version::LIB_VERSION};

/// The native library version reported by `cad_version()` (empty if null).
fn lib_version() -> String {
    // SAFETY: `cad_version` returns an owned, NUL-terminated C string or null.
    let ptr = unsafe { ffi::cad_version() };
    if ptr.is_null() {
        return String::new();
    }
    // SAFETY: `ptr` is a valid string owned by the library, freed below.
    let owned = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    // SAFETY: hand the library-allocated string back to be freed.
    unsafe { ffi::cad_free_string(ptr) };
    owned
}

/// The version string of the loaded native library (e.g. `"3.4.0"`).
#[must_use]
pub fn version() -> String {
    lib_version()
}

/// Reports an error unless the native lib version is exactly `expected`.
fn ensure_lib_version(expected: &str, lib: &str) -> Result<(), String> {
    if lib == expected {
        return Ok(());
    }
    Err(format!(
        "native libcadlib v{lib} does not match these Rust bindings (v{expected}); install it \
         with install-cadlib.sh or install-cadlib.ps1 from the v{expected} release"
    ))
}

static CHECKED: OnceLock<Result<(), String>> = OnceLock::new();

/// Verify, once per process, that the native lib is the one these bindings
/// require. The result is cached so repeated `Cad::load` calls are cheap.
pub(crate) fn check_lib_version() -> Result<(), String> {
    CHECKED
        .get_or_init(|| ensure_lib_version(LIB_VERSION, &lib_version()))
        .clone()
}

#[cfg(test)]
mod tests {
    use super::{check_lib_version, ensure_lib_version, lib_version, LIB_VERSION};

    #[test]
    fn linked_lib_matches_bindings() {
        assert_eq!(check_lib_version(), Ok(()));
    }

    #[test]
    fn mismatched_lib_is_rejected() {
        let lib = lib_version();
        let expected = format!("{LIB_VERSION}-mismatch");
        assert_eq!(
            ensure_lib_version(&expected, &lib),
            Err(format!(
                "native libcadlib v{lib} does not match these Rust bindings (v{expected}); \
                 install it with install-cadlib.sh or install-cadlib.ps1 from the v{expected} \
                 release"
            ))
        );
    }
}
