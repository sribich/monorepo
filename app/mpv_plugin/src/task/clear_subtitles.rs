use crate::actor::Task;
use crate::context::MpvContext;

pub struct ClearSubtitles {
    context: MpvContext,
}

impl ClearSubtitles {
    pub fn new(context: MpvContext) -> Self {
        Self { context }
    }
}

impl Task for ClearSubtitles {
    async fn execute(&self) {
        self.context.clear_subtitles().await;
    }
}
