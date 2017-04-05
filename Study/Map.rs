use std::collections::HashMap;

fn main() {
    //申请一个可变的引用。
    let mut scores = HashMap::new();
    //插入值
    scores.insert("suyanlong", 27);
    scores.insert("suyanlong1", 27);
    scores.insert("suyanlong2", 27);
    scores.insert("suyanlong3", 27);
    scores.insert("suyanlong3", 26); //覆盖前面已经存在K的V值

    {
        //访问值，by Key
        let val = scores.get("suyanlong");
        println!("{:?}", val);
    }

    //更新HashMap
    // 1、键值不存在时，进行插入，否则不插入。我们经常会检查某个特定的键是否有值，如果没有就插入一个值。
    // 为此哈希 map 有一个特有的 API，叫做entry，
    scores.entry("suyanlong3").or_insert(12); //已经存在值了，不做任何操作。
    scores.entry("suyanlong4").or_insert(13); //不存在进行插入操作。

    // 2、存在键值对时，对在原有值的基础上进行更新。
    {
        let exit_val = scores.entry("suyanlong").or_insert(0);
        *exit_val += 1;// *解引用。
        println!("exit_val = {:?}",exit_val);
    }
    
    // 3、直接覆盖旧值。同样是调用insert()函数，进行覆盖。
    // scores.insert("suyanlong3", 20); //覆盖前面已经存在K的V值

    //整体遍历HashMap
    for pat in &scores {
        // pat其实是一个元组（k,v）,可以通过下标访问，同样pat可以换成（k,v）元组
        println!("K = {},V = {}", pat.0, pat.1);
    }
    
    //通过宏定义整体进行打印。
    println!("{:?}",scores);
}