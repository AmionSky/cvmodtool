/// Json merge both map and array
pub fn json_merge(a: &mut json::Value, b: json::Value) {
    match (a, b) {
        (a @ &mut json::Value::Object(_), json::Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                json_merge(a.entry(k).or_insert(json::Value::Null), v);
            }
        }
        (a @ &mut json::Value::Array(_), json::Value::Array(b)) => {
            let a = a.as_array_mut().unwrap();
            a.extend(b);
        }
        (a, b) => *a = b,
    }
}
