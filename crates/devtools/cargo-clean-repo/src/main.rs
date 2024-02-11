use husky_path_utils::{
    clear_directory, find_dirs_ending_with, find_paths, rust::collect_cargo_manifest_dirs, Path,
};
use husky_print_utils::p;
use std::path::PathBuf;

fn main() {
    // make sure that this is the repository  https://github.com/xiyuzhai-husky-lang/husky
    assert!(PathBuf::from("Cargo.toml").exists());
    assert!(PathBuf::from("crates").exists());
    assert!(PathBuf::from("rust-toolchain").exists());
    assert!(PathBuf::from("husky-toolchain.toml").exists());
    assert!(PathBuf::from(".corgi/config.toml").exists());
    // remove_folder_in_tests("try/try");
    clean_expect_files();
    // clean_library_adversarials();
    // clean_tests()
    // restructure()
    // clean_target_rs()
}

pub fn clean_target_rs() {
    for dir in find_dirs_ending_with(Path::new("."), "target-rs") {
        clear_directory(&dir).unwrap()
    }
}

pub fn clean_expect_files() {
    for dir in collect_cargo_manifest_dirs(".") {
        assert!(dir.join("Cargo.toml").exists());
        let expect_files_dir = dir.join("expect-files");
        if expect_files_dir.exists() {
            assert!(expect_files_dir.is_dir());
            clear_directory(&expect_files_dir).unwrap();
        }
    }
}

pub fn clean_library_adversarials() {
    for dir in collect_cargo_manifest_dirs(".") {
        assert!(dir.join("Cargo.toml").exists());
        let library_adversarials_dir = dir.join("adversarials/library");
        if library_adversarials_dir.exists() {
            assert!(library_adversarials_dir.is_dir());
        }
        clear_directory(&library_adversarials_dir).unwrap();
    }
}

pub fn remove_folder_in_tests(ends_with: &str) {
    let collect_paths = find_paths(&PathBuf::from("tests"));
    for path in collect_paths {
        if path.ends_with(ends_with) {
            std::fs::remove_dir_all(path).unwrap()
        }
    }
}

pub fn restructure() {
    fn corgi_toml(package_name: &str) -> String {
        format!(
            r#"[package]
name = "{package_name}""#
        )
    }

    let collect_paths = find_paths(&PathBuf::from("tests"));
    for path in collect_paths {
        if path.join("main.hsy").exists() {
            let package_name = path.file_name().unwrap().to_str().unwrap().to_owned();
            if package_name == "src" {
                continue;
            }
            let mut jobs = vec![];
            for entry in std::fs::read_dir(path.clone()).unwrap() {
                let subpath = entry.unwrap().path();
                if !subpath.ends_with("rust") && !subpath.ends_with("src") {
                    let newpath = subpath
                        .parent()
                        .unwrap()
                        .join("src")
                        .join(subpath.file_name().unwrap());
                    jobs.push((subpath, newpath))
                }
            }
            std::fs::create_dir_all(path.join("src")).unwrap();
            for (subpath, newpath) in jobs {
                p!(subpath, newpath);
                std::fs::rename(subpath, newpath).unwrap()
            }
            std::fs::write(path.join("Corgi.toml"), corgi_toml(&package_name)).unwrap()
        }
    }
}
