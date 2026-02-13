use core::result::Result;
use std::sync::Arc;

use railgun_di::Component;

use crate::feature::scheduler::domain::card::Card;
use crate::feature::scheduler::domain::card::CardId;
use crate::feature::scheduler::domain::state::NextState;
use crate::feature::scheduler::repository::scheduler::SchedulerRepository;
use features::shared::infra::Procedure;

pub struct Req {
    pub id: CardId,
    pub answer: NextState,
}

pub struct Res {}

#[derive(Component)]
pub struct AnswerCardProcedure {
    scheduler_repository: Arc<SchedulerRepository>,
}

impl Procedure for AnswerCardProcedure {
    type Err = core::convert::Infallible;
    type Req = Req;
    type Res = Res;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let mut result: Card = self
            .scheduler_repository
            .reader()
            .from_id(data.id)
            .await
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        println!("{result:#?}");
        data.answer.apply(&mut result);
        println!("{result:#?}");

        self.scheduler_repository
            .writer()
            .apply_learning_state(&result)
            .await;

        Ok(Self::Res {})
    }
}

/*


        let scheduler = self.scheduler_service.get_scheduler();
        let next_states = scheduler.get_next_states(&result);

        Ok(Self::Res {
            card: result,
            next_states,
        })
*/
