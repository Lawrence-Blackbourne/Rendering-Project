use std::{env,
          fs,
          path::Path};

fn main() {
    generate_regular_image_format_conversion_data()
}

fn generate_regular_image_format_conversion_data() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("regular_image_format_conversion_data.rs");
    let text = String::new();

    fs::write(&dest_path, text.as_str()).unwrap();
}