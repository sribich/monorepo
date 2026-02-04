use std::{fs::File, io::Read, path::Path};

use color_eyre::eyre::Result;

use self::format::{srt::SrtFile, SubtitleFormat};

pub mod format;
mod timing;

pub trait SubtitleFileInterface {}

#[derive(Clone, Debug)]
pub enum SubtitleFile {
    // .srt
    SubRipFile(SrtFile),
    // .ssa/ass
    SubStationAlphaFile(), // SsaFile
    // .idx
    VobSubIdxFile(), // VobIdxFile
    // .sub
    VobSubSubFile(), // VobSubFile
    // .sub
    MicroDVDFile(), // MicroDvdFile
}

pub struct SubtitleEntry {
    // pub timespan: TimeSpan
    // pub line: Option<String>,
}

/// Given the path to a file, return a `SubtitleFile` that is able
/// to parse subtitle entries from the result.
///
pub fn get_subtitle_file(path: impl AsRef<Path>) -> Result<SubtitleFile, i32> {
    let format = get_subtitle_format(&path).ok_or(0)?;

    match format {
        SubtitleFormat::SubRip => Ok(SubtitleFile::SubRipFile(SrtFile::new(&path))),
        SubtitleFormat::SubStationAlpha => Err(0),
        SubtitleFormat::VobSubIdx => Err(0),
        SubtitleFormat::VobSubSub => Err(0),
        SubtitleFormat::MicroDVD => Err(0),
    }
}

/// Returns the subtitle format used by the provided file.
///
pub fn get_subtitle_format(path: &impl AsRef<Path>) -> Option<SubtitleFormat> {
    if let Some(extension) = path.as_ref().extension() {
        return match extension.to_str() {
            Some("sub") => {
                let mut cursor = vec![0u8; 4];
                let mut handle = File::open(path).ok()?;
                handle.read_exact(&mut cursor).ok()?;

                // Test for VobSub magic number
                if cursor.iter().eq([0x00, 0x00, 0x01, 0xba].iter()) {
                    Some(SubtitleFormat::VobSubSub)
                } else {
                    Some(SubtitleFormat::MicroDVD)
                }
            }
            Some("srt") => Some(SubtitleFormat::SubRip),
            Some("idx") => Some(SubtitleFormat::VobSubIdx),
            Some("ssa") | Some("ass") => Some(SubtitleFormat::SubStationAlpha),
            _ => None,
        };
    }

    return None;
}
