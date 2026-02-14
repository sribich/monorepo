use std::sync::Arc;

use railgun::error::Error;
use railgun::error::Location;
use railgun::di::Component;
use shared::domain::value::existing_file::ExistingFile;
use shared::infra::Procedure;

use crate::app::task::load_dictionary::LoadDictionaryTask;
use crate::domain::value::language_type::LanguageType;

pub struct ImportDictionaryReq {
    pub dictionary_path: ExistingFile,
    pub language_type: LanguageType,
}

#[derive(Component)]
pub struct ImportDictionaryProcedure {
    load_dictionary: Arc<LoadDictionaryTask>,
}

impl Procedure for ImportDictionaryProcedure {
    type Err = core::convert::Infallible;
    type Req = ImportDictionaryReq;
    type Res = ();

    #[must_use]
    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        self.load_dictionary.send(data.dictionary_path, data.language_type).await;

        Ok(())
    }
}
