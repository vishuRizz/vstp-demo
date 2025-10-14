use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

/// A generic object pool for reusing allocations
#[derive(Debug)]
pub struct Pool<T> {
    items: Arc<Mutex<VecDeque<T>>>,
    max_size: usize,
}

impl<T> Pool<T> {
    /// Create a new pool with the specified maximum size
    pub fn new(max_size: usize) -> Self {
        Self {
            items: Arc::new(Mutex::new(VecDeque::with_capacity(max_size))),
            max_size,
        }
    }

    /// Get an item from the pool or create a new one
    pub async fn get<F>(&self, create: F) -> T
    where
        F: FnOnce() -> T,
    {
        let mut items = self.items.lock().await;
        items.pop_front().unwrap_or_else(create)
    }

    /// Return an item to the pool
    pub async fn put(&self, item: T) {
        let mut items = self.items.lock().await;
        if items.len() < self.max_size {
            items.push_back(item);
        }
    }

    /// Get the current number of items in the pool
    pub async fn len(&self) -> usize {
        let items = self.items.lock().await;
        items.len()
    }

    /// Check if the pool is empty
    pub async fn is_empty(&self) -> bool {
        let items = self.items.lock().await;
        items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_basic() {
        let pool = Pool::new(2);

        // Get items
        let item1: Vec<i32> = pool.get(|| Vec::new()).await;
        let item2: Vec<i32> = pool.get(|| Vec::new()).await;

        // Return items
        pool.put(item1).await;
        pool.put(item2).await;

        assert_eq!(pool.len().await, 2);
    }

    #[tokio::test]
    async fn test_pool_max_size() {
        let pool = Pool::new(1);

        // Get and return two items
        let item1 = pool.get(|| 1).await;
        let item2 = pool.get(|| 2).await;

        pool.put(item1).await;
        pool.put(item2).await;

        // Only one item should be in the pool
        assert_eq!(pool.len().await, 1);
    }
}
