//! Stores a file
//!
use async_trait::async_trait;

use crate::shared::UseCase;

//==============================================================================
// Data
//==============================================================================
pub struct Req {}

pub struct Res {}

//==============================================================================
// UseCase
//==============================================================================
pub struct StoreFileUseCase {}

impl StoreFileUseCase {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UseCase for StoreFileUseCase {
    type Err = core::convert::Infallible;
    type Req = Req;
    type Res = Res;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        Ok(())
    }
}
