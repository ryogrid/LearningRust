macro_rules! ArMu_new {
    ($wrapped:expr) => (
        Arc::new(Mutex::new($wrapped))
    );    
}

type ArMu<T> = Arc<Mutex<T>>;

pub struct DataStore {
    pub num: Mutex<i32>,
    pub data: HashMap<String, i32>,
}

impl DataStore {
    pub fn new(sd: HashMap<String,i32>) -> DataStore {
        let sd = HashMap::new();
        DataStore {num: ArMu_new(0), data: sd}
    }
}

pub struct DataStoreWrapper {
    num: Mutex<i32>,
    ds: Mutex<DataStore>,
}

pub struct Hoge{
    pub num: Mutex<i32>,
    pub dsw: Arc<DataStoreWrapper>,
}

impl Hoge {
    pub fn new(dsw: Arc<DataStoreWrapper>) -> Hoge {
        let hoge = Hoge {num: ArMu_new(0), ds: dsw};
        return hoge
    }
}

pub struct Fuga{
    pub num: Mutex<i32>,
    pub ds: Arc<DataStoreWrapper>,
}

impl Fuga {
    pub fn new(dsw: Arc<DataStoreWrapper>) -> Fuga {
        let fuga = Hoge {num: ArMu_new(0), ds: dsw};
        return fuga
    }
}

fn main(){
    let hmap = HashMap::new();
    let wrapHmap = ArMu_new!(hmap);
    let ds = DataStore::new(wrapHmap);
    let dsw = DataStoreWrapper{num: ArMu_new(0), ds: ArMu_new(ds)};
    let adsw = Arc::new(dsw);

    let hoge = Hoge::new(adsw);


    let handle1 = thread::spawn(move || {
        hoge.dsw.ds.lock().unwrap().data.insert("hoge", 1);
        let mut ds = dsw.lock().unwrap();
        let mut num = ds.num.lock().unwrap();
        *num += 1;
    });

    let handle2 = thread::spawn(move || {
        fuga.dsw.ds.lock().unwrap().data.insert("fuga", 1);
    });

    let mut thread_handles = vec![];
    thread_handles.push(handle1);
    thread_handles.push(handle2);

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
