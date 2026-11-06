//! Package cadlib provides Rust bindings for the `SimScale` cadlib native library.
//!
//! This crate links against the **prebuilt** cadlib native library over the
//! stable C ABI (`libcadlib.so`). Install the native library first
//! (see <https://github.com/SimScaleGmbH/simscale-rust-cadlib#install>);
//! the crate's `build.rs` links it by name (`-lcadlib`).
//!
//! [`Cad::load`] takes the CAD data map entry of a workflow method's run request:
//! `Cad::load(&request.data_map.cad)?`.
//!
//! ```no_run
//! use cadlib::{Cad, VolumeParameters};
//!
//! # let cad_model = serde_json::json!({});
//! let cad = Cad::load(&cad_model)?;
//! let params = VolumeParameters { entity_ids: vec!["B1_TE5".to_string()] };
//! let result = cad.volume(Some(&params))?;
//! println!("{result:?}");
//! # Ok::<(), cadlib::CadLibError>(())
//! ```
//!
//! All query methods are **blocking**. From an async (e.g. tokio) context, run
//! them on the blocking pool so they don't stall the runtime:
//!
//! ```ignore
//! let result = tokio::task::spawn_blocking(move || cad.volume(Some(&params))).await??;
//! ```

mod cad;
mod error;
mod ffi;
mod lib_version;
pub mod models;
mod version_check;

pub use cad::Cad;
pub use error::CadLibError;
pub use models::*;
pub use version_check::version;
