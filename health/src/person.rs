pub mod person {
  pub struct Person {
    pub name: String,
    pub age: u8,
    pub gender: char,
    pub height: f64,
    pub weight: f64
  }
  
  
  impl Person {
  
    pub fn bmr_caculate(&self) -> Result<f64, ()>  {
      let age = self.age as f64;
      match self.gender {
        'M' => Ok(10.0 * self.weight + 6.25 * self.height - 5.0 * age + 5.0),
        'F' => Ok(10.0 * self.weight + 6.25 * self.height - 5.0 * age - 161.0),
        _ => Err(())
      }
    }
    
    pub fn bmr(&self) -> f64 {
      self.bmr_caculate().unwrap()
    }
    
    pub fn tdee(&self, active: f64) -> f64 {
      self.bmr() * active
    }
    
    pub fn bmi(&self) -> f64 {
      let height = self.height / 100.0;
      self.weight / (height * height)
    }
  }
  
  pub fn calorie_recommendation(person: &Person, calories: f64, tdee: f64) {
    println!("你的建議攝取熱量介於{}與{}之間", person.bmr(), tdee);
  
  
    if calories < person.bmr() {
      println!("你的熱量攝取不足");
    } else if calories > tdee {
      println!("你的熱量攝取過多");
    } else {
      println!("你的熱量攝取正常");
    }
  }
  
  pub fn bmi_recommendation(bmi: f64) {
    if bmi < 18.5 {
      println!("你的體重過輕");
    } else if bmi < 24.0 {
      println!("你的體重正常");
    } else if bmi < 27.0 {
      println!("你的體重過重");
    } else {
      println!("你的體重肥胖");
    }
  }
    
}