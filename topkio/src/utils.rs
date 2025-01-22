// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::primitives::{ChunkResponse, Content, Part, Structure};

pub(crate) fn parse_chunk(chunk: &str) -> Result<ChunkResponse, serde_json::Error> {
    // 1. Remove "data: " prefix (if present)
    let data_str = chunk.strip_prefix("data: ").unwrap_or(chunk);

    // 2. Find the end of the JSON data (if there are trailing characters)
    let end_index = data_str.find('\n').unwrap_or(data_str.len());
    let json_str = &data_str[..end_index];

    // 3. Deserialize the JSON string
    let result: ChunkResponse = serde_json::from_str(json_str)?;

    Ok(result)
}

pub(crate) fn build_structure(text: &str) -> Structure {
    let part = Part {
        text: text.to_string(),
    };
    let content = Content {
        parts: vec![part],
        role: "user".to_string(),
    };
    Structure {
        contents: vec![content],
    }
}
