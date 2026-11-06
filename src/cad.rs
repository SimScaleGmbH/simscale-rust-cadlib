// Code generated for the SimScale cadlib Rust bindings. DO NOT EDIT.
//! Safe Rust wrapper over the cadlib native library.

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    error::{parse_cad_error, CadLibError},
    ffi,
    models::{
        BoundingBoxParameters, BoundingBoxResponse, CenterOfGravityParameters,
        CenterOfGravityResponse, ColorsParameters, ColorsResponse, DetectCommonRotationParameters,
        DetectCommonRotationResponse, DetectEnclosureParameters, DetectEnclosureResponse,
        EntitiesResponse, GenerateComfortMapDiscParameters, GenerateComfortMapDiscResponse,
        MaterialPointsParameters, MaterialPointsResponse, ModelMetricsParameters,
        ModelMetricsResponse, PlanarPatchPrimitivesParameters, PlanarPatchPrimitivesResponse,
        TessellateFacesParameters, TessellateFacesResponse, TessellateParameters,
        TessellateResponse, VolumeParameters, VolumeResponse,
    },
    version_check::check_lib_version,
};

/// Take ownership of a C string returned by the library: copy it into an owned
/// `String`, then free the original with `cad_free_string`. `ptr` may be null.
fn take_c_string(ptr: *mut c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    // SAFETY: `ptr` is a non-null, NUL-terminated string produced by the FFI,
    // valid until we free it on the next line.
    let owned = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    // SAFETY: `ptr` was allocated by the library; hand it back to be freed.
    unsafe { ffi::cad_free_string(ptr) };
    owned
}

/// Run an FFI call that signals failure via a non-zero return code and an
/// out-error string. Returns the decoded error on failure.
fn with_error_handling(call: impl FnOnce(*mut *mut c_char) -> c_int) -> Result<(), CadLibError> {
    let mut err: *mut c_char = std::ptr::null_mut();
    if call(&mut err) != 0 {
        return Err(parse_cad_error(&take_c_string(err)));
    }
    Ok(())
}

/// Serialize optional parameters to a JSON `CString` (null pointer when `None`).
fn marshal_params<P: Serialize>(params: Option<&P>) -> Result<Option<CString>, CadLibError> {
    match params {
        None => Ok(None),
        Some(p) => {
            let json = serde_json::to_string(p).map_err(|e| CadLibError::Unexpected {
                message: format!("failed to marshal parameters: {e}"),
            })?;
            CString::new(json)
                .map(Some)
                .map_err(|e| CadLibError::Unexpected {
                    message: format!("parameters contain an interior NUL byte: {e}"),
                })
        }
    }
}

/// Run an FFI call producing a JSON result string, then deserialize it.
fn deserialize_result<R: DeserializeOwned>(
    call: impl FnOnce(*mut *mut c_char, *mut *mut c_char) -> c_int,
) -> Result<R, CadLibError> {
    let mut result: *mut c_char = std::ptr::null_mut();
    with_error_handling(|e| call(&mut result, e))?;
    let json = take_c_string(result);
    serde_json::from_str(&json).map_err(|e| CadLibError::Unexpected {
        message: format!("failed to unmarshal result: {e}"),
    })
}

/// A handle to a loaded CAD model.
///
/// Dropping a `Cad` releases the underlying native resources; there is no
/// explicit close (unlike the Go bindings, which need one). All query methods
/// are blocking — wrap them in `tokio::task::spawn_blocking` from async code.
#[derive(Debug)]
pub struct Cad {
    handle: *mut ffi::CadHandle,
}

impl Cad {
    /// Load a CAD model from the CAD data map entry of a workflow method's run
    /// request, or anything else serializing to the same shape (a
    /// `serde_json::Value`, a map, a struct of your own).
    ///
    /// Only the entry's `externalReference` is required; `dataType` and
    /// `source.type` are validated when present and other keys are ignored.
    ///
    /// ```ignore
    /// let cad = Cad::load(&request.data_map.cad)?;
    /// ```
    ///
    /// Raw JSON text has to be parsed first (`serde_json::from_str::<Value>`):
    /// a `&str` serializes to a JSON string rather than an object, which is
    /// rejected with `invalid-cad-model-json`.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] when the entry is rejected (no or
    /// malformed reference, an unsupported data type or source) and
    /// [`CadLibError::Unexpected`] when the entry cannot be serialized, contains
    /// an interior NUL byte, or the native library is incompatible with these
    /// bindings. The error codes it can return are listed in
    /// <https://github.com/SimScaleGmbH/simscale-rust-cadlib/blob/main/ERRORS.md>.
    pub fn load<T: Serialize + ?Sized>(entry: &T) -> Result<Self, CadLibError> {
        check_lib_version().map_err(|message| CadLibError::Unexpected { message })?;

        let cad_model = serde_json::to_string(entry).map_err(|e| CadLibError::Unexpected {
            message: format!("failed to marshal the cad data map entry: {e}"),
        })?;

        let c_model = CString::new(cad_model).map_err(|e| CadLibError::Unexpected {
            message: format!("cad data map entry contains an interior NUL byte: {e}"),
        })?;

        let mut handle: *mut ffi::CadHandle = std::ptr::null_mut();
        with_error_handling(|e|
            // SAFETY: `c_model` is a valid NUL-terminated string; `handle` is a
            // valid out-pointer the FFI writes on success.
            unsafe { ffi::cad_load(c_model.as_ptr(), &mut handle, e) })?;

        Ok(Self { handle })
    }
}

