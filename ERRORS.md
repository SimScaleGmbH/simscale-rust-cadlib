# Errors

Every binding surfaces two kinds of failure:

- **Expected** errors carry a stable `error_code` and optional structured `error_details`. They
  describe something about the request or the model, and are meant to be matched on.
- **Unexpected** errors carry only a message. They mean a bug or an environment problem — a failed
  HTTP request, a serialization failure, a panic caught at the FFI boundary.

| Binding | Expected | Unexpected |
|---------|----------|------------|
| Rust | `CadLibError::Expected` | `CadLibError::Unexpected` |
| Go | `*cadlib.CadExpectedError` (`ErrorCode`, `ErrorDetails`) | `*cadlib.CadUnexpectedError` |
| Python | `CadExpectedError` (`error_code`, `error_details`) | `CadUnexpectedError` |
| C | `CAD_ERROR_EXPECTED` + JSON in `out_error` | `CAD_ERROR_RUNTIME` and the other codes |

In Go, a failure in the bindings to marshal an argument or unmarshal a result is returned as a plain
`error` wrapping the `encoding/json` error, not as a `*cadlib.CadUnexpectedError`.

Only expected errors carry an error code, so those are the ones listed below, grouped by the call that
raises them.

## Error codes

One row per code. Which method can raise which is documented on the method itself; this table is for looking a code up once you have one.

### Raised while loading a CAD model

`load` takes the CAD data map entry of a workflow method's run request. These are the codes it can
raise, in the order they are checked.

| `error_code` | Explanation | `error_details` |
|--------------|-------------|-----------------|
| `invalid-cad-model-json` | The entry did not serialize to a JSON object, or one of its fields has the wrong type. | `error` (parser message), `hint` (present when the payload is not an object at all) |
| `unsupported-cad-model-data-type` | The entry declares a `dataType` this library is not compatible with. Only `simscale.cad:cad-model` at major version 1 is accepted. | `dataType` (verbatim), `expected` |
| `unsupported-cad-model-source-type` | The entry declares a `source.type` other than `EXTERNAL`. | `sourceType` (verbatim), `expected` |
| `unsupported-cad-model-source` | The entry points at data on the local filesystem (`path`). Loading CAD from a local file is not implemented. | `path`, `reason` |
| `unsupported-parametric-cad-model` | The entry holds parametric data (`parameterValueCombinations`), which cannot be loaded as a single CAD model. | `combinations` (count), `reason` |
| `invalid-cad-model` | The entry carries no external reference, or the reference does not hold both `cad-model.cad-id` and `cad-model.state-id` as UUIDs. | `keys` + `expected` when no reference was found; `reference` + `missing` when a reference was malformed |

> **Troubleshooting: `invalid-cad-model-json` on JSON text that looks valid.** In Rust and Go
> `load` serializes whatever it is handed, so a `String` serializes to a JSON *string*, not the
> object `load` expects — the text is never parsed. Parse it first
> (`serde_json::from_str::<Value>`) or wrap it (`json.RawMessage`). Python's `Cad.load` takes text
> directly, so this does not apply there.

### Raised by the library during a query

Which codes you can get depends on the model format — Parasolid, STL or nTop — because a query is
only implemented for some of them:

| Query | Parasolid | STL | nTop |
|-------|:---------:|:---:|:----:|
| `tessellate` | ● | ● | ● |
| `volume`, `bounding_box`, `model_metrics`, `colors` | ● | | ● |
| `generate_comfort_map_disc` | ● | ● | |
| `center_of_gravity`, `detect_enclosure`, `detect_common_rotation`, `material_points`, `planar_patch_primitives`, `tessellate_faces` | ● | | |
| `entities` | ● | ● | ● |


**Codes carry a suffix when they name what went wrong.** When an error can point at the entities
responsible, `-with-entity` is appended to the code: `wrong-entity-type` arrives as
`wrong-entity-type-with-entity`. `-with-numbers`, `-with-text` and `-with-positions` behave the same
way and stack in that order.

