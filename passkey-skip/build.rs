fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        embed_resource::compile::<&str, String, Vec<String>>("windows-resources.rc", Vec::new());
    }
} 
