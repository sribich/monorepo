use std::future::Future;

use query_core::TransactionOptions;
use query_core::protocol::EngineProtocol;

use super::client::ExecutionEngine;
use super::client::InternalClient;
use super::client::PrismaClient;
use super::query::QueryError;

pub struct TransactionBuilder<'client, TClient> {
    client: &'client TClient,
    internal_client: &'client InternalClient,
    timeout: u64,
    max_wait: u64,
    isolation_level: Option<String>,
}

impl<'client, TClient: PrismaClient> TransactionBuilder<'client, TClient> {
    pub fn new(client: &'client TClient, internal_client: &'client InternalClient) -> Self {
        Self {
            client,
            internal_client,
            timeout: 5000,
            max_wait: 2000,
            isolation_level: None,
        }
    }

    pub fn with_timeout(self, timeout: u64) -> Self {
        Self { timeout, ..self }
    }

    pub fn with_max_wait(self, max_wait: u64) -> Self {
        Self { max_wait, ..self }
    }

    // pub fn with_isolation_level(self, isolation_level: impl IsolationLevel) ->
    // Self {     Self {
    //         isolation_level: Some(isolation_level.to_string()),
    //         ..self
    //     }
    // }

    pub async fn run<TErr, TRet, TFut, TFn>(self, tx: TFn) -> Result<TRet, TErr>
    where
        TFut: Future<Output = Result<TRet, TErr>>,
        TFn: FnOnce(TClient) -> TFut,
        TErr: From<QueryError>,
    {
        match self.internal_client.engine() {
            ExecutionEngine::Real { context, .. } => {
                let new_tx_id = context
                    .executor
                    .start_tx(
                        context.query_schema.clone(),
                        EngineProtocol::Json,
                        TransactionOptions::new(self.max_wait, self.timeout, self.isolation_level),
                    )
                    .await
                    .map_err(|e| QueryError::Execute(e.into()))?;

                match tx(self.client.with_tx_id(Some(new_tx_id.clone()))).await {
                    result @ Ok(_) => {
                        context
                            .executor
                            .commit_tx(new_tx_id)
                            .await
                            .map_err(|e| QueryError::Execute(e.into()))?;

                        result
                    }
                    err @ Err(_) => {
                        context.executor.rollback_tx(new_tx_id).await.ok();

                        err
                    }
                }
            }
            // _ => tx(self.client.with_tx_id(None)).await,
        }
    }

    // pub async fn begin(self) -> super::Result<(TransactionController<TClient>,
    // TClient)> {     Ok(match &self.internals.engine {
    //         ExecutionEngine::Real { connector, .. } => {
    //             let new_tx_id = connector
    //                 .executor
    //                 .start_tx(
    //                     connector.query_schema.clone(),
    //                     EngineProtocol::Graphql,
    //                     TransactionOptions::new(self.max_wait, self.timeout,
    // self.isolation_level),                 )
    //                 .await
    //                 .map_err(|e| QueryError::Execute(e.into()))?;
    //
    //             (
    //                 TransactionController::new(new_tx_id.clone()),
    //                 self.client.with_tx_id(Some(new_tx_id)),
    //             )
    //         },
    //         _ => (
    //             TransactionController::new("".to_string().into()),
    //             self.client.with_tx_id(None),
    //         ),
    //     })
    // }
}
