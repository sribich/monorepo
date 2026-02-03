use fsrs::{MemoryState, NextStates};
use prisma_client::model;

pub struct CardService {}

impl CardService {
    pub fn get_fsrs() -> fsrs::FSRS {
        fsrs::FSRS::new(Some(&[
            // 0.2890, 0.5927, 1.7414, 29.0102, 5.2709, 1.5568, 1.1732, 0.0007, 1.4568, 0.0984,
            // 0.8551, 2.1403, 0.0472, 0.3630, 1.5474, 0.2685, 2.8755,
            0.2890, 0.5927, 1.7414, 29.0102, 5.2709, 1.5568, 1.1732, 0.0007, 1.4568, 0.0984, 0.8551,
            2.1403, 0.0472, 0.3630, 1.5474, 0.2685, 2.8755,
        ]))
        .unwrap()
    }

    pub fn get_next_states(card: &model::card::Data, now: i64) -> NextStates {
        let fsrs = Self::get_fsrs();

        let memory_state = if let Some(stability) = card.stability
            && let Some(difficulty) = card.difficulty
        {
            Some(MemoryState {
                stability: stability as f32,
                difficulty: difficulty as f32,
            })
        } else {
            None
        };

        let last_review = card
            .last_review
            .map(|time| {
                let diff = chrono::DateTime::from_timestamp(now, 0).unwrap()
                    - chrono::DateTime::from_timestamp(time, 0).unwrap();

                diff.num_days() as u32
            })
            .unwrap_or(0);

        fsrs.next_states(memory_state, 0.95, last_review).unwrap()
    }
}
