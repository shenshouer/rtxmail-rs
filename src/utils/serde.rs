// use serde_json::Value;
// use std::collections::HashMap;

// /// json新增字段
// pub fn merge_with_hashmap(v: &mut Value, fields: &HashMap<String, String>) {
//     if let Value::Object(m) = v {
//         for (k, v) in fields {
//             m.insert(k.clone(), Value::String(v.clone()));
//         }
//     }
// }
