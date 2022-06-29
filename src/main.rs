use std::collections::HashMap;
use walkdir::WalkDir;
use path_slash::PathExt;

fn main() {
	let mut duplicates = HashMap::new();

	let dir1 = "TestSamples/Sims4/Body/";
	let dir2 = "TestSamples/Sims4/Clothing/";

	let files1: Vec<_> = WalkDir::new(dir1)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| !e.file_type().is_dir())
		.collect();

	let files2: Vec<_> = WalkDir::new(dir2)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| !e.file_type().is_dir())
		.collect();

	for file1 in files1 {
		for file2 in &files2 {
			let file1_str = String::from(file1.file_name().to_string_lossy());
			let file2_str = String::from(file2.file_name().to_string_lossy());

			if file1_str == file2_str {
				duplicates.insert(file1.path().to_slash().unwrap(), file1_str);
				duplicates.insert(file2.path().to_slash().unwrap(), file2_str);
			}
		}
	}

	for dupe in duplicates {
		println!("{}", dupe.0);
	}
}
