use std::fmt::Debug;

use crate::IsSegment;
use crate::Segment;

struct SegmentAligner<TSegmentData, TSourceA, TSourceB>
where
    TSegmentData: IsSegment + Debug,
    TSourceA: Debug,
    TSourceB: Debug,
{
    a: Vec<Segment<TSegmentData, TSourceA>>,
    b: Vec<Segment<TSegmentData, TSourceB>>,
    // options: SegmentAlignerOptions,
    // matches: Vec<MatchKind>,
}
