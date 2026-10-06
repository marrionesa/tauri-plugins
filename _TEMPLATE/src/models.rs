// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Request payload for `ping`. Field names are camelCase on the JS side.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PingRequest {
    pub value: String,
}

/// Response payload for `ping`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PingResponse {
    pub value: String,
}