The suffix is only added when there is something to attach, so the same failure can arrive with or
without it — a flat-face selection that reports no entities stays `detect-rotation-flat-faces-selected`.
The tables write that as `detect-rotation-flat-faces-selected[-with-entity]`: the bracketed part may
or may not be there. **Match on the base code as a prefix**, not on the whole string.

Two tables follow: codes that several queries share, then codes specific to a single query.

#### Shared by several queries

| `error_code` | Applies to | Explanation | `error_details` | Parasolid | STL | nTop |
|--------------|------------|-------------|-----------------|:---------:|:---:|:----:|
| `action-interrupted-by-user` | every query | The query was interrupted. | — | ● | ● | ● |
| `action-timed-out` | every query | The query ran past its time limit and was killed. | — | ● | ● | ● |
| `cad-query-out-of-memory` | every query | The kernel ran out of memory on this query. | — | ● | ● | ● |
| `cant-query-empty-state` | every query | The CAD state has no successful operation, so there is nothing to query. | — | ● | ● | ● |
| `cant-query-failed-operation` | every query | The CAD state's last operation failed, so there is nothing to query. | — | ● | ● | ● |
| `entity-not-found[-with-entity]` | queries with input entities | An entity id in the input is well formed but no such entity is in the model. | entities | ● | ● | ● |
| `invalid-entity-id[-with-entity]` | queries with input entities | An entity id in the input is malformed. | entities | ● | ● | ● |
| `invalid-parameter` | every query | A field in the request is malformed. | — | ● | ● | ● |
| `invalid-parameters[-with-entity]` | queries with input entities | A selected entity cannot be used for this query. | entities | ● | | |
| `invalid-parameters[-with-text]` | queries with parameters | A parameter in the request holds an unsupported value; the text names the field. | text | ● | | |
| `invalid-selection-not-sheet-body[-with-entity]` | queries with input entities | The selection contains a solid where a sheet is required. | entities | ● | | |
| `malformed-json` | every query | The request or its parameters could not be parsed. Also raised when a parameter string holds control characters or script-like content. | — | ● | ● | ● |
| `missing-auth-token` | every query | `SIMSCALE_USER_TOKEN_FILE` is unset or the file cannot be read. | — | ● | ● | ● |
| `mixed-solid-regions-occurrences` | queries with input entities | The selection mixes solids with whole solid bodies, which would double-count. | — | ● | | |
| `no-bodies-selected` | queries with input entities | `entity_ids` is empty. | — | ● | | ● |
| `no-entities-selected` | queries with input entities | `entities` is empty. | — | ● | | |
| `no-faces-selected` | queries with input entities | `faces` is empty. | — | ● | | |
| `occurrence-id-not-found[-with-entity]` | queries with input entities | The entity ids in input belong to a body that could not be found in the model. | entities | ● | | |
| `query-type-unsupported-for-cad-format` | every query | The query is not implemented for this model format. | — | | ● | ● |
| `simgeo-crashed` | every query | The kernel crashed while running the query. | — | ● | ● | ● |
| `wrong-entity-type[-with-entity]` | queries with input entities | An entity id in input refers to an entity type this query does not accept. | entities | ● | | ● |


#### Specific to one query


