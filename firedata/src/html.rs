pub mod html {

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use firebase_rs::Firebase;
use crate::user::user::*;



   #[derive(Deserialize)]
  pub struct ItemForm {
      ID: String,
      備註: String,
      價格: String, // Change to String to handle conversion
      品名: String,
      時間: String,
      種類: String,
  }
  
  pub async fn add_item(
      item: web::Json<ItemForm>,
      user_data: web::Data<Mutex<User>>,
      firebase: web::Data<Firebase>
  ) -> impl Responder {
      let mut user = user_data.lock().unwrap();
      let new_item = Item {
          ID: item.ID.clone(),
          備註: item.備註.clone(),
          價格: item.價格.parse().expect("Invalid price"), // Convert from String to i32
          品名: item.品名.clone(),
          時間: item.時間.clone(),
          種類: item.種類.clone(),
      };
  
      match user.upload(&firebase, &new_item).await {
          Ok(_) => HttpResponse::Ok().body("Item added successfully"),
          Err(_) => HttpResponse::InternalServerError().body("Error adding item"),
      }
  }
  
  pub async fn print_item_table(user: web::Data<Mutex<User>>) -> impl Responder {
    let user_items = &user.lock().unwrap().user_items;
    let mut table = String::new();
    
    if let Some(items) = user_items {
          let sortted_items = items.clone().order_by_time();
          table.push_str("<h2>支出表格</h2>");
          table.push_str("<table border='1'><tr><th>ID</th><th>備註</th><th>價格</th><th>品名</th><th>時間</th><th>種類</th></tr>");
          for (_, item) in &sortted_items.items {
              table.push_str(&format!(
                  "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                  item.ID, item.備註, item.價格, item.品名, item.時間, item.種類
              ));
          }
          table.push_str("</table>");
      }
      
      HttpResponse::Ok().body(format!("{}", table))
  }
}