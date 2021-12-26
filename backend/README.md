# Eiffelvis Backend
[![Backend CI](https://github.com/ItJustWorksTM/EiffelVis/actions/workflows/backend_ci.yml/badge.svg)](https://github.com/ItJustWorksTM/EiffelVis/actions/workflows/backend_ci.yml)

Eiffelvis is mainly implemented inside [eiffelvis_core](./crates/eiffelvis_core), it makes up 90% of the actual code along with [eiffelvis_http](./crates/eiffelvis_http). This top level binary simply bundles all these libraries into a usable executable with command line options.

Take a peek inside the `crates` folder if you are curious.

## Basic setup

```c
# Clone the repo
git clone "https://github.com/ItJustWorksTM/EiffelVis eiffelvis"
# Traverse into the backend folder
cd eiffelvis/backend
# Run via cargo
cargo run -- --help
```
*Note*: Prebuild binaries are available for all 3 major platforms [here](https://github.com/ItJustWorksTM/EiffelVis/releases), or check the latest ci jobs.

## Usage

Usage is pretty straight forward, there are no config files or settings to modify on your system, Eiffelvis will start a web server and connect to the RabbitMQ broker with respective given adresses.

```
-a, --address <address>          HTTP host address [default: 127.0.0.1]
    --chunk-size <chunk-size>    Maximum amount of events a single chunk will hold [default: 128]
    --max-chunks <max-chunks>    Maximum amount of chunks stored in memory [default: 8]
-p, --port <port>                HTTP host port [default: 3001]
-q, --rmq-queue <rmq-queue>      AMQP queue [default: hello]
-r, --rmq-uri <rmq-uri>          AMQP URI [default: amqp://localhost:5672/%2f]
-t, --timeout <timeout>          AMQP reconnect timeout [default: 3001]
    --tls-cert <tls-cert>        TLS certificate pem file
    --tls-key <tls-key>          TLS private key pem file

```
*Note: `--chunk-size` and `--max-chunks` determine how many events will be held in memory at a maximum, if at any moment the number is exceeded the oldest chunk-size amount of events will get erased to make place for the new events.*

*Note 2: chunk size and max chunks are required to be a power of 2, will panic otherwise.*