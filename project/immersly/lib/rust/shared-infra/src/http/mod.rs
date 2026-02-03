use std::sync::Arc;

use railgun::di::Injector;

#[derive(Clone)]
pub struct AppState {
    pub injector: Arc<Injector>,
}
