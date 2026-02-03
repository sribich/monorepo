use std::{marker::PhantomData, sync::Arc};

use prisma_client::{PrismaClient, QueryError, model};
use railgun_di::Component;

use crate::{
    feature::library::domain::{aggregate::series::Series, cdc::SeriesChange},
};

pub struct Reader;
pub struct Writer;

#[derive(Component)]
pub struct SeriesRepository<T = ()>
where
    T: 'static + Send + Sync,
{
    db: Arc<Sqlite>,
    _phantom: PhantomData<T>,
}

impl SeriesRepository<()> {
    pub fn reader(&self) -> SeriesRepository<Reader> {
        SeriesRepository {
            db: Arc::clone(&self.db),
            _phantom: Default::default(),
        }
    }

    pub fn writer(&self) -> SeriesRepository<Writer> {
        SeriesRepository {
            db: Arc::clone(&self.db),
            _phantom: Default::default(),
        }
    }
}

impl SeriesRepository<Reader> {
    pub async fn list(&self) -> Vec<Series> {
        let result = self
            .db
            .client()
            .library()
            .find_many(vec![])
            .exec()
            .await
            .unwrap();

        SeriesTransformer::transform_many(result)
    }
}

impl SeriesRepository<Writer> {
    pub async fn save(&self, mut series: Series) {
        let transaction = self.db.client()._transaction();

        transaction
            .run(|client| async move {
                for event in series.change_events() {
                    match event {
                        SeriesChange::Created(series) => {
                            self.create_series(&client, series).await?;
                        },
                    }
                }

                Ok(()) as Result<(), QueryError>
            })
            .await
            .unwrap();
    }

    async fn create_series(
        &self,
        client: &PrismaClient,
        series: Series,
    ) -> core::result::Result<(), QueryError> {
        client
            .library()
            .create(
                series.id().as_bytes().to_vec(),
                series.title().to_owned(),
                vec![],
            )
            .exec()
            .await?;

        Ok(())
    }
}

struct SeriesTransformer {}

impl SeriesTransformer {
    fn transform(data: model::library::Data) -> Series {
        Series::from_parts(
            Muid::from_slice_unchecked(&data.id),
            data.title,
            data.image_resource_id
                .map(|it| Muid::from_slice_unchecked(&it),
        )
    }

    fn transform_many(data: Vec<model::library::Data>) -> Vec<Series> {
        data.into_iter()
            .map(SeriesTransformer::transform)
            .collect::<Vec<_>>()
    }
}
