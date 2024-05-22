use std::path::PathBuf;
use once_cell::sync::Lazy;

pub static EXE: Lazy<PathBuf> =
    Lazy::new(|| std::env::current_exe().expect("Failed to get self executable path!"));
pub static EXEDIR: Lazy<PathBuf> = Lazy::new(|| {
    #[cfg(debug_assertions)]
    return PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    #[cfg(not(debug_assertions))]
    EXE.parent()
        .expect("Failed to get containing directory!")
        .to_path_buf()
});
pub static WORKDIR: Lazy<PathBuf> =
    Lazy::new(|| std::env::current_dir().expect("Failed to get working directory path!"));

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
