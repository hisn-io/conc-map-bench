use bustle::*;
use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use super::Value;

const BATCH_SIZE: usize = 2000;

#[derive(Clone)]
pub struct PapayaTable<K: 'static, H: 'static>(Arc<papaya::HashMap<K, Value, H>>);

impl<K, H> Collection for PapayaTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        let map = papaya::HashMap::builder()
            .capacity(capacity)
            .hasher(H::default())
            .resize_mode(papaya::ResizeMode::Blocking)
            .collector(
                seize::Collector::new()
                    //.epoch_frequency(None)
                    .batch_size(BATCH_SIZE),
            )
            .build();

        Self(Arc::new(map))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for PapayaTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.pin().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.pin().insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.pin().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.pin().update(*key, |v| v + 1).is_some()
    }
}

#[derive(Clone)]
pub struct PinnedPapayaTable<K: 'static, H: 'static>(Arc<papaya::HashMap<K, Value, H>>);

impl<K, H> Collection for PinnedPapayaTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = PinnedPapayaHandle<'static, K, H>;

    fn with_capacity(capacity: usize) -> Self {
        let map = papaya::HashMap::builder()
            .capacity(capacity)
            .hasher(H::default())
            .resize_mode(papaya::ResizeMode::Blocking)
            .collector(
                seize::Collector::new()
                    //.epoch_frequency(None)
                    .batch_size(BATCH_SIZE),
            )
            .build();

        Self(Arc::new(map))
    }

    fn pin(&self) -> Self::Handle {
        unsafe {std::mem::transmute(PinnedPapayaHandle(self.0.pin()))}
    }
}

pub struct PinnedPapayaHandle<'a, K: 'static, H: 'static>(papaya::HashMapRef<'a, K, Value, H, papaya::LocalGuard<'a>>);

impl<'a, K, H> CollectionHandle for PinnedPapayaHandle<'a, K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.update(*key, |v| v + 1).is_some()
    }
}
