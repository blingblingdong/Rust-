use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;


fn show(table: &Table){
  for(artist, album) in table {
    println!("{}的單曲：", artist);
    for song in album {
      println!(" {}", song);
    }
  }
}


fn main() {
  let mut table = Table::new();
  
  table.insert("周杰倫".to_string(), vec!["青花瓷".to_string(), "稻香".to_string()]);
  table.insert("蔡依林".to_string(), vec!["倒帶".to_string(), "愛情三十六計".to_string()]);
  table.insert("張學友".to_string(), vec!["吻別".to_string(), "當愛已成往事".to_string()]);
  
  show(&table);
  assert_eq!(table["周杰倫"], ["青花瓷", "稻香"]);
}
