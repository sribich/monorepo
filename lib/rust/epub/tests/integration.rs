use std::path::PathBuf;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;

use epub::archive::EpubArchive;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

#[test]
fn validation_pass() {
    let path = PathBuf::from(&format!(
        "{}/Japanese/TMW eBook Collection Pt. 1",
        std::env::var("HOME").unwrap()
    ));

    let entries = gather_epubs(path);

    let (success, fail) = (AtomicU16::new(0), AtomicU16::new(0));
    let len = entries.len();

    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();

    entries.par_iter().for_each(|path| {
        let mut archive = EpubArchive::open(path);

        let is_err = archive.is_err();

        let (good, bad) = if let Err(e) = &archive {
            // println!("{:#?}", path);
            // println!("{:#?}", e);

            (
                success.load(Ordering::Relaxed),
                fail.fetch_add(1, Ordering::Relaxed),
            )
        } else {
            (
                success.fetch_add(1, Ordering::Relaxed),
                fail.load(Ordering::Relaxed),
            )
        };

        if is_err {}

        println!("{:#?}", path);
        archive.map(|mut it| it.text());

        println!("{}/{} ({} failed)", good + bad, len, bad);
    });
}

fn gather_epubs(path: PathBuf) -> Vec<PathBuf> {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|s| matches!(s.path().extension(), Some(ext) if ext == "epub"))
        .map(walkdir::DirEntry::into_path)
        .collect::<Vec<_>>()
}

// 1
