//! This crate defines commands for `Object`.
use command::OneshotCommandContext;
use error::Error;
use fibers::{Executor, Spawn};
use futures::Future;
use libfrugalos::entity::bucket::BucketId;
use libfrugalos::entity::device::DeviceId;
use libfrugalos::entity::object::ObjectId;
use std::collections::BTreeSet;
use Result;

#[allow(missing_docs)]
pub struct DeleteObjectsByIds {
    context: OneshotCommandContext,
}

impl DeleteObjectsByIds {
    /// Creates a new instance of `DeleteObjectsByIds`.
    pub fn new(context: OneshotCommandContext) -> Self {
        DeleteObjectsByIds { context }
    }

    /// Deletes the objects from the given device.
    /// Objects which don't exist are simply ignored.
    pub fn run(
        &mut self,
        bucket_id: BucketId,
        device_id: DeviceId,
        object_ids: BTreeSet<ObjectId>,
    ) -> Result<()> {
        let fiber = self.context.executor.spawn_monitor(
            self.context
                .frugalos_client
                .delete_from_device_by_object_ids(bucket_id, device_id, object_ids)
                .map_err(|e| track!(e)),
        );
        self.context
            .executor
            .run_fiber(fiber)
            .unwrap()
            .map_err(|e| track!(Error::from(e)))
    }
}
