#![feature(ptr_metadata, try_as_dyn, vec_into_chunks, macro_metavar_expr_concat)]
use std::sync::Arc;

use app::procedure::get_exact_word::GetExactWordProcedure;
use app::procedure::get_word::GetWordProcedure;
use app::procedure::import_dictionary::ImportDictionaryProcedure;
use app::procedure::list_dictionaries::ListDictionariesProcedure;
use app::service::dictionary::DictionaryService;
use app::service::dictionary_lookup::DictionaryLookupService;
use app::task::load_dictionary::LoadDictionaryTask;
use infra::ranker::dictionary::DictionaryRanker;
use infra::repository::dictionary::DictionaryRepository;
use infra::repository::frequency::FrequencyRepository;
use infra::repository::pitch_accent::PitchAccentRepository;
use infra::repository::word::WordRepository;
use railgun::di::Container;
use railgun::di::InjectorBuilder;
use railgun::di::InjectorError;
use railgun::di::Routes;
use railgun::module;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use shared::infra::http::AppState;

pub mod app;
pub mod domain;
mod infra;

module!(DictionaryModule, AppState);

impl Container for DictionaryModule {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add::<DictionaryService>()?
            .add::<DictionaryLookupService>()?
            .add::<DictionaryRanker>()?
            .add::<DictionaryRepository>()?
            .add::<FrequencyRepository>()?
            .add::<GetExactWordProcedure>()?
            .add::<GetWordProcedure>()?
            .add::<ImportDictionaryProcedure>()? //
            .add::<ListDictionariesProcedure>()? //
            .add::<LoadDictionaryTask>()? //
            .add::<PitchAccentRepository>()? //
            .add::<WordRepository>()?; //

        Ok(())
    }
}

impl Routes<AppState> for DictionaryModule {
    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router
            .procedure(
                "dictionary:GetExactWord",
                procedure.query(infra::handler::get_exact_word::handler),
            )
            .procedure(
                "dictionary:GetWord",
                procedure.query(infra::handler::get_word::handler),
            )
            .procedure(
                "dictionary:ImportDictionary",
                procedure.mutation(infra::handler::import_dictionary::handler),
            )
            .procedure(
                "dictionary:ListDictionaries",
                procedure.query(infra::handler::list_dictionaries::handler),
            )
    }
}
