use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;

use epub::archive::EpubArchive;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

type Result<T> = core::result::Result<T, Box<dyn core::error::Error>>;

#[test]
fn test_thing() -> Result<()> {
    return Ok(());

    let path = PathBuf::from("~/Japanese/TMW eBook Collection Pt. 1");
    let entries = gather_epubs(path);

    let (mut good, mut bad) = (AtomicU16::new(0), AtomicU16::new(0));
    let len = entries.len();

    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    entries.par_iter().enumerate().for_each(|(idx, entry)| {
        let archive = EpubArchive::opennew(entry);

        let (good_count, bad_count) = if archive.is_ok() {
            (
                good.fetch_add(1, Ordering::Relaxed),
                bad.load(Ordering::Relaxed),
            )
        } else {
            dbg!(archive);
            dbg!(entry);
            panic!();

            (
                good.load(Ordering::Relaxed),
                bad.fetch_add(1, Ordering::Relaxed),
            )
        };

        println!(
            "{}/{} ({} good, {} bad)",
            good_count + bad_count,
            &len,
            good_count,
            bad_count,
        );
    });

    let archive = EpubArchive::open("./tests/fixtures/bookworm.epub")?;

    /*
    let mut content = archive.content();

    let mut text = "".to_owned();

    loop {
        if let Some(data) = content.read() {
            text.push_str(&data);
            text.push('\n');
        } else {
            break;
        }

        let data = content.read();
    }

    println!("{:?}", text);

    // println!("{}", content.read().unwrap());
    // content.next();

    // println!("{:#?}", archive);
     */

    assert!(1 == 2);

    Ok(())
}

fn gather_epubs(path: PathBuf) -> Vec<PathBuf> {
    println!("gathering epubs");

    let mut vec = vec![];

    if path.is_file() {
        if let Some(ext) = path.extension()
            && ext == "epub"
        {
            vec.push(path);
        }

        return vec;
    }

    for entry in path.read_dir().unwrap() {
        vec.append(&mut gather_epubs(entry.unwrap().path()));
    }

    vec
}
