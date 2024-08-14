pub mod user {

use firebase_rs::Firebase;
use serde_json::Value;
use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;  v 


  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct Item {
      pub ID: String,
      pub 備註: String,
      pub 價格: i32,
      pub 品名: String,
      pub 時間: String,
      pub 種類: String,
  }
  
  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct User {
    pub user_name: String,
    pub user_mail: String,
    pub user_items: Option<Items>
  }
  
  impl User {
  
    pub fn new(user_name: String, user_mail: String) -> Self {
      User {user_name, user_mail, user_items: None}
    }
  
    pub async fn download(&mut self, firebase: &Firebase) -> Self {
      match firebase.at(&self.user_name).get::<Value>().await  {
            Ok(response) => {
                // 获取和解析 JSON 数据
                let data: Value = response;
                let map: IndexMap<String, Item> = serde_json::from_value(data).unwrap();
                let new_items = Items::new(map);
                self.user_items = Some(new_items);
            }
            Err(e) => {
                println!("Error getting data: {}", e);
            }
        }
        
        self.clone()
    }
    
        pub async fn upload(&mut self, firebase: &Firebase,item: &Item) -> Result<Self, Box<dyn std::error::Error>> {
      
          let mut map: IndexMap<String, Item> = IndexMap::new();
          map.insert(item.ID.clone(), item.clone());
      
          let response = firebase
              .at(&self.user_name)
              .update::<IndexMap<String, Item>>(&map)
              .await;
              
              
          match response {
              Ok(_) => {
                  println!("Data updated successfully.");
                  Ok(self.download(firebase).await)
              }
              Err(e) => {
                  eprintln!("Failed to update data: {}", e);
                  Err(Box::new(e))
              }
          }
      }
  }
  
  
  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct Items{
      pub items: IndexMap<String, Item>
  }
  
  impl Items {
       fn new(items: IndexMap<String, Item>) -> Self {
          Items { items }
      }
  
      
      pub fn filter_by_date(mut self, early_date: &str, later_date: &str) -> Self {
          let f_early_date = NaiveDate::parse_from_str(early_date, "%Y-%m-%d").unwrap();
          let f_later_date = NaiveDate::parse_from_str(later_date, "%Y-%m-%d").unwrap();
  
          self.items = self.items.into_iter()
              .filter(|(_, item)| {
                  let item_date = NaiveDate::parse_from_str(&item.時間, "%Y-%m-%d").unwrap();
                  item_date >= f_early_date && item_date <= f_later_date
              })
              .collect();
          
          self
      }
  
      pub fn filter_by_price(mut self, higher_price: i32, lower_price: i32) -> Self {
          self.items = self.items.into_iter()
              .filter(|(_, item)| item.價格 >= lower_price && item.價格 <= higher_price)
              .collect();
          
          self
      }
  
      pub fn filter_by_category(mut self, categories: Vec<String>) -> Self {
          self.items = self.items.into_iter()
              .filter(|(_, item)| categories.contains(&item.種類))
              .collect();
          
          self
      }
      
      pub fn order_by_time(mut self) -> Self {
        let date_format = "%Y-%m-%d";

        // 將 items 轉換為向量
        let mut v: Vec<_> = self.items.into_iter().collect();

        // 按時間排序
        v.sort_by(|a, b| {
            let date_a = NaiveDate::parse_from_str(&a.1.時間, date_format).unwrap_or_else(|_| NaiveDate::from_ymd(1970, 1, 1));
            let date_b = NaiveDate::parse_from_str(&b.1.時間, date_format).unwrap_or_else(|_| NaiveDate::from_ymd(1970, 1, 1));
            date_b.cmp(&date_a) // 由新到舊
        });

        // 使用 IndexMap 保持順序
        let sorted_map: IndexMap<_, _> = v.into_iter().collect();
        
        Items::new(sorted_map)
    }
      
      
      
  }
}