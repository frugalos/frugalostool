//! command definitions
use error::Error;
use error::ErrorKind;
use fibers::{Executor, Spawn, ThreadPoolExecutor};
use fibers_rpc::client::ClientServiceBuilder as RpcServiceBuilder;
use futures::Future;
use libfrugalos::client::frugalos::Client;
use libfrugalos::entity::bucket::BucketId;
use libfrugalos::entity::device::DeviceId;
use libfrugalos::entity::object::ObjectId;
use slog::Logger;
use sloggers::types::Severity;
use std::collections::BTreeSet;
use std::net::SocketAddr;
use std::path::PathBuf;
use structopt::StructOpt;
use trackable::error::ErrorKindExt;
use Result;

pub mod object;

#[allow(missing_docs)]
pub struct OneshotCommandContext {
    pub logger: Logger,
    pub executor: ThreadPoolExecutor,
    pub frugalos_client: Client,
}

impl OneshotCommandContext {
    #[allow(missing_docs)]
    pub fn new(logger: Logger, rpc_addr: SocketAddr) -> Result<Self> {
        let executor = track!(ThreadPoolExecutor::with_thread_count(1).map_err(Error::from))?;
        let rpc_service = RpcServiceBuilder::new()
            .logger(logger.clone())
            .finish(executor.handle());
        let rpc_service_handle = rpc_service.handle();
        let frugalos_client = Client::new(rpc_addr, rpc_service_handle);

        executor.spawn(rpc_service.map_err(|e| panic!("{}", e)));

        Ok(OneshotCommandContext {
            logger,
            executor,
            frugalos_client,
        })
    }
}

/// Options for this application.
/// See https://github.com/TeXitoi/structopt/tree/master/examples.
#[derive(Debug, StructOpt)]
#[allow(missing_docs)]
pub struct ApplicationOptions {
    #[structopt(flatten)]
    pub global: GlobalOptions,

    #[structopt(subcommand)]
    pub command: SubCommandOptions,
}

/// Options for all commands.
#[derive(Debug, StructOpt)]
#[allow(missing_docs)]
pub struct GlobalOptions {
    #[structopt(
        short = "l",
        long = "loglevel",
        default_value = "info",
        parse(try_from_str = "parse_log_level")
    )]
    pub log_level: Severity,

    #[structopt(long = "logfile", parse(from_os_str))]
    pub log_file: Option<PathBuf>,

    #[structopt(long = "max-concurrent-logs", default_value = "4096")]
    pub max_concurrent_logs: usize,
}

/// Options for sub-commands.
#[derive(Debug, StructOpt)]
#[allow(missing_docs)]
pub enum SubCommandOptions {
    #[structopt(name = "delete-objects-by-ids")]
    DeleteObjectsByIds {
        #[structopt(long = "rpc-addr", default_value = "127.0.0.1:14278")]
        rpc_addr: SocketAddr,

        #[structopt(long = "bucket")]
        bucket: BucketId,

        #[structopt(long = "device")]
        device: DeviceId,

        #[structopt(long = "delimiter", default_value = ",")]
        delimiter: String,

        /// It seems to be impossible to reference the given delimiter in this context,
        /// so accepts String here and parses it manually.
        /// `structopt(raw(use_delimiter = true, delimiter_value = ","))` is
        /// insufficient because an `ObjectId` may contain an arbitrary character.
        #[structopt(long = "object-ids", default_value = "")]
        object_ids: String,
    },
}

/// Parses `ObjectVersion`s from a string.
pub fn parse_object_ids(raw: &str, delimiter: &str) -> BTreeSet<ObjectId> {
    raw.split(delimiter).map(|id| id.to_owned()).collect()
}

/// Parses `Severity` from a string.
fn parse_log_level(raw: &str) -> Result<Severity> {
    Ok(match raw {
        "trace" => Severity::Trace,
        "debug" => Severity::Debug,
        "info" => Severity::Info,
        "warn" => Severity::Warning,
        "critical" => Severity::Critical,
        "error" => Severity::Error,
        _ => {
            return Err(ErrorKind::InvalidInput
                .cause(format!("Unrecognized: {}", raw))
                .into())
        }
    })
}
