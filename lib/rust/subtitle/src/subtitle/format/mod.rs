pub mod srt;
pub mod util;

#[derive(Clone, Copy, Debug)]
pub enum SubtitleFormat {
    // .srt
    SubRip,
    // .ssa/ass
    SubStationAlpha,
    // .idx
    VobSubIdx,
    // .sub
    VobSubSub,
    // .sub
    MicroDVD,
}
/*
#[derive(Clone, Debug)]
pub enum SubtitleFile {

}

impl SubtitleFile {
    pub fn get_subtitle_entries(&self) -> Result<Vec<SubtitleEntry>, ()> {
        match self {
            SubtitleFile::SubRipFile(file) => file.get_subtitle_entries(),
        }
    }
}
*/
