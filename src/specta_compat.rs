//! Manual `specta::Type` impls that work around rc.24 limitations.
//!
//! specta-rc.24's `serde_json::Value` `Type` impl is `inline: true` and
//! recursive (`Vec<Value>`, `Map<String, Value>`), so any field typed as
//! `serde_json::Value` emits an inline shape that references an undefined
//! `Value` name -- the export pass filters the type out because
//! `inline: true` makes `requires_reference` return false.
//!
//! Until that's patched upstream, [`JsonValue`] gives consumers a properly
//! named recursive TS alias. The Rust type is empty and never instantiated;
//! it exists only to be referenced through `#[specta(type = ...)]` overrides.

use std::borrow::Cow;

use specta::datatype::{DataType, NamedDataType};

/// Stand-in for `serde_json::Value` in `specta::Type` overrides.
///
/// Emits as a top-level recursive TS alias:
///
/// ```ts
/// export type JsonValue =
///     | null
///     | boolean
///     | number
///     | string
///     | JsonValue[]
///     | { [key: string]: JsonValue };
/// ```
///
/// Use it via `#[specta(type = ::geojson::specta_compat::JsonValue)]` on any
/// field whose runtime type is `serde_json::Value`. Runtime serde behavior
/// is unchanged -- only the typegen view is overridden.
pub struct JsonValue;

impl specta::Type for JsonValue {
    fn definition(types: &mut specta::Types) -> DataType {
        const NAME: &str = "JsonValue";
        const SENTINEL: &str = "geojson::specta_compat::JsonValue";

        let reference = NamedDataType::init_with_sentinel(
            &[],
            vec![],
            false,
            types,
            SENTINEL,
            |_types, ndt| {
                ndt.set_name(Cow::Borrowed(NAME));
                ndt.set_module_path(Cow::Borrowed(module_path!()));
                ndt.set_ty(DataType::Reference(specta_typescript::define(
                    "null | boolean | number | string | JsonValue[] | { [key: string]: JsonValue }",
                )));
            },
        );
        DataType::Reference(reference)
    }
}
