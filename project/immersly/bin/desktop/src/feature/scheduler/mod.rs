use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use handler::add_card_image;
use procedure::add_card_image::AddCardImageProcedure;
use procedure::answer_card::AnswerCardProcedure;
use procedure::create_card::CreateCardProcedure;
use procedure::play_card_audio::PlayCardAudioProcedure;
use procedure::review_card::ReviewCardProcedure;
use procedure::schedule_cards::ScheduleCardsProcedure;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use railgun_di::Injector;
use railgun_di::InjectorBuilder;
use railgun_di::InjectorError;
use repository::scheduler::SchedulerRepository;
use service::scheduler::SchedulerService;

use crate::AppState;
use crate::startup::Feature;

mod domain;
mod handler;
mod procedure;
mod repository;
mod service;

pub struct SchedulerFeature {}

impl SchedulerFeature {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Feature for SchedulerFeature {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add::<SchedulerRepository>()?
            .add::<SchedulerService>()?
            .add::<AnswerCardProcedure>()?
            .add::<CreateCardProcedure>()?
            .add::<PlayCardAudioProcedure>()?
            .add::<ReviewCardProcedure>()?
            .add::<ScheduleCardsProcedure>()?
            .add::<AddCardImageProcedure>()?;

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
                "scheduler:answerCard",
                procedure.mutation(handler::answer_card::handler),
            )
            .procedure(
                "scheduler:createCard",
                procedure.mutation(handler::create_card::handler),
            )
            .procedure(
                "scheduler:reviewCard",
                procedure.query(handler::review_card::handler),
            )
            .procedure(
                "scheduler:scheduleCards",
                procedure.mutation(handler::schedule_cards::handler),
            )
            .procedure(
                "scheduler:addCardImage",
                procedure.mutation(handler::add_card_image::handler),
            )
            .apply(|router| {
                router.route(
                    "/scheduler:playAudio/{id}/{kind}",
                    axum::routing::get(handler::play_card_audio::handler).with_state(state),
                )
            })
        // .apply(|router| {
        //     // .procedure("library:EditTitle", procedure.mutation(edit_title::handler))
        //     router.nest_service(
        //         "/edit_title/{id}",
        //         axum::routing::post(add_card_image::handler)
        //             .with_state(Arc::clone(&state))
        //             .layer(DefaultBodyLimit::max(1024 * 1024 * 1024 * 1024)),
        //     )
        // })
    }
}
