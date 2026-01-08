use std::fmt;

use serde::Deserialize;
use zeroize::Zeroizing;

#[derive(Deserialize, Clone)]
#[serde(transparent)]
pub struct Secret(Zeroizing<String>);

impl Secret {
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "*************")
    }
}
