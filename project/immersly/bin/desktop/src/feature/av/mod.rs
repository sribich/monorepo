use procedure::ExtractAudioProcedure;
use railgun_di::Injector;
use railgun_di::InjectorBuilder;
use railgun_di::InjectorError;

use crate::startup::Feature;

pub mod procedure;

pub struct AudioVideoFeature {}

impl AudioVideoFeature {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Feature for AudioVideoFeature {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector.add::<ExtractAudioProcedure>()?;

        Ok(())
    }
}
