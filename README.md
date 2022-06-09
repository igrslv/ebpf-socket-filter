# eBPF socket filter

"Introduction to eBPF filters" [Slides](Intro_eBPF.pdf)

## Prerequisites

* [VirtualBox](https://www.virtualbox.org/wiki/Downloads)
* [Vagrant](https://www.vagrantup.com/downloads)

## Run program

The program sums the size of ICMP packets to localhost destination.

```bash
vagrant ssh
cd /vagrant
cargo build --release
sudo target/release/socket_filter
# in another terminal
vagrant ssh
ping -4 localhost
````
