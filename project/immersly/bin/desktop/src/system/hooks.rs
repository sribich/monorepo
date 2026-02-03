use async_trait::async_trait;
use railgun_di::Injector;

use super::OnStartup;

#[async_trait]
pub trait LifecycleHooks {
    async fn run_startup_hooks(&self) -> Result<(), Box<dyn core::error::Error>>;
}

#[async_trait]
impl LifecycleHooks for Injector {
    async fn run_startup_hooks(&self) -> Result<(), Box<dyn core::error::Error>> {
        let hooks = self.get_vec::<dyn OnStartup>()?;

        for hook in hooks {
            hook.run().await?;
        }

        Ok(())
    }
}
