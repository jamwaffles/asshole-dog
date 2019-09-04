pub mod add_timestamp;
pub mod get_timestamps;

#[derive(serde_derive::Serialize, Debug, Clone)]
struct OkResponse {
    ok: bool,
}

impl Default for OkResponse {
    fn default() -> Self {
        Self { ok: true }
    }
}
