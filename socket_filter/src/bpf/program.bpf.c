#include <stddef.h>
#include <arpa/inet.h>

#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <linux/if_packet.h>
#include <linux/ip.h>

#include "bpf/bpf_helpers.h"

#define IP_PROTO_OFF offsetof(struct iphdr, protocol)
#define IP_DEST_OFF offsetof(struct iphdr, daddr)

struct bpf_map_def SEC("maps") map = {
    .type = BPF_MAP_TYPE_ARRAY,
    .key_size = sizeof(__u32),
    .value_size = sizeof(__u64),
    .max_entries = 32
};

/*
 * Track size of outgoing ICMP and UDP packets
 */
SEC("socket")
int bpf_program(struct __sk_buff *skb) {
    // Only outgoing packets
    if (skb->pkt_type != PACKET_OUTGOING) return 0;

    __u32 proto = 0, dest = 0;

    // Only ICMP and UDP packets
    bpf_skb_load_bytes(skb, ETH_HLEN + IP_PROTO_OFF, &proto, 1);
    if (proto != IPPROTO_ICMP && proto != IPPROTO_UDP) return 0;

    // Only localhost destination
    bpf_skb_load_bytes(skb, ETH_HLEN + IP_DEST_OFF, &dest, sizeof(dest));
    if (ntohl(dest) != 0x7f000001) return 0;

    long *value = bpf_map_lookup_elem(&map, &proto);
    if (value) {
        __sync_fetch_and_add(value, skb->len);
    }

    return 0;
}

char _license[] SEC("license") = "GPL";
