use crate::pipeline::{marker::Map, PipelineT};

use super::ReadyService;

impl<S, F> ReadyService for PipelineT<S, F, Map>
where
    S: ReadyService,
{
    type Ready = S::Ready;

    #[inline]
    async fn ready(&self) -> Self::Ready {
        self.first.ready().await
    }
}
