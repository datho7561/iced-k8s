# iced-k8s

Simple cross-platform GUI Kubernetes cluster explorer implemented in Rust using
[iced](https://github.com/iced-rs/iced)
and
[kube](https://github.com/kube-rs/kube).

I'm using it to learn Rust. It is WIP.

## Running

1. Install Rust using [rustup](https://rustup.rs/)
2. Clone this repo.
3. Run `cargo run` in the repo

## Features

- Show deployments, pods, stateful sets, daemon sets, and replica sets for the currently configured Kubernetes context
- Polls the cluster for changes
- Delete any of the above resources
- View phase of Pods
- View number of instances that are available for deployments and replication controllers
- Change Kubernetes contexts from the ones listed in your ~/.kube/config