| `error_code` | Applies to | Explanation | `error_details` | Parasolid | STL | nTop |
|--------------|------------|-------------|-----------------|:---------:|:---:|:----:|
| `compute-cog-entity-not-supported` | `center_of_gravity` | A selected entity has unsupported type: it is neither a face nor an edge. | — | ● | | |
| `detect-rotation-axis-not-consistent-for-all-faces` | `detect_common_rotation` | The selected faces do not share the same rotation axis. | — | ● | | |
| `detect-rotation-could-not-detect-axis` | `detect_common_rotation` | No rotation axis could be detected for the selected faces. | — | ● | | |
| `detect-rotation-flat-faces-selected[-with-entity]` | `detect_common_rotation` | The selection contains flat faces, which have no rotation axis. | entities | ● | | |
| `detect-rotation-multiple-possible-directions` | `detect_common_rotation` | More than one rotation direction fits the selected faces. | — | ● | | |
| `empty-comfort-map-disc` | `generate_comfort_map_disc` | The query returned no geometry. | — | ● | ● | |
| `lawson-cylinder-exceeds-size-box` | `generate_comfort_map_disc` | `floor_radius` is too large for the model's size box. | — | ● | ● | |
| `lawson-negative-height-above-ground` | `generate_comfort_map_disc` | `height` is negative. | — | ● | ● | |
| `lawson-roi-radius-must-be-positive` | `generate_comfort_map_disc` | `floor_radius` is zero or negative. | — | ● | ● | |
| `lawson-tessellation-max-width-must-be-positive` | `generate_comfort_map_disc` | `fineness` is zero or negative. | — | ● | ● | |
| `lawson-tessellation-too-fine` | `generate_comfort_map_disc` | `fineness` produces more than 100 million triangles, which the query does not allow. | — | | ● | |
| `planar-patch-primitives-degenerate-patch[-with-entity]` | `planar_patch_primitives` | A face has no interior point, or a patch is not two-dimensional, so no primitive fits it. | entities | ● | | |
| `planar-patch-primitives-non-coplanar-faces[-with-entity]` | `planar_patch_primitives` | Faces that are adjacent, and so belong to the same patch, lie on different planes. | entities | ● | | |
| `planar-patch-primitives-non-planar-faces[-with-entity]` | `planar_patch_primitives` | One or more of the requested faces is not planar. | entities | ● | | |
| `tessellation-has-error[-with-entity]` | `tessellate`, `generate_comfort_map_disc`, `tessellate_faces` | The kernel could not tessellate one or more entities. | entities | ● | ● | |
| `tessellation-ntop-negative-adaptivity` | `tessellate` | `adaptivity` is negative. | — | | | ● |
| `tessellation-ntop-zero-edge-length` | `tessellate` | `max_edge_length` is zero. | — | | | ● |
| `tessellation-ntop-zero-feature-size` | `tessellate` | `feature_size` is zero. | — | | | ● |
| `tessellation-too-large` | `tessellate`, `generate_comfort_map_disc` | The result could not be written; the kernel ran out of disk space. | — | ● | ● | |


## C error codes

These belong to the C ABI — the binary interface the Rust, Go and Python bindings are built on. You
only deal with them when calling the native library directly; each binding turns them into its own
error type instead, as listed at the top of this page.

| Constant | Value | Go constant | Meaning |
|----------|-------|-------------|---------|
| `CAD_OK` | 0 | `CadLibOK` | Success |
| `CAD_ERROR_INVALID_UUID` | -1 | `CadLibErrorInvalidUUID` | Malformed UUID |
| `CAD_ERROR_NULL_POINTER` | -2 | `CadLibErrorNullPointer` | A required pointer argument was null |
| `CAD_ERROR_INVALID_UTF8` | -3 | `CadLibErrorInvalidUTF8` | A string argument was not valid UTF-8 |
| `CAD_ERROR_INVALID_JSON` | -4 | `CadLibErrorInvalidJSON` | A JSON argument could not be parsed |
| `CAD_ERROR_EXPECTED` | -5 | `CadLibErrorExpected` | An expected error; `out_error` holds its JSON |
| `CAD_ERROR_RUNTIME` | -6 | `CadLibErrorRuntime` | An unexpected error, including a caught panic |

On `CAD_ERROR_EXPECTED` and `CAD_ERROR_RUNTIME`, `*out_error` holds an owned JSON string that the
caller frees with `cad_free_string`:

```json
{"kind": "expected", "error_code": "invalid-cad-model", "error_details": {"keys": ["metadata"], "expected": "externalReference"}}
```
