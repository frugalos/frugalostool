//! This crate defines commands for `Object`.
use fibers::Spawn;
use fibers_global;
use futures::Future;
use libfrugalos::entity::bucket::BucketId;
use libfrugalos::entity::device::DeviceId;
use libfrugalos::entity::object::ObjectId;

use std::collections::BTreeSet;

use command::OneshotCommandContext;
use error::Error;
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
        let fiber = fibers_global::handle().spawn_monitor(
            self.context
                .frugalos_client
                .delete_from_device_by_object_ids(bucket_id, device_id, object_ids)
                .map_err(|e| track!(e)),
        );
        fibers_global::execute(fiber).map_err(|e| track!(Error::from(e)))
    }
}
