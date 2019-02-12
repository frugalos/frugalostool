extern crate frugalostool;
extern crate libfrugalos;
extern crate sloggers;
extern crate structopt;
#[macro_use]
extern crate trackable;

use frugalostool::command;
use frugalostool::command::object::*;
use frugalostool::Result;
use sloggers::Build;
use structopt::StructOpt;

#[allow(clippy::cyclomatic_complexity)]
fn main() -> Result<()> {
    let options = command::ApplicationOptions::from_args();
    let logger_builder = if let Some(filepath) = options.global.log_file {
        let mut builder = sloggers::file::FileLoggerBuilder::new(filepath);
        builder.level(options.global.log_level);
        builder.channel_size(options.global.max_concurrent_logs);
        sloggers::LoggerBuilder::File(builder)
    } else {
        let mut builder = sloggers::terminal::TerminalLoggerBuilder::new();
        builder.level(options.global.log_level);
        builder.channel_size(options.global.max_concurrent_logs);
        sloggers::LoggerBuilder::Terminal(builder)
    };
    let logger = track!(logger_builder.build())?;

    // SubCommands
    match options.command {
        command::SubCommandOptions::DeleteObjectsByIds {
            rpc_addr,
            bucket,
            device,
            delimiter,
            object_ids,
            ..
        } => {
            let object_ids = command::parse_object_ids(&object_ids, &delimiter);
            let context = command::OneshotCommandContext::new(logger.clone(), rpc_addr)?;
            track!(DeleteObjectsByIds::new(context).run(bucket, device, object_ids))?;
        }
        command::SubCommandOptions::Head {
            rpc_addr,
            bucket,
            object_id,
        } => {
            let context = command::OneshotCommandContext::new(logger.clone(), rpc_addr)?;

            let result = track!(Head::new(context).run(bucket, object_id))?;

            println!("result = {:?}", result);
        }
        command::SubCommandOptions::MdsHead {
            rpc_addr,
            bucket,
            object_id,
        } => {
            let context = command::OneshotCommandContext::new(logger.clone(), rpc_addr)?;

            let result = track!(MdsHead::new(context).run(bucket, object_id))?;

            println!("result = {:?}", result);
        }
    }

    // FIXME
    // NOTE: ログ出力(非同期)用に少し待機
    std::thread::sleep(std::time::Duration::from_millis(100));

    Ok(())
}
