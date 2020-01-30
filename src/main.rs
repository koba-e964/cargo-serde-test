use serde::Serialize;

#[derive(Serialize)]
struct B<T>(i64, i32, String, T);

fn main() {}
