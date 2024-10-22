/// Guard tracking that a caller has entered a blocking region.
#[must_use]
pub(crate) struct BlockingRegionGuard {}

impl BlockingRegionGuard {
    pub(super) fn new() -> BlockingRegionGuard {
        BlockingRegionGuard { /* _p: PhantomData  */}
    }
}