impl Drop for Cad {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            // SAFETY: `handle` came from `cad_load` and has not been freed yet.
            unsafe { ffi::cad_destroy(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

// `Cad` is `Send` but deliberately not `Sync`: a handle may be *moved* between
// threads (e.g. into `tokio::task::spawn_blocking`), which the native library
// tolerates. It is not `Sync` because the query methods take `&self` and the
// native state is not safe to call concurrently through a shared reference.
// SAFETY: the library keeps no thread-affine state tied to a handle.
unsafe impl Send for Cad {}

// ============================================================================
// Generated query methods
// ============================================================================

impl Cad {
    /// Tessellates the model and writes the STL files into a directory.
    ///
    /// One STL file is written per region by default, or one per face when
    /// ``write_one_file_per_face`` is set. The directory is created if it does not
    /// exist, and files in it with the same names are overwritten.
    ///
    /// ``tessellation_type``, ``max_facet_width`` and ``allowed_facet_faults_level``
    /// apply to Parasolid models, and ``tessellation_type`` has to be set explicitly;
    /// ``feature_size``, ``adaptivity`` and ``max_edge_length`` apply to nTop models
    /// and all default when omitted. ``tess_surface_area`` is set in the response
    /// only when ``calculate_area`` is requested.
    ///
    /// Supported for Parasolid, STL and nTop models.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if `dest_dir` contains an interior NUL byte,
    /// the parameters cannot be serialized, or the library fails internally.
    ///
    /// # Query errors
    ///
    /// * `tessellation-has-error[-with-entity]` - The kernel could not tessellate one or more entities.
    /// * `tessellation-ntop-negative-adaptivity` - `adaptivity` is negative.
    /// * `tessellation-ntop-zero-edge-length` - `max_edge_length` is zero.
    /// * `tessellation-ntop-zero-feature-size` - `feature_size` is zero.
    /// * `tessellation-too-large` - The result could not be written; the kernel ran out of disk space.
    pub fn tessellate(
        &self,
        dest_dir: &str,
        params: Option<&TessellateParameters>,
    ) -> Result<TessellateResponse, CadLibError> {
        let c_dest = CString::new(dest_dir).map_err(|e| CadLibError::Unexpected {
            message: format!("dest_dir contains an interior NUL byte: {e}"),
        })?;
        let params_json = marshal_params(params)?;
        let params_ptr = params_json
            .as_ref()
            .map_or(std::ptr::null(), |c| c.as_ptr());
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `c_dest`/`params_ptr` are
            // valid (or null) NUL-terminated strings; `r`/`e` are out-pointers.
            unsafe { ffi::cad_tessellate(self.handle, c_dest.as_ptr(), params_ptr, r, e) })
    }

    /// Tessellates the requested faces, writing one STL file per face into a directory.
    ///
    /// Each requested face is tessellated into its own STL file, named after the
    /// face id; a face requested more than once is tessellated only once. The
    /// response additionally carries the normal of each face, evaluated at the
    /// mid point of the face.
    ///
    /// The faces to tessellate are required, so a query without parameters is
    /// rejected. The directory is created if it does not exist, and files in it with
    /// the same names are overwritten.
    ///
    /// Supported only for Parasolid models. Other formats fail with
    /// ``query-type-unsupported-for-cad-format``.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if `dest_dir` contains an interior NUL byte,
    /// the parameters cannot be serialized, or the library fails internally.
    ///
    /// # Query errors
    ///
    /// * `tessellation-has-error` - The kernel could not tessellate one of the faces.
    pub fn tessellate_faces(
        &self,
        dest_dir: &str,
        params: Option<&TessellateFacesParameters>,
    ) -> Result<TessellateFacesResponse, CadLibError> {
        let c_dest = CString::new(dest_dir).map_err(|e| CadLibError::Unexpected {
            message: format!("dest_dir contains an interior NUL byte: {e}"),
        })?;
        let params_json = marshal_params(params)?;
        let params_ptr = params_json
            .as_ref()
            .map_or(std::ptr::null(), |c| c.as_ptr());
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `c_dest`/`params_ptr` are
            // valid (or null) NUL-terminated strings; `r`/`e` are out-pointers.
            unsafe { ffi::cad_tessellate_faces(self.handle, c_dest.as_ptr(), params_ptr, r, e) })
    }

    /// Generates the comfort-map disc and writes the STL result into a directory.
    ///
    /// The disc is a tessellated surface covering a region of interest: a circle of
    /// radius ``floor_radius`` around ``center``, placed ``height`` above the ground.
    /// ``fineness`` controls how finely it is tessellated.
    ///
    /// By default the disc is written as a single file. Asking for one of the zipped
    /// encodings in ``result_encoding`` unpacks the result into the directory instead.
    /// The directory is created if it does not exist, and files in it with
    /// the same names are overwritten.
    ///
    /// A disc with no geometry is reported as ``empty-comfort-map-disc``.
    ///
    /// Supported for Parasolid and STL models. Other formats fail with
    /// ``query-type-unsupported-for-cad-format``.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if `dest_dir` contains an interior NUL byte,
    /// the parameters cannot be serialized, or the library fails internally.
    ///
    /// # Query errors
    ///
    /// * `empty-comfort-map-disc` - The query returned no geometry.
    /// * `lawson-cylinder-exceeds-size-box` - `floor_radius` is too large for the model's size box.
    /// * `lawson-negative-height-above-ground` - `height` is negative.
    /// * `lawson-roi-radius-must-be-positive` - `floor_radius` is zero or negative.
    /// * `lawson-tessellation-max-width-must-be-positive` - `fineness` is zero or negative.
    /// * `lawson-tessellation-too-fine` - `fineness` produces more than 100 million triangles, which the query does not allow.
    /// * `tessellation-has-error[-with-entity]` - The kernel could not tessellate one or more entities.
    /// * `tessellation-too-large` - The result could not be written; the kernel ran out of disk space.
    pub fn generate_comfort_map_disc(
        &self,
        dest_dir: &str,
        params: Option<&GenerateComfortMapDiscParameters>,
    ) -> Result<GenerateComfortMapDiscResponse, CadLibError> {
        let c_dest = CString::new(dest_dir).map_err(|e| CadLibError::Unexpected {
            message: format!("dest_dir contains an interior NUL byte: {e}"),
        })?;
        let params_json = marshal_params(params)?;
        let params_ptr = params_json
            .as_ref()
            .map_or(std::ptr::null(), |c| c.as_ptr());
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `c_dest`/`params_ptr` are
            // valid (or null) NUL-terminated strings; `r`/`e` are out-pointers.
            unsafe { ffi::cad_generate_comfort_map_disc(self.handle, c_dest.as_ptr(), params_ptr, r, e) })
    }

