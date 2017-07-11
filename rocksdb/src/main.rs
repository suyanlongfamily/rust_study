extern crate rocksdb;
use rocksdb::DB;


fn main() {
    // NB: db is automatically closed at end of lifetime
    let db = DB::open_default("path/for/rocksdb/storage").unwrap();
    //原来是这样用的呀，文件数据库，或者说是嵌入式的数据库。也是key-value数据库。
    //理解它，其实就是一个文件数据库，就行了。不需要太多的理由。
    db.put(b"my key", b"my value");
    match db.get(b"my key") {
        Ok(Some(value)) => println!("retrieved value: {}", value.to_utf8().unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }
    db.delete(b"my key").unwrap();
    
}
