use std::sync::Arc;

use async_trait::async_trait;
use features::shared::OnStartup;
use railgun_di::Injector;

#[async_trait]
pub trait LifecycleHooks {
    async fn run_startup_hooks(&self) -> Result<(), Box<dyn core::error::Error>>;
}

#[async_trait]
impl LifecycleHooks for Injector {
    async fn run_startup_hooks(&self) -> Result<(), Box<dyn core::error::Error>> {
        let hooks = self.get_vec::<dyn OnStartup>()?;

        for mut hook in hooks {
            assert!(Arc::strong_count(&hook) == 2, "Startup hooks broken");

            let hook = unsafe { Arc::get_mut_unchecked(&mut hook) };

            hook.run().await?;
        }

        Ok(())
    }
}
