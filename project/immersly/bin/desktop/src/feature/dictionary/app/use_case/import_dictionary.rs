use std::sync::Arc;

use railgun::error::Error;
use railgun::error::Location;
use railgun_di::Component;

use crate::domain::value::existing_path::ExistingPath;
use crate::feature::dictionary::app::task::load_dictionary::LoadDictionaryTask;
use crate::feature::dictionary::domain::value::language_type::LanguageType;
use crate::system::UseCase;

#[derive(Error)]
#[error(module)]
pub enum ImportDictionaryError {
    #[error(transparent)]
    Unknown {
        #[error(impl_from)]
        error: Box<dyn core::error::Error>,
        location: Location,
    },
}

#[derive(Component)]
pub struct ImportDictionaryUseCase {
    load_dictionary: Arc<LoadDictionaryTask>,
}

impl UseCase for ImportDictionaryUseCase {
    type Err = core::convert::Infallible;
    type Req = (ExistingPath, LanguageType);
    type Res = ();

    #[must_use]
    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let result = self.load_dictionary.send(data.0, data.1).await;

        // Ok(result.info_owned())

        Ok(())
    }
}
