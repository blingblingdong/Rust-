//! ## 模擬蕨類植物的生長

/// 蕨類植物的結構
pub struct Fern {
    pub size: f64,
    pub growth_rate: f64
  }

/// 蕨類植物的生長過程
impl Fern {
    /// 模擬生長過程
    pub fn grow(&mut self){
      self.size *= 1.0 + self.growth_rate;
    }
  }
  
  
#[doc = "模擬生長過程"]
pub fn run_simulation(fern: &mut Fern, days: usize){
    for _ in 0..days{
      fern.grow();
    }
  }

