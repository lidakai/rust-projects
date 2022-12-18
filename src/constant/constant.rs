use std::collections::HashMap;
//       self.scores = HashMap::new();
// 常量
pub struct Constant {
    scores: HashMap<String,String>,
}

 impl Constant {
    pub fn new() -> Self {
        Self {
            scores: HashMap::new()
        }
    }
    pub fn set(&mut self, _key: String, _value: String) {
        self.scores.insert(_key, _value);
    }
    pub fn get(&self,_key:String) -> Option<&String> {
        // for (key, value) in &self.scores {
        //     println!("{}: {}", key, value);
        // }
       let s =  self.scores.get(&_key);


       return  s;
        // let score = &self.scores.get(&_key);
        // println!("score {:?}", score);
        // return score();
    }
}
