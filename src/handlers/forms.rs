use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteForm {
    pub verify: String,
}