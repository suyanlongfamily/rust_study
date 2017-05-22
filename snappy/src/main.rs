
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

}
