pub mod merkle;

pub trait Database {
    fn get(&self, key: &[u8]) -> &[u8];
    fn get_by_hash(&self, hash: H256) -> (&[u8], &[u8]);
}
