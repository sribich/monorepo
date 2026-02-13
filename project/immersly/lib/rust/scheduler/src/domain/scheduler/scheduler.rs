/*
use std::marker::PhantomData;

trait State {}

struct NewState;
struct LearningState(usize);
struct ReviewState;
struct RelearningState(usize);
struct GraduatedState;

impl State for NewState {}
impl State for LearningState {}
impl State for ReviewState {}
impl State for RelearningState {}
impl State for GraduatedState {}

struct Scheduler<T: State> {
    _marker: PhantomData<fn(T) -> T>,
}

impl<T: State> Scheduler<T> {}

impl Scheduler<NewState> {}

impl Scheduler<LearningState> {}

impl Scheduler<ReviewState> {}

impl Scheduler<RelearningState> {}

impl Scheduler<GraduatedState> {}
*/
