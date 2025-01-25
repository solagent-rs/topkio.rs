use crate::primitives::{ChunkResponse, GenerateContentResponse};

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

pub(crate) fn gemini_parse_chunk(
    chunk: &str,
) -> Result<GenerateContentResponse, serde_json::Error> {
    // 1. Remove "data: " prefix (if present)
    let data_str = chunk.strip_prefix("[").unwrap_or(chunk);
    let data_str = chunk.strip_suffix("]").unwrap_or(data_str);
    let data_str = chunk.strip_prefix(",").unwrap_or(data_str);
    let data_str = chunk.strip_suffix(",").unwrap_or(data_str);

    // 2. Deserialize the JSON string
    let result: GenerateContentResponse = serde_json::from_str(data_str)?;

    Ok(result)
}