    /// Computes the volume of each selected solid, and their total.
    ///
    /// The solids and sheets to measure are given by their entity ids, as
    /// ``entities`` returns them.
    ///
    /// ``volumes`` holds one entry per measured entity, keyed by its entity id,
    /// and ``total`` is their sum.
    ///
    /// Supported for Parasolid and nTop models. Other formats fail with
    /// ``query-type-unsupported-for-cad-format``.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if the parameters cannot be serialized or the
    /// library fails internally.
    pub fn volume(&self, params: Option<&VolumeParameters>) -> Result<VolumeResponse, CadLibError> {
        let params_json = marshal_params(params)?;
        let params_ptr = params_json
            .as_ref()
            .map_or(std::ptr::null(), |c| c.as_ptr());
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `params_ptr` is null or a
            // valid NUL-terminated JSON string; `r`/`e` are valid out-pointers.
            unsafe { ffi::cad_volume(self.handle, params_ptr, r, e) })
    }

    /// Computes the bounding box of the model, of each body, and of any faces given.
    ///
    /// The box of the whole model and one box per solid and sheet are always
    /// returned. Naming faces additionally returns one box per face, keyed by
    /// face id, and a single box enclosing all of them.
    ///
    /// Supported for Parasolid and nTop models. Other formats fail with
    /// ``query-type-unsupported-for-cad-format``.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if the parameters cannot be serialized or the
    /// library fails internally.
    pub fn bounding_box(
        &self,
        params: Option<&BoundingBoxParameters>,
    ) -> Result<BoundingBoxResponse, CadLibError> {
        let params_json = marshal_params(params)?;
        let params_ptr = params_json
            .as_ref()
            .map_or(std::ptr::null(), |c| c.as_ptr());
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `params_ptr` is null or a
            // valid NUL-terminated JSON string; `r`/`e` are valid out-pointers.
            unsafe { ffi::cad_bounding_box(self.handle, params_ptr, r, e) })
    }

    /// Computes the center of gravity of the given faces and edges.
    ///
    /// The selection accepts faces and edges only; any other entity type fails
    /// the query. One center of gravity is computed for the selection as a
    /// whole, not one per selected entity.
    ///
    /// Supported only for Parasolid models. Other formats fail with
    /// ``query-type-unsupported-for-cad-format``.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if the parameters cannot be serialized or the
    /// library fails internally.
    ///
    /// # Query errors
    ///
    /// * `compute-cog-entity-not-supported` - A selected entity has unsupported type, it is neither a face nor an edge.
    pub fn center_of_gravity(
        &self,
        params: Option<&CenterOfGravityParameters>,
    ) -> Result<CenterOfGravityResponse, CadLibError> {
        let params_json = marshal_params(params)?;
        let params_ptr = params_json
            .as_ref()
            .map_or(std::ptr::null(), |c| c.as_ptr());
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `params_ptr` is null or a
            // valid NUL-terminated JSON string; `r`/`e` are valid out-pointers.
            unsafe { ffi::cad_center_of_gravity(self.handle, params_ptr, r, e) })
    }

    /// Finds the solids that enclose a given point.
    ///
    /// The response holds the id of every solid that contains it.
    ///
    /// Supported only for Parasolid models. Other model formats fail with
    /// ``query-type-unsupported-for-cad-format``.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if the parameters cannot be serialized or the
    /// library fails internally.
    pub fn detect_enclosure(
        &self,
        params: Option<&DetectEnclosureParameters>,
    ) -> Result<DetectEnclosureResponse, CadLibError> {
        let params_json = marshal_params(params)?;
        let params_ptr = params_json
            .as_ref()
            .map_or(std::ptr::null(), |c| c.as_ptr());
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `params_ptr` is null or a
            // valid NUL-terminated JSON string; `r`/`e` are valid out-pointers.
            unsafe { ffi::cad_detect_enclosure(self.handle, params_ptr, r, e) })
    }

    /// Detects the rotation axis and center shared by the given faces.
    ///
    /// All the given faces must be rotationally aligned about a single axis; the
    /// query fails if they are not, or if no axis can be determined. The response
    /// gives the direction of that axis and a point on it.
    ///
    /// Supported only for Parasolid models. Other formats fail with
    /// ``query-type-unsupported-for-cad-format``.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if the parameters cannot be serialized or the
    /// library fails internally.
    ///
    /// # Query errors
    ///
    /// * `detect-rotation-axis-not-consistent-for-all-faces` - The selected faces do not share the same rotation axis.
    /// * `detect-rotation-could-not-detect-axis` - No rotation axis could be detected for the selected faces.
    /// * `detect-rotation-flat-faces-selected[-with-entity]` - The selection contains flat faces, which have no rotation axis.
    /// * `detect-rotation-multiple-possible-directions` - More than one rotation direction fits the selected faces.
    pub fn detect_common_rotation(
        &self,
        params: Option<&DetectCommonRotationParameters>,
    ) -> Result<DetectCommonRotationResponse, CadLibError> {
        let params_json = marshal_params(params)?;
        let params_ptr = params_json
            .as_ref()
            .map_or(std::ptr::null(), |c| c.as_ptr());
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `params_ptr` is null or a
            // valid NUL-terminated JSON string; `r`/`e` are valid out-pointers.
            unsafe { ffi::cad_detect_common_rotation(self.handle, params_ptr, r, e) })
    }

    /// Returns the colors defined on the model's solids and sheets.
    ///
    /// The query takes no parameters and covers the whole model. Each entry names
    /// the entity the color is defined on and the color itself, as a hex string
    /// such as ``#ff0000``.
    ///
    /// Supported for Parasolid and nTop models. Other formats fail with
    /// ``query-type-unsupported-for-cad-format``.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if the parameters cannot be serialized or the
    /// library fails internally.
    pub fn colors(&self, params: Option<&ColorsParameters>) -> Result<ColorsResponse, CadLibError> {
        let params_json = marshal_params(params)?;
        let params_ptr = params_json
            .as_ref()
            .map_or(std::ptr::null(), |c| c.as_ptr());
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `params_ptr` is null or a
            // valid NUL-terminated JSON string; `r`/`e` are valid out-pointers.
            unsafe { ffi::cad_colors(self.handle, params_ptr, r, e) })
    }

    /// Computes a point inside the material, for the whole model or for each solid.
    ///
    /// ``query_type`` chooses between the two. The response sets the field that
    /// matches it and leaves the other unset; the per-body field is keyed by
    /// entity id.
    ///
    /// A material point is only guaranteed to lie inside the material. Which
    /// interior point is returned is unspecified, and can differ between runs.
    ///
    /// Supported only for Parasolid models. Other formats fail with
    /// ``query-type-unsupported-for-cad-format``.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if the parameters cannot be serialized or the
    /// library fails internally.
    pub fn material_points(
        &self,
        params: Option<&MaterialPointsParameters>,
    ) -> Result<MaterialPointsResponse, CadLibError> {
        let params_json = marshal_params(params)?;
        let params_ptr = params_json
            .as_ref()
            .map_or(std::ptr::null(), |c| c.as_ptr());
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `params_ptr` is null or a
            // valid NUL-terminated JSON string; `r`/`e` are valid out-pointers.
            unsafe { ffi::cad_material_points(self.handle, params_ptr, r, e) })
    }

    /// Approximates groups of coplanar faces with a circle or rectangle primitive.
    ///
    /// Groups the requested faces into patches of adjacent, coplanar faces and
    /// approximates each patch with the primitive — a circle or a rectangle — whose
    /// area is closest to the area of the patch. All faces of a patch belong to the
    /// same occurrence. The query fails if any of the requested faces is not planar.
    ///
    /// Supported only for Parasolid models. Other formats fail with
    /// ``query-type-unsupported-for-cad-format``.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if the parameters cannot be serialized or the
    /// library fails internally.
    ///
    /// # Query errors
    ///
    /// * `planar-patch-primitives-degenerate-patch[-with-entity]` - A face has no interior point, or a patch is not two-dimensional, so no primitive fits it.
    /// * `planar-patch-primitives-non-coplanar-faces[-with-entity]` - Faces that are adjacent, and so belong to the same patch, lie on different planes.
    /// * `planar-patch-primitives-non-planar-faces[-with-entity]` - One or more of the requested faces is not planar.
    pub fn planar_patch_primitives(
        &self,
        params: Option<&PlanarPatchPrimitivesParameters>,
    ) -> Result<PlanarPatchPrimitivesResponse, CadLibError> {
        let params_json = marshal_params(params)?;
        let params_ptr = params_json
            .as_ref()
            .map_or(std::ptr::null(), |c| c.as_ptr());
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `params_ptr` is null or a
            // valid NUL-terminated JSON string; `r`/`e` are valid out-pointers.
            unsafe { ffi::cad_planar_patch_primitives(self.handle, params_ptr, r, e) })
    }

    /// Computes volume and surface-area metrics for the model and its entities.
    ///
    /// Computes aggregated volume and surface area for the model, along with
    /// per-solid, per-sheet, per-face and per-edge metrics.
    ///
    /// The query takes no parameters: the metrics always cover the whole model.
    ///
    /// Supported for Parasolid and nTop models. Other formats fail with
    /// ``query-type-unsupported-for-cad-format``.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if the parameters cannot be serialized or the
    /// library fails internally.
    pub fn model_metrics(
        &self,
        params: Option<&ModelMetricsParameters>,
    ) -> Result<ModelMetricsResponse, CadLibError> {
        let params_json = marshal_params(params)?;
        let params_ptr = params_json
            .as_ref()
            .map_or(std::ptr::null(), |c| c.as_ptr());
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `params_ptr` is null or a
            // valid NUL-terminated JSON string; `r`/`e` are valid out-pointers.
            unsafe { ffi::cad_model_metrics(self.handle, params_ptr, r, e) })
    }

    /// Returns the model's entities: solids, sheets, faces, edges and vertices.
    ///
    /// The list is flat: each entity references its parents and children by their
    /// 0-based index in it. Solids and sheets are the top-level entities.
    ///
    /// Each entity carries the entity id that the other queries take as input.
    ///
    /// Supported for Parasolid, STL and nTop models.
    ///
    /// # Errors
    ///
    /// Returns [`CadLibError::Expected`] for a known domain error and
    /// [`CadLibError::Unexpected`] if the library fails internally.
    pub fn entities(&self) -> Result<EntitiesResponse, CadLibError> {
        deserialize_result(|r, e|
            // SAFETY: `self.handle` is a live handle; `r`/`e` are valid out-pointers.
            unsafe { ffi::cad_entities(self.handle, r, e) })
    }
}
