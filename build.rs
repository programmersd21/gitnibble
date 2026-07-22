use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("embedded_templates.rs");

    let templates_dir = Path::new("templates");
    let mut code = String::new();
    code.push_str("pub static EMBEDDED_TEMPLATES: &[(&str, &str)] = &[\n");

    if templates_dir.exists() {
        if let Ok(entries) = fs::read_dir(templates_dir) {
            let mut entries_vec: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            entries_vec.sort_by_key(|e| e.file_name());

            for entry in entries_vec {
                let path = entry.path();
                if path.is_file() {
                    let name = path.file_name().unwrap().to_string_lossy();
                    if name.ends_with(".gitignore") {
                        let key = name.trim_end_matches(".gitignore");
                        let rel_path = path.to_str().unwrap().replace('\\', "/");
                        code.push_str(&format!(
                            "    ({:?}, include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{}\"))),\n",
                            key, rel_path
                        ));
                    }
                }
            }
        }
    }

    code.push_str("];\n");
    fs::write(dest_path, code).unwrap();

    println!("cargo:rerun-if-changed=templates");
}
