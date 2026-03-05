use todoer::project::resolve_project;
use std::fs;
use tempfile::tempdir;

#[test]
fn resolves_project_from_todoer_file() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("repo");
    let sub = root.join("a");
    fs::create_dir_all(&sub).unwrap();
    fs::write(root.join(".todoer.toml"), "project = \"Name\"").unwrap();

    let proj = resolve_project(None, None, &sub, &root).unwrap();
    assert_eq!(proj.name, "Name");
}
