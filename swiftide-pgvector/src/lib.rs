mod persist;
mod retrieve;

use anyhow::Result;
use derive_builder::Builder;
use sqlx::PgPool;
use swiftide_core::Persist;

#[derive(Builder, Clone, Debug)]
pub struct PgVector {
    pool: PgPool,
    #[builder(default = "String::from(\"swiftide_reg\")")]
    table_name: String,
    vector_size: i32,
    #[builder(default = "128")]
    batch_size: usize,
}

impl PgVector {
    pub async fn try_new(pool: PgPool, vector_size: i32) -> Result<Self> {
        let vector = PgVectorBuilder::default()
            .pool(pool)
            .vector_size(vector_size)
            .build()?;
        vector.setup().await?;

        Ok(vector)
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn swiftide_pgvector_should_work() {}
}
