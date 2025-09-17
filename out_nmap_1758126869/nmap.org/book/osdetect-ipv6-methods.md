---
title: "IPv6 fingerprinting | Nmap Network Scanning"
source_url: https://nmap.org/book/osdetect-ipv6-methods.html
fetched_at: 2025-09-17T16:42:45.642665+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 8. Remote OS Detection](https://nmap.org/book/osdetect.html)
* IPv6 fingerprinting

[Prev](https://nmap.org/book/osdetect-methods.html)

[Next](https://nmap.org/book/osdetect-other-methods.html)

IPv6 fingerprinting
----------

[]()

Nmap has a similar but separate OS detection engine specialized for
IPv6. At a high level, the technique is the same: send probes, collect
responses, and match the set of responses against a database. The
differences are in the specific probes used, and in the way they are
matched.

IPv6 OS detection is used just like IPv4. Just use the`-6` and `-O` options together. For
example,**nmap -6 -O *`<target>`***.

### Probes Sent ###

[]()

IPv6 OS detection uses many of the same probes that IPv4 OS detection
does. Most of the power to distinguish operating systems comes from
higher-layer protocols like TCP, though there are a few new
IPv6-specific detection features.

In all cases, the IPv6 flow label is 0x12345, on platforms that allow us
to set it. On platforms that do not (which includes non-Linux Unix
platforms when not using Ethernet to send), the flow label will be 0.
Because this can affect the responses, the value of the flow label is
recorded in the `EXTRA` field of OS fingerprints.
Except for the `NS` probe, hop limits are set randomly.

In all, up to 18 probes may be sent. They are sent in the following order.

#### Sequence generation (`S1`–`S6`) ####

These are the same six probes as the `T1` collection of
probes sent in IPv4 detection. See [the section called “Sequence generation (`SEQ`, `OPS`, `WIN`, and `T1`)”](https://nmap.org/book/osdetect-methods.html#osdetect-probes-seq)for documentation of the packet contents. These six probes are sent
100 ms apart for timing measurements.

The `S1`–`S6` probes are skipped
if the target lacks an open port.

#### ICMPv6 echo (`IE1`) ####

This is more or less an ordinary ICMPv6 echo request. The type is 128
(Echo Request) and the code is 9, though it should be 0. The ICMPv6 ID
is 0xabcd and the sequence number is 0. The data payload is 120 zero
bytes. There is one Hop-By-Hop extension header containing only padding.

#### ICMPv6 echo (`IE2`) ####

This is an echo request with a type of 128 (Echo Request) and a code of
0. The ICMPv6 ID is 0xabcd and the sequence is 1. There is no data
payload.

What makes this probe interesting are the erroneous extension headers it
includes. There are four of them in all, in this order:

|    Hop-By-Hop     |
|-------------------|
|Destination Options|
|      Routing      |
|    Hop-By-Hop     |

These headers are erroneous: no header other than Destination Options is
supposed to appear more than once, and Hop-by-hop options are only
supposed to appear in the first position. In our tests, no operating
systems treat this as a legitimate echo request. They do, however,
respond with different ICMPv6 errors.

#### Node Information Query (`NI`) ####

RFC 4620 defines ICMPv6 messages called Node Information Queries that
allow asking a target for its hostnames, IPv4 addresses, and IPv6
addresses. The `NI` probe has type 139 (ICMP Node
Information Query) and code 0 (indicating that the subject is an IPv6
address). The qtype is 4 (IPv4 Addresses). The A flag (return all
unicast addresses) flag is set, and no others. The nonce is set to the
fixed string "\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x0a".

Despite being asked for IPv4 addresses, some operating systems return a
DNS name instead.

#### Neighbor Solicitation (`NS`) ####

The `NS` probe sends a Neighbor Solicitation query, as
if asking for the target's hardware address. The type is 135 and the
code is 0. The hop limit is always set to 255, no matter the setting of`--ttl`; RFC 2461 forbids hosts to reply otherwise. All
flags are set to 0.

This probe is only sent to hosts on the same subnet.

#### UDP (`U1`) ####

A UDP packet is sent to a a closed port, if available. The data payload
is set to 300 'C' (0x43) bytes. This probe is designed to elicit an
ICMPv6 Port Unreachable message.

#### TCP explicit congestion notification (`TECN`) ####

This is the same as the `ECN` probe from IPv4. It is a
SYN packet to an open port, that also has the ECE and CWR flags set. The
urgent field value of 0xF7F5 is used even though the urgent flag is not
set. The acknowledgment number is zero, sequence number is random, and
the window size field is three. TCP options are WScale (10), NOP, MSS
(1460), SACK permitted, NOP, NOP.

#### TCP (`T2`–`T7`) ####

These correspond to the`T2`–`T7` probes from IPv4
detection, described in [the section called “TCP (`T2`–`T7`)”](https://nmap.org/book/osdetect-methods.html#osdetect-probes-t). The
numbering starts at 2 rather than 1 because the six sequencing probes
are collectively known as “`T1`” in IPv4
(they were renamed to `S1`–`S6`for IPv6).

### Feature extraction ###

After responses are received, various pieces of data are extracted from
them. In machine learning literature these pieces of data are known as“features”. Examples of features are: IPv6 hop limit,
ICMPv6 type and code, and code of first TCP option. (In Nmap's
terminology, these are known as `IPV6_HOPLIMIT`,`ICMPV6_TYPE`, and `TCP_OPT_0`respectively.) Some features are simply extracted directly from response
packets, and some are the result of doing a calculation over several
packets (like `TCP_ISR`, the TCP initial sequence
number counter rate).

Any features whose value cannot be determined (for example, features
from a response that was never received) are set to −1. The
features are put in a big one-dimensional feature vector. Then each is
scaled and translated to put it approximately into the range [0, 1],
using scale parameters estimated from our training data.

#### List of all features ####

`TCP_ISR`

TCP ISN counter rate. This is derived from the`S1`–`S6` sequence probes, which
are sent 100 ms apart. The differences between consecutive sequence
responses are added up, then this sum is divided by the time elapsed
between the first and last probe.

The following features are repeated for each response, so for example a
fully qualified feature name might be `S1.PLEN`.

`PLEN`

IPv6 Payload Length field

`TC`

IPv6 Traffic Class field

`HLIM`

A guess at the original value of the IPv6 Hop Limit field

The following features are repeated for each TCP response. A full
feature name might be `T2.TCP_WINDOW`.

`TCP_WINDOW`

TCP window size

`TCP_FLAG_F`, `TCP_FLAG_S`, `TCP_FLAG_R`, `TCP_FLAG_P`, `TCP_FLAG_A`, `TCP_FLAG_U`, `TCP_FLAG_E`, `TCP_FLAG_C`

TCP flags. Each flag becomes a feature with the value 0
or 1.

`TCP_FLAG_RES8`, `TCP_FLAG_RES9`, `TCP_FLAG_RES10`, `TCP_FLAG_RES11`

These are the four bits of the reserved part of the TCP
header. RFC 3540 defines `TCP_FLAG_RES8` as the nonce
sum (NS) bit.

`TCP_OPT_0`, *`<...>`*, `TCP_OPT_16`

Type codes for the first 16 TCP options.

`TCP_OPTLEN_0`, *`<...>`*, `TCP_OPTLEN_16`

Lengths of the first 16 TCP options.

`TCP_MSS`

Value of the first MSS option, if present.

`TCP_SACKOK`

1 if the SACK-permitted option is present, 0
otherwise.

`TCP_WSCALE`

Value of the first Window Scale option, if
present.

### Differences from IPv4 ###

IPv6 fingerprints look somewhat different from IPv4 fingerprints.
Instead of a broken-down list of packet features, they consist of a hex
dump of packet contents along with send and receive times. See[the section called “Understanding an Nmap Fingerprint”](https://nmap.org/book/osdetect-fingerprint-format.html) for details.

The IPv6 matching algorithm is quite different. It uses a machine
learning algorithm called logistic regression rather than simple
comparison against a list of fingerprints.[the section called “IPv6 matching”](https://nmap.org/book/osdetect-guess.html#osdetect-guess-ipv6) has a description of the
algorithm.

---

[Prev](https://nmap.org/book/osdetect-methods.html)TCP/IP Fingerprinting Methods Supported by Nmap

[Up](https://nmap.org/book/osdetect.html)Chapter 8. Remote OS Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/osdetect-other-methods.html)Fingerprinting Methods Avoided by Nmap