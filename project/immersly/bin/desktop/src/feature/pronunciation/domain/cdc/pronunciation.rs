use crate::feature::pronunciation::domain::aggregate::pronunciation::Pronunciation;

#[derive(Clone)]
pub enum PronunciationChangeCapture {
    Created(Pronunciation),
}
