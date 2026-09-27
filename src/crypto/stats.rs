//! Cryptography statistics

#[derive(Debug, Clone, Default)]
pub struct CryptoStats {
    pub operations_total: u64,
    pub errors: u64,
}
