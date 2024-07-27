/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// Needed for Clippy to accept #[cfg(all())]
#![allow(clippy::non_minimal_cfg)]

use godot::classes::{EditorExportPlugin, IEditorExportPlugin};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=EditorExportPlugin, init)]
struct KeywordParameterEditorExportPlugin {
    _base: Base<EditorExportPlugin>,
}

#[godot_api]
impl IEditorExportPlugin for KeywordParameterEditorExportPlugin {
    // This test requires that the second non-self parameter on `export_file`
    // remain named `_type`. Additionally tell rustfmt to skip this code and
    // allow all clippy lints.
    #[allow(clippy::all)]
    #[rustfmt::skip]
    fn export_file(&mut self, _path: GString, _type: GString, _features: PackedStringArray) { }
}
