// Code generated for the SimScale cadlib Rust bindings. DO NOT EDIT.
//! Raw `extern "C"` declarations for the prebuilt cadlib native library.
//!
//! These mirror the JSON-string FFI surface in `cadlib.h`.
//! The native library is linked by name in build.rs.

#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_int};

/// Opaque handle to a loaded CAD model, created by [`cad_load`] and released by
/// [`cad_destroy`]. Never construct or dereference one directly.
#[repr(C)]
pub struct CadHandle {
    _private: [u8; 0],
}

extern "C" {
    /// Load a CAD model from a CAD data map entry, serialized as a JSON object.
    /// Writes the handle to `*out_handle` on success; on failure returns non-zero
    /// and writes an error string to `*out_error` (free it with `cad_free_string`).
    pub fn cad_load(
        cad_model: *const c_char,
        out_handle: *mut *mut CadHandle,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Free a handle created by `cad_load`. `handle` may be null.
    pub fn cad_destroy(handle: *mut CadHandle);

    /// Free a string returned by the library (results and error messages).
    pub fn cad_free_string(s: *mut c_char);

    /// Return the native library version (e.g. "3.4.0"). Free with `cad_free_string`.
    pub fn cad_version() -> *mut c_char;

    /// Tessellates the model and writes the STL files into a directory.
    ///
    /// Output files are written to `dest_dir`.
    pub fn cad_tessellate(
        handle: *mut CadHandle,
        dest_dir: *const c_char,
        parameters_json: *const c_char,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Tessellates the requested faces, writing one STL file per face into a directory.
    ///
    /// Output files are written to `dest_dir`.
    pub fn cad_tessellate_faces(
        handle: *mut CadHandle,
        dest_dir: *const c_char,
        parameters_json: *const c_char,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Generates the comfort-map disc and writes the STL result into a directory.
    ///
    /// Output files are written to `dest_dir`.
    pub fn cad_generate_comfort_map_disc(
        handle: *mut CadHandle,
        dest_dir: *const c_char,
        parameters_json: *const c_char,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Computes the volume of each selected solid, and their total.
    ///
    /// Result JSON in `*out_result`, error in `*out_error`.
    pub fn cad_volume(
        handle: *mut CadHandle,
        parameters_json: *const c_char,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Computes the bounding box of the model, of each body, and of any faces given.
    ///
    /// Result JSON in `*out_result`, error in `*out_error`.
    pub fn cad_bounding_box(
        handle: *mut CadHandle,
        parameters_json: *const c_char,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Computes the center of gravity of the given faces and edges.
    ///
    /// Result JSON in `*out_result`, error in `*out_error`.
    pub fn cad_center_of_gravity(
        handle: *mut CadHandle,
        parameters_json: *const c_char,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Finds the solids that enclose a given point.
    ///
    /// Result JSON in `*out_result`, error in `*out_error`.
    pub fn cad_detect_enclosure(
        handle: *mut CadHandle,
        parameters_json: *const c_char,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Detects the rotation axis and center shared by the given faces.
    ///
    /// Result JSON in `*out_result`, error in `*out_error`.
    pub fn cad_detect_common_rotation(
        handle: *mut CadHandle,
        parameters_json: *const c_char,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Returns the colors defined on the model's solids and sheets.
    ///
    /// Result JSON in `*out_result`, error in `*out_error`.
    pub fn cad_colors(
        handle: *mut CadHandle,
        parameters_json: *const c_char,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Computes a point inside the material, for the whole model or for each solid.
    ///
    /// Result JSON in `*out_result`, error in `*out_error`.
    pub fn cad_material_points(
        handle: *mut CadHandle,
        parameters_json: *const c_char,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Approximates groups of coplanar faces with a circle or rectangle primitive.
    ///
    /// Result JSON in `*out_result`, error in `*out_error`.
    pub fn cad_planar_patch_primitives(
        handle: *mut CadHandle,
        parameters_json: *const c_char,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Computes volume and surface-area metrics for the model and its entities.
    ///
    /// Result JSON in `*out_result`, error in `*out_error`.
    pub fn cad_model_metrics(
        handle: *mut CadHandle,
        parameters_json: *const c_char,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

    /// Returns the model's entities: solids, sheets, faces, edges and vertices.
    ///
    /// Takes no parameters. Result JSON in `*out_result`, error in `*out_error`.
    pub fn cad_entities(
        handle: *mut CadHandle,
        out_result: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> c_int;

}
