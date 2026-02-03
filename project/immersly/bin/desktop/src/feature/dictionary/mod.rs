use std::sync::Arc;

use app::dataview::definition::DefinitionDataView;
use app::dataview::definitions::DefinitionsDataView;
use app::service::dictionary::DictionaryService;
use app::service::dictionary_lookup::DictionaryLookupService;
use app::task::load_dictionary::LoadDictionaryTask;
use app::use_case::get_exact_word::GetExactWordQuery;
use app::use_case::get_word::GetWordQuery;
use app::use_case::import_dictionary::ImportDictionaryUseCase;
use app::use_case::list_dictionaries::ListDictionariesQuery;
use features::shared::infra::http::AppState;
use infra::handler::get_exact_word;
use infra::handler::get_word;
use infra::handler::import_dictionary;
use infra::handler::list_dictionaries;
use infra::ranker::dictionary::DictionaryRanker;
use infra::repository::dictionary::DictionaryRepository;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use railgun_di::Injector;
use railgun_di::InjectorBuilder;
use railgun_di::InjectorError;

use crate::startup::Feature;

pub mod app;
pub mod domain;
mod infra;

pub struct DictionaryFeature {}

impl DictionaryFeature {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Feature for DictionaryFeature {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add::<DictionaryService>()?
            .add::<DictionaryLookupService>()?
            .add::<DictionaryRanker>()?
            .add::<DictionaryRepository>()?
            .add::<LoadDictionaryTask>()?
            .add::<ImportDictionaryUseCase>()?
            .add::<GetExactWordQuery>()?
            .add::<GetWordQuery>()?
            .add::<ListDictionariesQuery>()?
            .add::<DefinitionDataView>()?
            .add::<DefinitionsDataView>()?;

        Ok(())
    }

    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router
            .procedure(
                "dictionary:GetExactWord",
                procedure.query(get_exact_word::handler),
            )
            .procedure("dictionary:GetWord", procedure.query(get_word::handler))
            .procedure(
                "dictionary:ImportDictionary",
                procedure.mutation(import_dictionary::handler),
            )
            .procedure(
                "dictionary:ListDictionaries",
                procedure.query(list_dictionaries::handler),
            )
    }
}
