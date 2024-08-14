mod person;
mod food;

use person::person::*;
use food::food::*;

fn main() {
  let mut Jhon = Person {
    name: String::from("Jhon"),
    age: 25,
    gender: 'M',
    height: 175.0,
    weight: 70.0
  };
  

  
  let apple = Food {
    name: String::from("Apple"),
    calories: 52.0
  };
  
  let banana = Food {
    name: String::from("Banana"),
    calories: 89.0
  };
  
  let daily_food = vec![apple, banana];
  
  let total_calories = caculate_calories(daily_food);
  println!("Today you ate total {} calories", total_calories);
    
    
  println!("{}的BMR是{:?}", Jhon.name, Jhon.bmr());
  println!("{}的BMI是{:?}", Jhon.name, Jhon.bmi());
  println!("{}的TDEE是{:?}", Jhon.name, Jhon.tdee(1.55));
  
  
  calorie_recommendation(&Jhon, 2000.0, Jhon.tdee(1.55));
}