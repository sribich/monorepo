use core::result::Result;
use std::sync::Arc;

use railgun::di::Component;
use shared::infra::Procedure;

use crate::{domain::{state::NextState, value::card_id::CardId}, infra::repository::scheduler::SchedulerRepository};

pub struct AnswerCardReq {
    pub id: CardId,
    pub answer: NextState,
}

#[derive(Component)]
pub struct AnswerCardProcedure {
    scheduler_repository: Arc<SchedulerRepository>,
}

impl Procedure for AnswerCardProcedure {
    type Err = core::convert::Infallible;
    type Req = AnswerCardReq;
    type Res = ();

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let mut result = self
            .scheduler_repository
            .from_id(&data.id)
            .await
            .unwrap()
            .unwrap();

        data.answer.apply(&mut result);

        self.scheduler_repository
            .apply_learning_state(&result)
            .await;

        Ok(())
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
