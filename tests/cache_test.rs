mod tests {
    /*
    use std::{any::Any, sync::Arc};

    use amari_rs::cache::{CacheDataEntry, CacheSystem};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Clone, Debug)]
    struct CacheDataTester {
        id: u64,
        name: String,
    }

    #[derive(Deserialize, Serialize, Clone, Debug)]
    struct CacheDataTester2 {
        xn: u64,
        name: String,
    }

    #[tokio::test]
    async fn test_cache() {
        let mut cacher: CacheSystem = CacheSystem::new(60);
        let data = CacheDataTester { id: 1, name: "test".to_string() };

        cacher.set(("x".into(), 111, 0), Arc::new(data.clone().into()));
        let check1: Arc<CacheDataTester> = cacher.get(("x".into(), 111, 0)).unwrap();

        dbg!(&check1);
        assert_eq!(check1.id, data.id);

        let data2 = CacheDataTester2 { xn: 1, name: "test".to_string() };
        cacher.set(("x".into(), 111, 0), Arc::new(data2.clone()));
    }
    */

    use std::sync::Arc;
    use amari_rs::cache::Cache;

    #[test]
    fn test_all() {
        let mut cache = Cache::new(1000, 256 * 1024 * 1024);
        let test1: Vec<u8> = vec![1, 2, 3];

        cache.set(&("test".into(), 111, 0, None), Arc::new(test1.clone()));

        let grab1 = cache.get(&("test".into(), 111, 0, None)).unwrap();
        let data1 = grab1.downcast_ref::<Vec<u8>>().unwrap();

        assert_eq!(data1.len(), 3);
        dbg!(&data1[0]);

        struct X { test: u64 }
        let data2 = X { test: 4423 };

        cache.set(&("test2".into(), 112, 2, None), Arc::new(data2));
        let grab2 = cache.get(&("test2".into(), 112, 2, None)).unwrap();

        let data2 = grab2.downcast_ref::<X>().unwrap();
        assert_eq!(data2.test, 4423);

        dbg!(&data2.test);
    }
}