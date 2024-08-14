// json

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    ID: String,
    備註: String,
    價格: i32,
    品名: String,
    時間: String,
    種類: String,
}


use serde_json::{Value, from_str};
use std::collections::HashMap;

fn main() {
    let json_str = r#"{
        "-Nw4oRhsWXslvIjqsWxF": {"ID":"2024-04-22 12:39:46","備註":"雞肉飯","價格":-130,"品名":"中餐","時間":"2024-04-22","種類":"三餐"},
        "-Nw4odFxxC-1Xy5l86ES": {"ID":"2024-04-22 12:40:37","備註":"悟饕雞腿飯","價格":-130,"品名":"晚餐","時間":"2024-04-22","種類":"三餐"}
    }"#;

    // 将 JSON 字符串解析为 `Value`
    let data: Value = from_str(json_str).unwrap();

    // 将 `Value` 转换为 `HashMap<String, Item>`
    let map: HashMap<String, Item> = serde_json::from_value(data).unwrap();

    // 遍历 `HashMap` 并打印数据
    for (key, item) in map.iter() {
        println!("ID: {}", item.ID);
        println!("備註: {}", item.備註);
        println!("價格: {}", item.價格);
        println!("品名: {}", item.品名);
        println!("時間: {}", item.時間);
        println!("種類: {}", item.種類);
        println!("-------------------");
    }
}
