mod persist;
mod retrieve;

use anyhow::Result;
use derive_builder::Builder;
use sqlx::PgPool;

#[derive(Builder, Clone, Debug)]
pub struct PgVector {
    pool: PgPool,
    #[builder(default = "String::from(\"swiftide_reg\")")]
    table_name: String,
    vector_size: i32,
    #[builder(default = "Some(128)")]
    batch_size: Option<usize>,
}

impl PgVector {
    pub fn try_new(pool: PgPool, vector_size: i32) -> Result<Self> {
        Ok(PgVectorBuilder::default()
            .pool(pool)
            .vector_size(vector_size)
            .build()?)
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
