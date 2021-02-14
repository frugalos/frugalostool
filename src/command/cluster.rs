//! This module provides some ways to control a frugalos cluster.
use std::io::{self, Write};

use error::Error;
use Result;

/// A command for creating `docker-compose.yml`.
/// There is no standard method for generating `docker-compose.yml` programatically,
/// so this struct gives some help.
pub struct CreateDockerCompose;

impl CreateDockerCompose {
    /// Creates a new `CreateDockerCompose`.
    pub fn new() -> Self {
        Self
    }

    /// Runs this command.
    pub fn run(
        &self,
        cluster_size: u8,
        cluster_addr_start: u8,
        node_index_start: u8,
    ) -> Result<()> {
        DockerComposeFile::new(cluster_addr_start)
            .add_nodes(cluster_size, node_index_start)
            .write_to(&mut io::stdout())
    }
}

/// A content of `docker-compose.yml`.
struct DockerComposeFile {
    buffer: String,
    addr_start: u8,
}

impl DockerComposeFile {
    fn new(addr_start: u8) -> Self {
        let mut buffer = String::new();
        buffer.push_str("version: '2'\n");
        buffer.push_str(&format!(
            "
networks:
  frugalos_net:
    ipam:
      driver: default
      config:
        - subnet: 172.18.0.0/16
"
        ));
        buffer.push_str("\n");
        buffer.push_str("services:");
        Self { buffer, addr_start }
    }

    /// Adds frugalos nodes to a content.
    fn add_nodes(mut self, cluster_size: u8, node_index_start: u8) -> Self {
        for i in node_index_start..(node_index_start + cluster_size) {
            let data_dir = format!("/tmp/frugalos_it/frugalos{}/", i);
            let hostname = format!("frugalos{}", i);
            let command = if i == node_index_start {
                format!("bootstrap.sh")
            } else {
                format!("join.sh")
            };
            let addr = format!("172.18.0.{}", self.addr_start);
            let depends_on = if i == node_index_start {
                format!("")
            } else {
                format!("depends_on: \n      - frugalos{}", node_index_start)
            };
            self.addr_start += 1;
            self.buffer.push_str(&format!(
                "
  {}:
    image: frugalos
    hostname: {}
    command: {}
    volumes:
      - {}:/var/lib/frugalos/
    networks:
      frugalos_net:
        ipv4_address: {}
    env_file: frugalos.env
    {}
",
                hostname, hostname, command, data_dir, addr, depends_on
            ));
        }
        self
    }

    fn write_to<W: Write>(&self, w: &mut W) -> Result<()> {
        w.write_all(self.buffer.as_bytes())
            .map(|_| ())
            .map_err(|e| track!(Error::from(e)))
    }
}
