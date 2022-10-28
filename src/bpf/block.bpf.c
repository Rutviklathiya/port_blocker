#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_endian.h>

//#include <linux/if_ether.h>
#define ETH_P_IP    0x0800
#define ETH_P_IPV6  0x86DD

#define MAXIMUM_PORTS 10

const size_t MAX_PORTS = MAXIMUM_PORTS; 

struct {
	__uint(type, BPF_MAP_TYPE_ARRAY);
	__uint(max_entries, MAX_PORTS);
	__type(key, u32);
	__type(value, u16);
} allowed_ports SEC(".maps");

bool is_port_allowed(u16 hport)
{
	u32 i = 0;
	for (i = 0; i < MAXIMUM_PORTS; i++) {
		u32 key = i;
		u16 *allow_port = bpf_map_lookup_elem(&allowed_ports, &key);
		if (allow_port && hport == *allow_port) {
			return true;
		}
	}

	return false;
}

struct src_dst_t {
    u16 src; //source port in host order
    u16 dst; //destination port in host order
};

/**
 * get_ports
 *
 * Description: fills out the src_dst_t structure "ports" with
 *              the source and destination of an incoming packet
 *
 * @param ctx - struct xdp_md* - meta-data for the incoming packet
 *              see: https://elixir.bootlin.com/linux/latest/source/include/uapi/linux/bpf.h#L5861
 * @param[OUT] ports - ptr to a src_dst_t struct to be filled out
 *
 * @return - 0 if a TCP or UDP packet is found, along with filling
 *           the ports values
 *           positive value if non-TCP or non-UDP packet is found
 *           (ports are ignored)
 *           negative value upon ERROR
 */
int get_ports(struct xdp_md *ctx, struct src_dst_t *ports) {
	// See:
//https://github.com/libbpf/libbpf-rs/blob/master/examples/tc_port_whitelist/src/bpf/tc.bpf.c
	// for hints
    
	return -1;
}

SEC("xdp")
int allow_xdp(struct xdp_md *ctx)
{
    struct src_dst_t ports = {};

    if (0 != get_ports(ctx, &ports)) {
        return XDP_DROP;
    }

	if (is_port_allowed(ports.dst) || is_port_allowed(ports.src)) {
		return XDP_PASS;
	} else {
		return XDP_DROP;
	}
}

char __license[] SEC("license") = "GPL";

