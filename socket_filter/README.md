## eBPF socket filter

The program sums the size of ICMP packets to localhost destination.

```bash
cargo build --release
sudo target/release/socket_filter
ping -4 localhost # in a separate terminal
````
