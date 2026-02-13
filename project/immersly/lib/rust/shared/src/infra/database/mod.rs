use std::path::Path;

use prisma_client::PrismaClient;
pub use prisma_client::QueryError;
pub use prisma_client::model;

#[derive(Clone)]
pub struct Sqlite {
    client: PrismaClient,
}

impl Sqlite {
    pub async fn new<P: AsRef<Path>>(db_file: P) -> Result<Self, Box<dyn core::error::Error>> {
        let client = PrismaClient::builder()
            .with_url(format!(
                "file://{}",
                db_file.as_ref().to_str().ok_or("Invalid db file path")?
            ))
            .build()
            .await?;

        Ok(Self { client })
    }

    pub async fn migrate(&self) -> Result<(), Box<dyn core::error::Error>> {
        self.client.into_migrator().migrate_deploy().await?;

        Ok(())
    }

    pub fn client(&self) -> &PrismaClient {
        &self.client
    }
}
