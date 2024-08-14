use firebase_rs::Firebase;
use tokio;
use std::sync::Mutex;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;
mod user;
mod html;

use user::user::*;
use html::html::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut user = User::new("main/Dong".to_string(), "kk".to_string());
    let firebase = Firebase::new("https://financial-shinyapp-default-rtdb.asia-southeast1.firebasedatabase.app/").unwrap();
    user.download(&firebase).await;

    let user_data = web::Data::new(Mutex::new(user));
    let firebase_data = web::Data::new(firebase);
    
    

    HttpServer::new(move || {
        App::new()
            .app_data(user_data.clone())
            .app_data(firebase_data.clone())
            .route("/add_item", web::post().to(add_item))
            .route("/get_table", web::get().to(print_item_table))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}






/*
#[tokio::main]
async fn main() {
    let firebase = Firebase::new("https://financial-shinyapp-default-rtdb.asia-southeast1.firebasedatabase.app/").unwrap();
    let path = "main/Dong";
    let mut user1 = User::new("main/Dong".to_string(),"kk".to_string());
    user1.download(&firebase).await;
    let data = user1.user_items.unwrap().order_by_time();
    for (key, item) in data.items {
        println!("{}: {:?}", key, item);
    }
    
    /*
    let mut total = 0;
    for (key, item) in data.unwrap() {
        let formatted_time = NaiveDate::parse_from_str(&item.時間, "%Y-%m-%d").unwrap();
        if formatted_time.month() == 4 {total += item.價格;}
    }
    println!("Total: {}", total);
    */
    
     
    /*
    let date1 = "2024-03-01";
    let date2 = "2024-05-03";
    let map = in_time_date(date1, date2, data.clone().unwrap()).
    filter_by_price(0, -100);
    
    for (key, item) in map {
        println!("{}: {:?}", key, item);
    }
    */
    
    
    /*
    let data1 = Items::new(data.unwrap());
    let data2 = data1.filter_by_date("2024-03-01","2024-05-03").
    filter_by_price(0, -40);
    for (key, item) in data2.items {
        println!("{}: {:?}", key, item);
    }
    */
}
*/

  