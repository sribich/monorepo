use std::sync::Arc;

use app::{
    command::{add_card::AddCardCommand, score::ScoreCommand},
    query::{play_audio::PlayAudioQuery, study::StudyQuery},
};
pub use infra::handler::get_study_router;
use railgun_di::Injector;

pub mod app;
mod domain;
mod infra;

pub struct StudyDomain {
    //=========
    // Provides
    //=========
    add_card_command: Arc<AddCardCommand>,
    play_audio_query: Arc<PlayAudioQuery>,
    score_command: Arc<ScoreCommand>,
    study_query: Arc<StudyQuery>,
    //=========
    // Requires
    //=========
}

pub fn get_study_domain(injector: &mut Injector) -> StudyDomain {
    StudyDomain {
        add_card_command: injector.provide_by(AddCardCommand::new),
        play_audio_query: injector.provide_by(PlayAudioQuery::new),
        score_command: injector.provide_by(ScoreCommand::new),
        study_query: injector.provide_by(StudyQuery::new),
    }
}
