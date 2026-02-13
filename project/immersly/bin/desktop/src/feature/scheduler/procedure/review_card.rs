use core::result::Result;
use std::sync::Arc;

use railgun_di::Component;

use crate::feature::scheduler::domain::card::Card;
use crate::feature::scheduler::domain::state::NextStates;
use crate::feature::scheduler::repository::scheduler::SchedulerRepository;
use crate::feature::scheduler::service::scheduler::SchedulerService;
use features::shared::infra::Procedure;

pub struct Req {}

pub struct Res {
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
    type Req = Req;
    type Res = Option<Res>;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let result = self
            .scheduler_repository
            .reader()
            .get_next_due()
            .await
            .unwrap();

        if result.is_none() {
            return Ok(None);
        }

        let result: Card = result.unwrap().try_into().unwrap();

        let scheduler = self.scheduler_service.get_scheduler();
        let next_states = scheduler.get_next_states(&result);

        Ok(Some(Res {
            card: result,
            next_states,
        }))
    }
}
