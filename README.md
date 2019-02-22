A command line utilities for FrugalOS.

Build
------------

```console
$ cargo build
```

How to use
------------

Run:

```console
$ frugalostool -h
```

### Creates a `docker-compose.yml`

`create-docker-compose` command generates a `docker-compose.yml` and outputs a result to stdout.
Keep in mind that a generated `docker-compose.yml` is used for running a cluster NOT in a production environment but a development environment.

An example:

```console
$ frugalostool create-docker-compose --cluster-size 10 > /tmp/docker-compose.yml
```

Currently, `--cluster-addr-start` and `--node-index-start` don't work properly due to the limitation of `bootstrap.sh` and `join.sh` in `frugalos/frugalos` repository.
