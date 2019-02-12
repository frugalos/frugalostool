//! This crate defines commands for `Object`.
use fibers::Spawn;
use fibers_global;
use futures::Future;
use libfrugalos::entity::bucket::BucketId;
use libfrugalos::entity::device::DeviceId;
use libfrugalos::entity::object::{ObjectId, ObjectVersion};

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

/// This command issues Head rpc with a given bucket_id and object_id.
pub struct Head {
    context: OneshotCommandContext,
}

impl Head {
    /// Creates a new instance of `MdsHead`.
    pub fn new(context: OneshotCommandContext) -> Self {
        Head { context }
    }

    /// Issues Head with the given `object_id` of the given `bucket_id`.
    pub fn run(
        &mut self,
        bucket_id: BucketId,
        object_id: ObjectId,
    ) -> Result<Option<ObjectVersion>> {
        use std::time::Duration;

        let fiber = fibers_global::handle().spawn_monitor(
            self.context
                .frugalos_client
                .head_object(
                    bucket_id,
                    object_id,
                    Duration::from_secs(30),
                    Default::default(),
                )
                .map_err(|e| track!(e)),
        );
        fibers_global::execute(fiber).map_err(|e| track!(Error::from(e)))
    }
}

/// This command issues MdsHead rpc with a given bucket_id and object_id.
pub struct MdsHead {
    context: OneshotCommandContext,
}

impl MdsHead {
    /// Creates a new instance of `MdsHead`.
    pub fn new(context: OneshotCommandContext) -> Self {
        MdsHead { context }
    }

    /// Issues MdsHead with the given `object_id` of the given `bucket_id`.
    pub fn run(
        &mut self,
        bucket_id: BucketId,
        object_id: ObjectId,
    ) -> Result<Option<ObjectVersion>> {
        use std::time::Duration;

        let fiber = fibers_global::handle().spawn_monitor(
            self.context
                .frugalos_client
                .mds_head_object(
                    bucket_id,
                    object_id,
                    Duration::from_secs(30),
                    Default::default(),
                )
                .map_err(|e| track!(e)),
        );
        fibers_global::execute(fiber).map_err(|e| track!(Error::from(e)))
    }
}
