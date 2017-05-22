
mod snappy;
use snappy::*;

fn main() {
    // let data = vec![12,12,111,32,21,12,120];
    let data = "123".to_string();
    println!("---{:?}----",&data);
    // let com_data = compress(data.as_bytes());
    // println!("-----compress = {:?}----",com_data);
    // println!("-----uncompress = {:?}----",decompress(&com_data));
    // println!("Hello, world!");
    let d = vec![0xde, 0xad, 0xd0, 0x0d];
    let c = compress(&d);
    let falg = validate_compressed_buffer(&c);
    assert!(validate_compressed_buffer(&c));
    let str_info = String::from("苏彦龙年后的搜房闹洞房是的覅动手变覅阿斯蒂芬妮萨大富豪");
    println!("-------str_info = {}---",str_info);
    let str_info_bytes = str_info.as_bytes();
    println!("-------str_info = {:?}---",str_info_bytes);
    let str_info_tmp = String::from_utf8(Vec::from(str_info_bytes)).unwrap();
    println!("-------str_info_tmp = {:?}---",str_info_tmp);
    //utf-8 字符编码的转换。
    //编码转换技巧是什么。
    use std::time::{Duration, SystemTime};

    let now = SystemTime::now();
    println!("-----{:?}----",now.elapsed().unwrap());
    // let now1 = SystemTime::now();
    // std::thread::sleep_ms(10);
    println!("-----{:?}----",now.elapsed().unwrap());
    println!("-----{:?}----",now.elapsed().unwrap());
    println!("-----{:?}----",now.elapsed().unwrap().subsec_nanos());
    println!("-----{:?}----",now.elapsed().unwrap());
    println!("-----{:?}----",now.elapsed().unwrap());
    println!("-----{:?}----",now.elapsed().unwrap());

    use std::fs;
    use std::fs::File;
    use std::fs::FileType;
    use std::io::Read;
    use std::os::linux::fs::MetadataExt;

    let dir = fs::create_dir("./new_dir");
    let mut file_handle = File::open("./src/snappy.rs").unwrap();
    let mut str_info = String::default();
    file_handle.read_to_string(&mut str_info);

    println!("---infomation snappy {:?}",str_info);
    println!("---infomation snappy size {:?}",std::fs::metadata("./src/snappy.rs").unwrap().st_size());
    println!("---------------------------------------------------");
    let path = String::from("/home/suyanlong/rust_study/snappy/src/snappy.rs");
    let mut file = File::open(&path).unwrap();
    let metdata = fs::metadata(&path).unwrap();
    println!("-----decompress befor = {:?}",metdata.len());
    
    let mut data_vec = vec![];
    let data_size = file.read_to_end(&mut data_vec);
    println!("------data_size = {:?}",data_size.unwrap());

    let com_data = compress(&data_vec);
    let mut com_file = File::create("./data_compress");

    use std::io::Write;
    com_file = com_file.map(|mut file|{
        //终于明白这个map的用法了。
        //有些需要继续使用当前所有权，但是，调用有些方法时，直接转移了，原来的所有者就不能用了。
        //所以，增加一个map方法，返回值与原来的是同一个类型，或者说原来的所有权。
        file.write_all(&com_data);
        file
    });
    println!("-----compress data size = {:?}---",(com_file).unwrap().metadata().unwrap().len());




}
