use std::{fs, path::Path, process::Command};

fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn hash_dir(dir: &Path, h: &mut blake3::Hasher) {
    let mut paths: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    for p in paths {
        if p.is_dir() {
            hash_dir(&p, h);
        } else if p.extension().is_some_and(|e| e == "rs") {
            h.update(p.to_string_lossy().as_bytes());
            h.update(&fs::read(&p).unwrap());
        }
    }
}

fn main() {
    let sha = git(&["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".into());
    let dirty = git(&["status", "--porcelain", "--", "src"]).is_some_and(|s| !s.is_empty());

    let mut h = blake3::Hasher::new();
    hash_dir(Path::new("src"), &mut h);

    println!("cargo:rustc-env=GIT_SHA={sha}");
    println!("cargo:rustc-env=GIT_DIRTY={dirty}");
    println!("cargo:rustc-env=SRC_HASH={}", h.finalize().to_hex());

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=.git/HEAD");

    if let Some(r) = git(&["symbolic-ref", "-q", "HEAD"]) {
        println!("cargo:rerun-if-changed=.git/{r}");
    }
}
