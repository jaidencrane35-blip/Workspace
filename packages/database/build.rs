//! Embed SQL migrations into the crate so installed binaries do not depend on
//! a developer checkout path (`CARGO_MANIFEST_DIR` at runtime).

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let migrations_dir = manifest_dir.join("migrations");
    println!("cargo:rerun-if-changed=migrations");

    let mut entries: Vec<_> = fs::read_dir(&migrations_dir)
        .unwrap_or_else(|error| panic!("read migrations dir {}: {error}", migrations_dir.display()))
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("sql"))
        .collect();
    entries.sort_by_key(|entry| entry.file_name());
    assert!(
        !entries.is_empty(),
        "no SQL migrations found in {}",
        migrations_dir.display()
    );

    let mut code = String::from(
        "/// Compile-time embedded SQL migrations for installable builds.\n\
         pub(crate) fn compiled_migrations() -> Vec<crate::migration::Migration> {\n\
             vec![\n",
    );

    for entry in &entries {
        let path = entry.path();
        let stem = path
            .file_stem()
            .and_then(|value| value.to_str())
            .expect("migration stem");
        let abs = path
            .canonicalize()
            .unwrap_or_else(|_| path.clone())
            .to_string_lossy()
            .replace('\\', "/");
        // Strip Windows verbatim prefix so include_str! accepts the path.
        let abs = abs.strip_prefix("//?/").unwrap_or(&abs);
        code.push_str(&format!(
            "        crate::migration::Migration {{\n\
                            version: \"{stem}\".into(),\n\
                            name: \"{stem}\".into(),\n\
                            sql: include_str!(r#\"{abs}\"#).into(),\n\
                        }},\n"
        ));
        println!("cargo:rerun-if-changed={}", path.display());
    }
    code.push_str("    ]\n}\n");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("bundled_migrations.rs"), code)
        .expect("write bundled_migrations.rs");
}
