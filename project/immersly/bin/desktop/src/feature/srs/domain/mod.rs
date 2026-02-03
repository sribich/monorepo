// add_card
// edit_card
// schedule_card
// review_card
//

// User Settings
//
//   1. Learning steps 1m 15m
//   2. Relearning steps 30m
//   3. Day ends at
//
//   - Cap answer time
//
//   4. Suspend card
//   5. Graduate card

trait Scheduler {
    const VERSION: &'static str;
}

// struct Scheduler {}

struct Card {}

struct Review {
    pub id: usize,
    pub card_id: usize,

    pub rating: Rating,
    // interval
    // last_interval
    // ease_

    // time_taken (ms)
    // review_kind
}

enum ReviewKind {
    Learning,
    Review,
    Relearning,
}

enum Rating {
    Again = 0,
    Hard = 1,
    Good = 2,
    Easy = 3,
}

struct Card {}

struct ReviewId(Muid);
struct CardId(Muid);

struct Review {
    id: ReviewId,
    card_id: CardId,

    rating: Rating,

    date: usize,
    // time_taken_ms
    // timestamp
}

// A card goes to the scheduler
// Deck
//
//
//

// Card

// model Card {
// id  Bytes @id
//
// due BigInt
// last_review BigInt?
//
// word String
//
// reading String?
// reading_audio String?
//
// sentence String?
// sentence_audio String?
//
// definition_native String?
// definition_tl String?
//
// stability Float?
// difficulty Float?
// comment String?
//
// reviews CardReview[]
// }
//
// model CardReview {
// id Bytes @id
//
// date   BigInt
// rating Int
//
// card_id Bytes
// card    Card @relation(fields: [card_id], references: [id], onDelete: Cascade, onUpdate: Cascade)
// }
