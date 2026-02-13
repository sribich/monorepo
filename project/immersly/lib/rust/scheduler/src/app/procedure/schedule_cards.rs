use core::result::Result;
use std::sync::Arc;

use railgun::di::Component;
use shared::infra::Procedure;

use crate::app::service::scheduler::SchedulerService;
use crate::infra::repository::scheduler::SchedulerRepository;

pub struct ScheduleCardsReq {
    /// The number of new cards that should be scheduled.
    pub count: u32,
}

pub struct ScheduleCardsRes {
    /// The number of cards that were scheduled.
    ///
    /// This may be lower than [`Req::count`] if the number of pending
    /// `new` cards is lower than the requested amount.
    pub count: u32,
}

#[derive(Component)]
pub struct ScheduleCardsProcedure {
    scheduler_repository: Arc<SchedulerRepository>,
    scheduler_service: Arc<SchedulerService>,
}

impl Procedure for ScheduleCardsProcedure {
    type Err = core::convert::Infallible;
    type Req = ScheduleCardsReq;
    type Res = ScheduleCardsRes;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let unscheduled_cards = self
            .scheduler_repository
            .list_new_cards(data.count)
            .await
            .unwrap();

        let scheduler = self.scheduler_service.get_scheduler();
        let cards = scheduler.schedule_new_cards(unscheduled_cards);

        self.scheduler_repository.schedule_new_cards(&cards).await;

        Ok(ScheduleCardsRes {
            count: cards.len() as u32,
        })
    }
}
