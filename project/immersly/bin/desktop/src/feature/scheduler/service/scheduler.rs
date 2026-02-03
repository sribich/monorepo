use railgun_di::Component;

use crate::feature::scheduler::domain::scheduler::LEARNING_STEPS;
use crate::feature::scheduler::domain::scheduler::RELEARNING_STEPS;
use crate::feature::scheduler::domain::scheduler::Scheduler;
use crate::feature::scheduler::domain::scheduler::SchedulerState;
use crate::feature::scheduler::domain::steps::Steps;
use crate::feature::scheduler::domain::timer::Timer;

#[derive(Component)]
pub struct SchedulerService {}

impl SchedulerService {
    pub fn get_scheduler(&self) -> Scheduler {
        Scheduler::new(self.scheduler_state())
    }

    fn scheduler_state(&self) -> SchedulerState {
        let desired_retention = 0.9;
        let fsrs = fsrs::FSRS::new(Some(&[
            0.3483, 0.7196, 2.1369, 30.3239, 7.2501, 0.4343, 2.1942, 0.0222, 1.4094, 0.0817,
            0.8070, 1.8749, 0.2283, 0.4387, 1.4147, 0.5817, 2.9898, 1.0147, 0.3329,
        ]))
        .unwrap();

        SchedulerState {
            desired_retention,
            fsrs,
            learning_steps: Steps::new(LEARNING_STEPS.to_vec()),
            relearning_steps: Steps::new(RELEARNING_STEPS.to_vec()),
            timer: Timer::new(),
        }
    }
}
