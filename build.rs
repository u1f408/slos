use cargo_toml::Manifest;
use std::{env, path::PathBuf};

fn detect_features(manifest: &Manifest) -> Vec<String> {
	let mut features = Vec::new();

	for (key, value) in env::vars() {
		if key.starts_with("CARGO_FEATURE_") {
			if value != "1" {
				continue;
			}

			let feature_env = String::from(&key[14..]).to_ascii_lowercase();
			if manifest.features.contains_key(&feature_env) {
				features.push(feature_env);
			} else if manifest
				.features
				.contains_key(&feature_env.replace("_", "-"))
			{
				features.push(feature_env.replace("_", "-"));
			} else {
				panic!(
					"Enabled feature {:?} was not found in Cargo manifest",
					feature_env
				);
			}
		}
	}

	features
}

fn main() {
	let manifest_path: PathBuf = [&env::var("CARGO_MANIFEST_DIR").unwrap(), "Cargo.toml"]
		.iter()
		.collect();
	let manifest = Manifest::from_path(&manifest_path).unwrap();

	println!(
		"cargo:rustc-env=SLOS_FEATURES={}",
		detect_features(&manifest).join(" ")
	);
}
