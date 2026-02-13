#![feature(ptr_metadata, try_as_dyn)]
use std::sync::Arc;

use app::procedure::add_card_image::AddCardImageProcedure;
use app::procedure::answer_card::AnswerCardProcedure;
use app::procedure::create_card::CreateCardProcedure;
use app::procedure::review_card::ReviewCardProcedure;
use app::procedure::schedule_cards::ScheduleCardsProcedure;
use app::service::scheduler::SchedulerService;
use infra::repository::scheduler::SchedulerRepository;
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

module!(SchedulerModule, AppState);

impl Container for SchedulerModule {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add::<SchedulerRepository>()?
            .add::<SchedulerService>()?
            .add::<AnswerCardProcedure>()?
            .add::<CreateCardProcedure>()?
            .add::<ReviewCardProcedure>()?
            .add::<ScheduleCardsProcedure>()?
            .add::<AddCardImageProcedure>()?;

        Ok(())
    }
}

impl Routes<AppState> for SchedulerModule {
    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router
            .procedure(
                "scheduler:answerCard",
                procedure.mutation(infra::handler::answer_card::handler),
            )
            .procedure(
                "scheduler:createCard",
                procedure.mutation(infra::handler::create_card::handler),
            )
            .procedure(
                "scheduler:reviewCard",
                procedure.query(infra::handler::review_card::handler),
            )
            .procedure(
                "scheduler:scheduleCards",
                procedure.mutation(infra::handler::schedule_cards::handler),
            )
            .procedure(
                "scheduler:addCardImage",
                procedure.mutation(infra::handler::add_card_image::handler),
            )
    }
}
