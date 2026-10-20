# rust-lb-proxy

A load-balancing reverse proxy written in Rust. It sits in front of a pool of backend servers and distributes incoming requests across them.

> **Status:** work in progress

## Features

- Reverse proxy that forwards client requests to backend servers
- **Least-connections load balancing**: each request goes to the backend with the fewest active connections. Trying to alternate between backends with equal connections.
- **Passive health checks**: backends are marked EJECTED based on how real requests to them behave.
- **Asynchronous**: built on [Tokio](https://tokio.rs), so it handles many concurrent connections efficiently

## How it works

```
             ┌──────────────┐        ┌───────────┐
  Client ──► │  rust-lb-    │ ─────► │ Backend 1 │
             │  proxy       │ ─────► │ Backend 2 │
             │ (load        │ ─────► │ Backend N │
             │  balancer)   │        └───────────┘
             └──────────────┘
```

1. A client sends a request to the proxy.
2. The proxy picks the healthy backend with the fewest active connections.
3. The request is forwarded and the backend's response is returned to the client.
4. If a backend fails, the proxy notices through the failed request (passive health checking) and stops routing to it.

## Project structure

| Directory   | Purpose                                                        |
| ----------- | -------------------------------------------------------------- |
| `proxy/`    | The load balancer / reverse proxy itself                       |
| `backends/` | Sample backend servers used to test the proxy                  |
| `setup/`    | Setup scripts / configuration for running the project locally  |

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain) and Cargo

## Getting started

Clone the repository:

```bash
git clone https://github.com/Ziousss/rust-lb-proxy.git
cd rust-lb-proxy
```

### 1. Build the proxy

```bash
cd proxy/src
cargo build
```

This should compile without any warnings.

### 2. Setup the fake network

```bash
cd ../../
sudo bash setup/setup.sh
sudo bash setup/backend.sh
```

This will create a fake network with one client (10.0.0.10:8080), one proxy (10.0.0.20:8080) and three backends (10.0.0.30:8080, 10.0.0.31:8080, 10.0.0.32:8080).

### 3. Send some requests

To simplify real interactions, we will assume the client only has access to the proxy's IP and that it does not matter which backend replies.

```bash
sudo ip netns exec client curl http://10.0.0.20:8080/
```

Repeat the request a few times and watch it get spread across the backends.

## Configuration

All of these can be found in `config/config.txt`. Feel free to add or delete some to experiment.

```
backends = 10.0.0.30:8080, 10.0.0.31:8080, 10.0.0.32:8080
bind = 10.0.0.20:8080
connection_timeout = 5
```