use block::{Block, Theme};
use std::collections::HashMap;
use std::hash::Hash;
use serde_json::Value;
use serde_json::map::Map;
use serde_json::error::Error;
use serde_json;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

fn merge_json_objects(v1: &Value, v2: &Value) -> Option<Value> {
    use Value::Object;
    if let &Object(ref map1) = v1 {
        if let &Object(ref map2) = v2 {
            let mut map_merged = Map::new();

            for (k, v) in map1 {
                map_merged.insert(k.clone(), v.clone());
            }

            for (k, v) in map2 {
                map_merged.insert(k.clone(), v.clone());
            }

            return Some(Object(map_merged));
        }
    }
    None
}

pub fn print_blocks(blocks: &Vec<&Block>, template: &Value, theme: &Theme) {
    print!("[");
    for (idx, block) in blocks.iter().enumerate() {

        // We get the status, and then we merge the template with it
        let status = &block.get_status(theme);

        let merged = merge_json_objects(template, status).unwrap();

        if let Value::Object(mut map) = merged {
            if let Some(id) = block.id() {
                map.insert(String::from("name"), Value::String(String::from(id)));
            }
            let m = Value::Object(map);
            print!("{}", m.to_string());
        }

        if idx != (blocks.len() - 1) {
            print!(",");
        }
    }
    println!("],");
}
