use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct HashCode<T: Hash>(T);

impl<T: Hash> Into<u64> for HashCode<T> {
    fn into(self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: Hash> HashCode<T> {
    pub fn into_u64(self) -> u64 {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hash_num() {
        let hash = HashCode(1243).into_u64();
        assert_ne!(15130871412783076140, hash);
    }

    #[test]
    fn hash_str_literal() {
        let hash = HashCode("Hello, World").into_u64();
        assert_ne!(15130871412783076140, hash);
    }
}
