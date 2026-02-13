use core::result::Result;
use std::sync::Arc;

use railgun::di::Component;
use shared::infra::Procedure;

use crate::{app::service::scheduler::SchedulerService, domain::{entity::card::Card, state::NextStates}, infra::repository::scheduler::SchedulerRepository};

pub struct ReviewCardReq {}

pub struct ReviewCardRes {
    pub card: Card,
    pub next_states: NextStates,
}

#[derive(Component)]
pub struct ReviewCardProcedure {
    scheduler_repository: Arc<SchedulerRepository>,
    scheduler_service: Arc<SchedulerService>,
}

impl Procedure for ReviewCardProcedure {
    type Err = core::convert::Infallible;
    type Req = ReviewCardReq;
    type Res = Option<ReviewCardRes>;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let result = self
            .scheduler_repository
            .get_next_due()
            .await
            .unwrap();

        if result.is_none() {
            return Ok(None);
        }

        let result: Card = result.unwrap().try_into().unwrap();

        let scheduler = self.scheduler_service.get_scheduler();
        let next_states = scheduler.get_next_states(&result);

        Ok(Some(ReviewCardRes {
            card: result,
            next_states,
        }))
    }
}
