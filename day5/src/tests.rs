use std::{fs::File, io::Read};


#[test]
fn fresh_spoiled_test() {
    let content = read_content("sample");
}


fn read_content(file_name: &str) -> String {
    let path = format!("./data/{}", file_name);
    let mut f = File::open(path).expect("Error reading file");
    let mut content = String::new();
    let _ = f.read_to_string(&mut content);
    content
}
