use anyhow::Result;

pub trait Detokenizer {
    type TokenData;

    fn extract(&self, data: String) -> Result<Self::TokenData>;
}