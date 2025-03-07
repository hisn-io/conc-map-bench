use bustle::*;
use seize::Guard;
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
            .collector(seize::Collector::new().batch_size(BATCH_SIZE))
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
    type Handle = PinnedPapayaHandle<K, H>;

    fn with_capacity(capacity: usize) -> Self {
        let map = papaya::HashMap::builder()
            .capacity(capacity)
            .hasher(H::default())
            .resize_mode(papaya::ResizeMode::Blocking)
            .collector(seize::Collector::new().batch_size(BATCH_SIZE))
            .build();

        Self(Arc::new(map))
    }

    fn pin(&self) -> Self::Handle {
        unsafe { std::mem::transmute(PinnedPapayaHandle::new(self.0.clone())) }
    }
}

pub struct PinnedPapayaHandle<K: 'static, H: 'static> {
    map: Arc<papaya::HashMap<K, Value, H>>,
    guard: papaya::LocalGuard<'static>,
    repin_after: usize,
}

impl<K, H> PinnedPapayaHandle<K, H> {
    fn new(map: Arc<papaya::HashMap<K, Value, H>>) -> Self {
        let guard = unsafe { std::mem::transmute(map.guard()) };
        Self {
            map,
            guard,
            repin_after: 0,
        }
    }

    fn refresh(&mut self) {
        if self.repin_after == 0 {
            self.guard.refresh();
            self.repin_after = 8;
        } else {
            self.repin_after -= 1;
        }
    }
}

impl<K, H> CollectionHandle for PinnedPapayaHandle<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        let out = self.map.get(key, &self.guard).is_some();
        self.refresh();
        out
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        let out = self.map.insert(*key, 0, &self.guard).is_none();
        self.refresh();
        out
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        let out = self.map.remove(key, &self.guard).is_some();
        self.refresh();
        out
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let out = self.map.update(*key, |v| v + 1, &self.guard).is_some();
        self.refresh();
        out
    }
}
