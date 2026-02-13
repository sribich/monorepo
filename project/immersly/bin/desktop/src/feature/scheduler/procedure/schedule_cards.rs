use core::result::Result;
use std::sync::Arc;

use railgun_di::Component;

use crate::feature::scheduler::domain::card::Card;
use crate::feature::scheduler::repository::scheduler::SchedulerRepository;
use crate::feature::scheduler::service::scheduler::SchedulerService;
use features::shared::infra::Procedure;

pub struct Req {
    /// The number of new cards that should be scheduled.
    pub count: u32,
}

pub struct Res {
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
    type Req = Req;
    type Res = Res;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let unscheduled_cards = self
            .scheduler_repository
            .reader()
            .list_new_cards(data.count)
            .await
            .unwrap()
            .into_iter()
            .map(|it| it.try_into().unwrap())
            .collect::<Vec<Card>>();

        let scheduler = self.scheduler_service.get_scheduler();
        let cards = scheduler.schedule_new_cards(unscheduled_cards);

        self.scheduler_repository
            .writer()
            .schedule_new_cards(&cards)
            .await;

        Ok(Self::Res {
            count: cards.len() as u32,
        })
    }
}
