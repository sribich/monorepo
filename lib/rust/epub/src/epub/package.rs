use super::v2;
use super::v3;

#[derive(Clone)]
pub enum Package {
    V2(v2::Package),
    V3(v3::Package),
}
