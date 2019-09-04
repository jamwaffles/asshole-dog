pub mod add_timestamp;

#[derive(serde_derive::Serialize, Debug, Clone)]
struct OkResponse {
    ok: bool,
}

impl Default for OkResponse {
    fn default() -> Self {
        Self { ok: true }
    }
}
