use std::time::{SystemTime, UNIX_EPOCH};

use super::resource_extractor::*;

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("roseau-rs-{name}-{nonce}"))
}

#[test]
fn extracts_first_resource_whose_relative_path_contains_name() {
    let root = temp_dir("resources");
    let nested = root.join("config");
    fs::create_dir_all(&nested).unwrap();
    fs::write(nested.join("roseau.properties"), "server.port=30000").unwrap();

    let output = temp_dir("output").join("roseau.properties");
    let extractor = ResourceExtractor::new(&root);

    assert!(extractor
        .extract_matching("roseau.properties", &output)
        .unwrap());
    assert_eq!(fs::read_to_string(&output).unwrap(), "server.port=30000");

    fs::remove_dir_all(root).unwrap();
    fs::remove_dir_all(output.parent().unwrap()).unwrap();
}

#[test]
fn missing_or_directory_destinations_do_not_extract() {
    let root = temp_dir("empty");
    fs::create_dir_all(&root).unwrap();
    let destination = temp_dir("existing-destination");
    fs::create_dir_all(&destination).unwrap();
    let extractor = ResourceExtractor::new(&root);

    assert!(!extractor
        .extract_matching("missing.txt", &destination)
        .unwrap());
    assert!(!ResourceExtractor::new(root.join("missing"))
        .extract_matching("missing.txt", destination.join("copy.txt"))
        .unwrap());

    fs::remove_dir_all(root).unwrap();
    fs::remove_dir_all(destination).unwrap();
}
