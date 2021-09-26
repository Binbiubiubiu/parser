use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum JsonValue{
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::Object(obj) => {
                let contents:Vec<_>= obj.iter().map(|(key,value)|{
                    format!("\"{}\":{}",key,value)
                }).collect();
                write!(f, "{{{}}}",contents.join(","))
            },
            JsonValue::Array(arr) => {
                let contents:Vec<_>=  arr.iter().map(|item|{
                    format!("{}",item)
                }).collect();

                write!(f, "[{}]", contents.join(","))
            },
            JsonValue::String(s) => write!(f, "{:?}",s),
            JsonValue::Number(n) => write!(f, "{}",n),
            JsonValue::Boolean(b) => write!(f, "{}",b),
            JsonValue::Null => write!(f, "null"),
        }
    }
}

// pub trait SetByPath{
//     fn set_paths(& mut self,path:Vec<&'static str>,value:JsonValue<'static>);
//     fn set(&mut self,key:&'static str,value:JsonValue<'static>);
// }

impl JsonValue {
    pub fn set_paths(&mut self, path: &[String], value: &JsonValue) {
        let obj = match self {
            JsonValue::Object(obj) => obj,
            _ => panic!("error type"),
        };
        
        if let Some(key) = path.first() {
  

            if path.len() == 1 {
                let is_array =key.len()>2 && key.ends_with("[]");
            
                if is_array {
                    let key = &key[0..key.len()-2].to_owned();
                    if let Some(json) =  obj.get_mut(key){
                        json.set(key.to_owned(), value.to_owned());
                    }else{
                        let mut json=JsonValue::Array(vec![]);
                        json.set(key.to_owned(), value.to_owned());
                        obj.insert(key.to_owned(), json);
                    }
                }else{
                    self.set(key.to_owned(), value.to_owned());
                };
                
                return;
            }

            
            if let Some(json) =  obj.get_mut(key){
                json.set_paths(&path[1..], value);
            }else{
                let mut json=JsonValue::Object(HashMap::new());
                json.set_paths(&path[1..], value);
                obj.insert(key.to_owned(), json);
            }
            
        }
    }

    pub fn set(&mut self, key: String, value: JsonValue) {
        match self {
            JsonValue::Object(obj) => {
                obj.insert(key, value);
            },
            JsonValue::Array(arr) => {
                arr.push(value);
            },
            _ => panic!("error type"),
        };
        
    }

    

    
}
