---
title: "Host Discovery Techniques | Nmap Network Scanning"
source_url: https://nmap.org/book/host-discovery-techniques.html
fetched_at: 2025-09-17T16:39:16.387108+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 3. Host Discovery (“Ping Scanning”)](https://nmap.org/book/host-discovery.html)
* Host Discovery Techniques

[Prev](https://nmap.org/book/host-discovery-controls.html)

[Next](https://nmap.org/book/host-discovery-strategies.html)

Host Discovery Techniques
----------

[]()

There was a day when finding whether an IP address was
registered to an active host was easy. Simply send an
ICMP echo[]()request (*ping*) packet and wait for a response. Firewalls rarely
blocked these requests, and the vast majority of hosts
obediently responded. Such a response has been required since 1989 by[RFC 1122](http://www.rfc-editor.org/rfc/rfc1122.txt), which clearly states that “Every host MUST implement
an ICMP Echo server function that receives Echo Requests and sends
corresponding Echo Replies”.

[]()

Unfortunately for network explorers, many administrators have
decided that security concerns trump RFC requirements and have blocked
ICMP ping messages.[Example 3.8](https://nmap.org/book/host-discovery-techniques.html#host-discovery-ex-ping2)uses an ICMP-only Nmap ping scan
against six popular Web sites, but receives only two responses. This
demonstrates that hosts can no longer be assumed unavailable based on
failure to reply to ICMP ping probes. The “`-sn
-PE`” options in this example specify an ICMP-only ping
scan. The `-R` option tells Nmap to perform
reverse-DNS resolution against all hosts, even down ones.

Example 3.8. Attempts to ping popular
Internet hosts

[]()[]()[]()

```
# nmap -sn -PE -R -v microsoft.com ebay.com citibank.com google.com \
                     slashdot.org yahoo.com

Starting Nmap ( https://nmap.org )
Host origin2.microsoft.com (207.46.250.252) appears to be down.
Host pages.ebay.com (66.135.192.87) appears to be down.
Host ld1-www.citicorp.com (192.193.195.132) appears to be down.
Host 216.239.57.99 appears to be up.
Host slashdot.org (66.35.250.150) appears to be down.
Host w3.rc.dcn.yahoo.com (216.109.127.30) appears to be up.
Nmap done: 6 IP addresses (2 hosts up) scanned in 3.76 seconds

```

Fortunately, Nmap offers a wide variety of host discovery
techniques beyond the standard ICMP echo request. They are described
in the following sections. Note that if you specify any of
the `-P` options discussed in this section,
they *replace* the default discovery probes rather
than adding to them.

### TCP SYN Ping (`-PS*`<port list>`*`) ###

[]()[]()

The `-PS` option sends an empty TCP packet with
the SYN flag set. The default destination port is 80 (configurable at
compile time by changing`DEFAULT_TCP_PROBE_PORT_SPEC`[]()in `nmap.h`),[]()but an alternate port can
be specified as a parameter. A list of ports may be specified
(e.g. `-PS22-25,80,113,1050,35000`), in which case
probes will be attempted against each port in parallel.

The SYN flag suggests to the remote system that you are
attempting to establish a connection. Normally the destination port
will be closed, and a RST (reset) packet will be sent back. If the port
happens to be open, the target will take the second step of a TCP
three-way-handshake[]()by responding with a SYN/ACK TCP packet. The machine
running Nmap then tears down the nascent connection by responding with
a RST rather than sending an ACK packet which would complete the
three-way-handshake and establish a full connection.[<sup id="idm45818756098624" class="footnote">[10]</sup>](https://nmap.org/book/host-discovery-techniques.html#ftn.idm45818756098624)

Nmap does not care whether the port is open or closed. Either
the RST or SYN/ACK response discussed previously tell Nmap that the
host is available and responsive.

[]()[]()

On Unix boxes, only the privileged user `root`is generally able to send and receive
raw TCP packets.[]()For unprivileged users, a workaround is automatically employed whereby the`connect`system call is initiated against each target port. This has
the effect of sending a SYN packet to the target host, in an attempt
to establish a connection. If `connect` returns with a quick success
or an ECONNREFUSED failure, the underlying TCP stack must have
received a SYN/ACK or RST and the host is marked available. If the
connection attempt is left hanging until a timeout is reached, the
host is marked as down.
This workaround is also used for IPv6[]()connections, as raw IPv6 packet building support is not yet available
in Nmap.

[Example 3.8](https://nmap.org/book/host-discovery-techniques.html#host-discovery-ex-ping2) failed to detect four out of six machines because they did not respond to ICMP echo requests. Repeating the experiment using a SYN probe to port 80 (HTTP) garners responses from all six, as shown in [Example 3.9](https://nmap.org/book/host-discovery-techniques.html#host-discovery-ex-synping).

Example 3.9. Retry host discovery using port 80 SYN probes

```
# nmap -sn -PS80 -R -v microsoft.com ebay.com citibank.com google.com \
                       slashdot.org yahoo.com

Starting Nmap ( https://nmap.org )
Host origin2.microsoft.com (207.46.249.252) appears to be up.
Host pages.ebay.com (66.135.192.87) appears to be up.
Host ld1-www.citicorp.com (192.193.195.132) appears to be up.
Host 216.239.57.99 appears to be up.
Host slashdot.org (66.35.250.150) appears to be up.
Host w3.rc.dcn.yahoo.com (216.109.127.30) appears to be up.
Nmap done: 6 IP addresses (6 hosts up) scanned in 0.48 seconds

```

In addition to detecting all six machines, the second run is
much faster. It takes less than half a second because the machines
are scanned in parallel and the scan never times out waiting for a response.
This test is not entirely fair because these are all popular web
servers and thus can be expected to listen on TCP port 80. However, it
still demonstrates the point that different types of hosts respond to
different probe types. Nmap supports the usage of many scan types in
parallel to enable effective scanning of diverse networks.

### TCP ACK Ping (`-PA*`<port list>`*`) ###

[]()[]()

The TCP ACK ping is quite similar to the SYN ping. The
difference, as you could likely guess, is that the TCP ACK flag is set
instead of the SYN flag. Such an ACK packet purports to be
acknowledging data over an established TCP connection, but no such
connection exists. So remote hosts should always respond with a RST
packet, disclosing their existence in the process.

The `-PA` option uses the same default port as the SYN
probe (80) and can also take a list of destination ports in the same
format.[]()[]()If an unprivileged user tries this, or an IPv6 target is
specified, the `connect` workaround discussed previously is used.
This workaround is imperfect because `connect` is actually sending a
SYN packet rather than an ACK.

[]()

The reason for offering both SYN and ACK ping probes is to
maximize the chances of bypassing firewalls. Many administrators
configure routers and other simple firewalls to block incoming
SYN packets except for those destined for public services like the
company web site or mail server. This prevents other incoming
connections to the organization, while allowing users to make
unobstructed outgoing connections to the Internet. This non-stateful
approach takes up few resources on the firewall/router and is widely
supported by hardware and software filters. As just one example of
the prevalence of this method, the Linux
Netfilter/iptables firewall[]()software offers the `--syn` convenience option, which
the man page describes as follows.

>
>
> Only match TCP packets with the SYN bit set
> and the ACK and RST bits cleared. Such packets are used to request
> TCP connection initiation; for example, blocking such packets coming
> in an interface will prevent incoming TCP connections, but outgoing
> TCP connections will be unaffected. It is equivalent to
> --tcp-flags SYN,RST,ACK SYN.
>
>

When firewall rules such as this are in place, SYN ping probes (`-PS`) are likely to be blocked when sent to closed target ports. In such cases, the ACK probe excels by cutting right through these rules.

[]()

Another common type of firewall uses stateful rules that drop unexpected packets. This feature was initially found mostly on high-end firewalls, though it has become much more common over the years. The Linux Netfilter/iptables system supports this through the `--state` option, which categorizes packets based on connection state as described in the following man page excerpt:

>
>
> Possible states are INVALID meaning that the
> packet is associated with no known connection, ESTABLISHED meaning
> that the packet is associated with a connection which has seen packets
> in both directions, NEW meaning that the packet has started a new
> connection, or otherwise associated with a connection which has not
> seen packets in both directions, and RELATED meaning that the packet
> is starting a new connection, but is associated with an existing
> connection, such as an FTP data transfer, or an ICMP
> error.
>
>

The ACK probe is unlikely to work against firewalls taking this
approach, as such an unexpected packet will be classified in the
INVALID state and probably dropped. [Example 3.10](https://nmap.org/book/host-discovery-techniques.html#host-discovery-ex-msackping) shows an attempted ACK ping
against Microsoft. Their stateful firewall drops the packet, leading
Nmap to wrongly conclude that the host is down. The SYN probe has a
much better chance of working in such cases.
This raises the question of which technique to use when the firewall
rules of the target networks are unknown or inconsistent. The proper
answer is usually both. Nmap can send SYN and ACK probes to many
ports in parallel, as well as performing other host discovery
techniques at the same time. This is further discussed in [the section called “Putting It All Together: Host Discovery Strategies”](https://nmap.org/book/host-discovery-strategies.html).

Example 3.10. Attempted ACK ping against Microsoft

```
# nmap -sn -PA www.microsoft.com

Starting Nmap ( https://nmap.org )
Warning: Hostname www.microsoft.com resolves to 5 IPs. Using 207.46.192.254.
Note: Host seems down. If it is really up, but blocking ping probes, try -Pn
Nmap done: 1 IP address (0 hosts up) scanned in 2.22 seconds

```

### UDP Ping (`-PU*`<port list>`*`) ###

[]()[]()

Another host discovery option is the UDP ping, which sends a UDP
packet to the given ports.
The port list takes the same format as with the
previously discussed `-PS` and `-PA`options. If no ports are specified, the default is 40,125. This default can be
configured at compile-time by changing`DEFAULT_UDP_PROBE_PORT_SPEC`[]()in `nmap.h`.[]()A highly uncommon port is used by default because sending to
open ports is often undesirable for this particular scan type.

For most ports, the packet will be empty,
though for a few common ports like 53 and 161, a protocol-specific
payload will be sent that is more likely to get a
response.[]()The`--data-length`[]()option sends a fixed-length random payload for all ports.

Upon hitting a closed port on the target machine, the UDP probe
should elicit an ICMP port unreachable packet in return. This
signifies to Nmap that the machine is up and available. Many other
types of ICMP errors, such as host/network unreachables or TTL
exceeded are indicative of a down or unreachable host. A lack of response is also
interpreted this way. If an open port is reached, most services
simply ignore the empty packet and fail to return any response. This
is why the default probe port is 40,125, which is highly unlikely to be in
use. A few services, such as the Character Generator (chargen) protocol, will respond to an empty UDP
packet, and thus disclose to Nmap that the machine is
available. Custom payloads, for the ports that have them, make it more
likely that a probe will get a response.

[]()

The primary advantage of this scan type is that it bypasses
firewalls and filters that only screen TCP. For example, I once owned
a Linksys BEFW11S4 wireless broadband router. The external interface
of this device filtered all TCP ports by default, but UDP probes would
still elicit port unreachable messages and thus give away the
device.

### ICMP Ping Types (`-PE`, `-PP`, and `-PM`) ###

[]()[]()[]()[]()[]()

In addition to the unusual TCP and UDP host discovery types
discussed previously, Nmap can send the standard packets sent by the
ubiquitous ping program. Nmap sends an
ICMP type 8 (echo request) packet to the target IP addresses, expecting a type
0 (echo reply) in return from available hosts. As noted at the
beginning of this chapter, many hosts and firewalls now block these
packets, rather than responding as required by [RFC 1122](http://www.rfc-editor.org/rfc/rfc1122.txt). For this
reason, ICMP-only scans are rarely reliable enough against unknown
targets over the Internet. But for system administrators monitoring
an internal network, this can be a practical and efficient approach.
Use the `-PE` option to enable this echo request
behavior.

While echo request is the standard ICMP ping query, Nmap does
not stop there. The ICMP standards
([RFC 792](http://www.rfc-editor.org/rfc/rfc792.txt)[]()and[RFC 950](http://www.rfc-editor.org/rfc/rfc950.txt)[]())
also
specify timestamp request, information request, and address mask
request packets as codes 13, 15, and 17, respectively. While the
ostensible purpose for these queries is to learn information such as
address masks and current times, they can easily be used for host
discovery. Nmap does
not currently implement information request packets, as they are not
widely supported (RFC 1122 insists that “a host SHOULD NOT
implement these messages”). Timestamp and address mask queries
can be sent with the `-PP` and `-PM`options, respectively. A timestamp reply (ICMP code 14) or address
mask reply (code 18) discloses that the host is available. These two
queries can be valuable when administrators specifically block echo request
packets, but forget that other ICMP queries can be used for the same
purpose.

### IP Protocol Ping (`-PO*`<protocol list>`*`) ###

[]()[]()[]()

The newest host discovery option is the IP protocol ping, which
sends IP packets with the specified protocol number set in their IP
header. The protocol list takes the same format as do port lists in
the previously discussed TCP and UDP host discovery options. If no
protocols are specified, the default is to send multiple IP packets
for ICMP (protocol 1), IGMP (protocol 2), and IP-in-IP (protocol 4).
The default protocols can be configured at compile-time by changing`DEFAULT_PROTO_PROBE_PORT_SPEC`[]()in `nmap.h`.[]()Note
that for the ICMP, IGMP, TCP (protocol 6), and UDP (protocol
17),[]()the packets are sent with the proper protocol headers while other
protocols are sent with no additional data beyond the IP header
(unless the`--data-length`[]()option is specified).

This host discovery method looks for either responses using the
same protocol as a probe, or ICMP protocol unreachable messages which
signify that the given protocol isn't supported by the destination
host. Either type of response signifies that the target host is
alive.

### ARP Scan (`-PR`) ###

[]()[]()

One of the most common Nmap usage scenarios is to scan an ethernet LAN. On most LANs, especially
those using private address ranges granted by [RFC 1918](http://www.rfc-editor.org/rfc/rfc1918.txt), the vast majority
of IP addresses are unused at any given time. When Nmap tries to send
a raw IP packet such as an ICMP echo request, the operating system
must determine the destination hardware (ARP) address corresponding to
the target IP so that it can address the ethernet frame properly.
This requires it to issue a series of ARP requests. This is shown in[Example 3.11](https://nmap.org/book/host-discovery-techniques.html#host-discovery-ex-eth-ip-scan), where a ping scan
is attempted against a local ethernet host. The`--send-ip`[]()option tells Nmap to send IP level packets (rather
than raw ethernet) even though it is a local network. Wireshark output
of the three ARP requests and their timing has been pasted into the
session.

Example 3.11. Raw IP ping scan of an offline target

[]()

```
# nmap -n -sn --send-ip 192.168.33.37

Starting Nmap ( https://nmap.org )
  0.000000 00:01:29:f5:27:f2 -> ff:ff:ff:ff:ff:ff ARP Who has 192.168.33.37?
  0.999836 00:01:29:f5:27:f2 -> ff:ff:ff:ff:ff:ff ARP Who has 192.168.33.37?
  1.999684 00:01:29:f5:27:f2 -> ff:ff:ff:ff:ff:ff ARP Who has 192.168.33.37?
Note: Host seems down. If it is really up, but blocking ping probes, try -Pn
Nmap done: 1 IP address (0 hosts up) scanned in 2.04 seconds

```

This example took more than two seconds to finish because the
(Linux) OS sent three ARP requests, one second apart, before giving up
on the host. Given that ARP replies usually come within a couple
milliseconds, multi-second waits are excessive. Decreasing this
timeout period is no priority for OS vendors because the vast majority
of packets are sent to hosts that actually exist. Nmap, on the other
hand, must send packets to 16 million IPs when given a target such as
10.0.0.0/8. A two second wait for each becomes a huge delay even
though many targets are pinged in parallel.

There is another problem with raw IP ping scans on LANs. When a
destination host is found to be unresponsive as in the previous
example, the source host generally adds an incomplete entry for that
destination IP in its kernel ARP table. ARP table space is finite,
and some operating systems react badly when it fills up. When Nmap is
used in raw IP mode (`--send-ip`), Nmap sometimes has
to wait several minutes for ARP cache entries to expire before it can
continue with host discovery.

ARP scanning resolves both problems by putting Nmap in control.
Nmap issues the raw ARP requests and handles retransmission and
timeout periods at its own discretion. The system ARP cache is
bypassed. [Example 3.12](https://nmap.org/book/host-discovery-techniques.html#host-discovery-ex-eth-arp-scan) shows the
difference. This ARP scan takes just over a tenth of the time taken
by its IP equivalent.

Example 3.12. ARP ping scan of an offline target

[]()[]()[]()

```
# nmap -n -sn -PR --packet-trace --send-eth 192.168.33.37

Starting Nmap ( https://nmap.org )
SENT (0.0060s) ARP who-has 192.168.33.37 tell 192.168.0.100
SENT (0.1180s) ARP who-has 192.168.33.37 tell 192.168.0.100
Note: Host seems down. If it is really up, but blocking ping probes, try -Pn
Nmap done: 1 IP address (0 hosts up) scanned in 0.23 seconds

```

[]()

In [Example 3.12](https://nmap.org/book/host-discovery-techniques.html#host-discovery-ex-eth-arp-scan), neither
the `-PR` or `--send-eth` options have
any effect. This is because ARP is the default scan type when
scanning ethernet hosts that Nmap detects are on a local ethernet
network. This includes traditional wired ethernet as well as 802.11 wireless networks. Not only is ARP scanning more efficient as discussed above, it is also
more accurate. Hosts frequently block IP-based ping packets, but they
generally cannot block ARP requests or responses and still communicate
on the network. Even if different ping types (such as`-PE` or `-PS`) are specified, Nmap uses
ARP instead for any of the targets which are on the same LAN.
If you absolutely don't want to do an ARP scan, specify`--send-ip`[]()as shown in [Example 3.11, “Raw IP ping scan of an offline target”](https://nmap.org/book/host-discovery-techniques.html#host-discovery-ex-eth-ip-scan).

Giving Nmap control to send raw ethernet frames also allows Nmap
to control the source MAC address. If you have the only PowerBook in
the room at a security conference and a massive ARP scan is initiated
from a MAC address registered to Apple, heads may turn in your
direction. You can spoof your MAC address with the`--spoof-mac` option, as discussed in [the section called “MAC Address Spoofing”](https://nmap.org/book/firewall-subversion.html#defeating-firewalls-mac-spoofing).

### Default Combination ###

[]()

If none of these host discovery techniques are chosen, Nmap uses
a default which is equivalent to the `-PE -PS443 -PA80 -PP`arguments for Windows or privileged (root) Unix users. Attentive
readers know that this means an ICMP echo request, a TCP SYN packet, a
TCP ACK packet, and an ICMP timestamp request are sent to each machine. An exception to this is
that an ARP scan is used for any targets which are on a local ethernet
network. For unprivileged Unix shell users, the default is equivalent
to `-PS80,443` (a TCP `connect` call against ports 80 and 443 of the
target hosts). For security auditing, I recommend using a more
comprehensive set of ping types, such as those discussed in [the section called “Designing the ideal combinations of probes”](https://nmap.org/book/host-discovery-strategies.html#host-discovery-ideal-probes).

---

[<sup class="para">[10] </sup>](https://nmap.org/book/host-discovery-techniques.html#idm45818756098624)The
RST packet is sent by the kernel of the machine running Nmap in
response to the unexpected SYN/ACK, not by Nmap
itself.

---

[Prev](https://nmap.org/book/host-discovery-controls.html)Host Discovery Controls

[Up](https://nmap.org/book/host-discovery.html)Chapter 3. Host Discovery (“Ping Scanning”)

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/host-discovery-strategies.html)Putting It All Together: Host Discovery Strategies