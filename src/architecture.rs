use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    Aarch64,
    Armhf,
    Riscv32,
    Riscv64,
    X86,
    X86_64,
}

impl Default for Architecture {
    fn default() -> Self {
        Self::X86_64
    }
}
