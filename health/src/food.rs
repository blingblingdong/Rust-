pub mod food {
  pub struct Food {
    pub name: String,
    pub calories: f64
  }
  
  impl Food {
    pub fn eat(&self){
      println!("You ate {} which has {} calories", self.name, self.calories);
    }
  }
  
  pub fn caculate_calories(daily_food: Vec<Food>) -> f64 {
    let mut total_calories = 0.0;
    for food in daily_food {
      total_calories += food.calories;
    }
    total_calories
  }
}