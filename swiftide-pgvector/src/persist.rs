use crate::PgVector;
use anyhow::Result;
use async_trait::async_trait;
use swiftide_core::{
    Persist,
    indexing::{IndexingStream, Node},
};

#[async_trait]
impl Persist for PgVector {
    #[tracing::instrument(skip_all)]
    async fn setup(&self) -> Result<()> {
        todo!()
    }

    #[tracing::instrument(skip_all)]
    async fn store(&self, node: Node) -> Result<Node> {
        // let mut nodes = vec![node; 1];
        // self.store_nodes(&nodes).await?;

        // let node = nodes.swap_remove(0);

        // Ok(node)
        todo!()
    }

    #[tracing::instrument(skip_all)]
    async fn batch_store(&self, nodes: Vec<Node>) -> IndexingStream {
        // self.store_nodes(&nodes).await.map(|()| nodes).into()
        todo!()
    }

    fn batch_size(&self) -> Option<usize> {
        self.batch_size
    }
}
