use crate::PgVector;
use anyhow::Result;
use async_trait::async_trait;
use pgvector::Vector;
use sqlx::{prelude::FromRow, types::Uuid};
use swiftide_core::{
    Retrieve,
    querying::{Document, Query, search_strategies::SimilaritySingleEmbedding, states},
};
use tracing::info;

const DEFAULT_LIMIT: usize = 5;

#[allow(unused)]
#[derive(Debug, Clone, FromRow)]
pub struct RetrievalResult {
    id: Uuid,
    chunk: String,
}

#[async_trait]
impl Retrieve<SimilaritySingleEmbedding<String>> for PgVector {
    #[tracing::instrument]
    async fn retrieve(
        &self,
        search_strategy: &SimilaritySingleEmbedding<String>,
        query_state: Query<states::Pending>,
    ) -> Result<Query<states::Retrieved>> {
        let embedding = if let Some(embedding) = query_state.embedding.as_ref() {
            Vector::from(embedding.clone())
        } else {
            return Err(anyhow::Error::msg("Missing embedding in query state"));
        };

        let pool = self.get_pool();

        let sql = format!(
            "SELECT chunk FROM {} ORDER BY embedding <=> $1 LIMIT $2",
            self.table_name,
        );
        info!("Running retrieve with SQL: {sql}");
        let data: Vec<RetrievalResult> = sqlx::query_as(&sql)
            .bind(embedding)
            .bind(DEFAULT_LIMIT as i32)
            .fetch_all(pool)
            .await?;

        let docs = data.into_iter().map(|r| Document::from(r.chunk)).collect();

        Ok(query_state.retrieved_documents(docs))
    }
}
