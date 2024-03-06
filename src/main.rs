use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::thread;
use std::thread::sleep;
use std::time;

macro_rules! ArMu_new {
    ($wrapped:expr) => (
        Arc::new(Mutex::new($wrapped))
    );    
}

type ArMu<T> = Arc<Mutex<T>>;

pub struct DataStore {
    pub num: i32,
    pub data: HashMap<String, i32>,
}

impl DataStore {
    pub fn new() -> DataStore {
        let sd = HashMap::new();
        DataStore {num: 0, data: sd}
    }
}

pub struct DataStoreWrapper {
    num: Mutex<i32>,
    ds: ArMu<DataStore>,
}

impl DataStoreWrapper {
    pub fn new(ds: ArMu<DataStore>) -> DataStoreWrapper {
        let num = 0;
        DataStoreWrapper {num: Mutex::new(num), ds: ds}
    }
}

pub struct Hoge{
    pub num: Mutex<i32>,
    pub dsw: Arc<DataStoreWrapper>,
}

impl Hoge {
    pub fn new(dsw: Arc<DataStoreWrapper>) -> Hoge {
        let num = 0;
        let hoge = Hoge {num: Mutex::new(num), dsw: dsw};
        return hoge
    }
}

pub struct Fuga{
    pub num: Mutex<i32>,
    pub dsw: Arc<DataStoreWrapper>,
}

impl Fuga {
    pub fn new(dsw: Arc<DataStoreWrapper>) -> Fuga {
        let num = 0;
        let fuga = Fuga {num: Mutex::new(num), dsw: dsw};
        return fuga
    }
}

fn main(){
    let ds = DataStore::new();
    let ads = ArMu_new!(ds);
    let dsw = DataStoreWrapper::new(ads);
    let adsw = Arc::new(dsw);

    let hoge = Hoge::new(adsw.clone());
    let fuga = Fuga::new(adsw.clone());

    let hoge1 = Arc::new(hoge);
    let hoge2 = hoge1.clone();

    let fuga1 = Arc::new(fuga);
    let fuga2 = fuga1.clone();


    let handle1 = thread::spawn(move || {
        let mut num = 0;        
        loop {            
            hoge1.dsw.ds.lock().unwrap().data.insert("hoge".to_string() + &&num.to_string(), num);
            let mut numLocal = fuga1.dsw.num.lock().unwrap();
            *numLocal += 1;
            num += 1;
            println!("num of thread1: {}", *numLocal);
            println!("thread1");
            sleep(time::Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        let mut num = 0;
        loop {            
            fuga2.dsw.ds.lock().unwrap().data.insert("fuga".to_string() + &num.to_string(), num);
            let mut numLocal = hoge2.dsw.num.lock().unwrap();
            *numLocal += 1;
            num += 1;
            println!("thread2");
            println!("num of thread2: {}", *numLocal);
            sleep(time::Duration::from_secs(1));
        }
    });

    let mut thread_handles = vec![];
    thread_handles.push(handle1);
    thread_handles.push(handle2);

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
