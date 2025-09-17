---
title: "IP Protocol Number List: nmap-protocols | Nmap Network Scanning"
source_url: https://nmap.org/book/nmap-protocols.html
fetched_at: 2025-09-17T16:48:38.090388+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 14. Understanding and Customizing Nmap Data Files](https://nmap.org/book/data-files.html)
* IP Protocol Number List: nmap-protocols

[Prev](https://nmap.org/book/nmap-mac-prefixes.html)

[Next](https://nmap.org/book/data-files-nse.html)

IP Protocol Number List: `nmap-protocols`
----------

[]()

This file maps the one-byte IP protocol number in the IP header into the corresponding protocol name. [Example 14.6](https://nmap.org/book/nmap-protocols.html#data-files-nmap-protocols-file) is a typical
excerpt.

Example 14.6. Excerpt from `nmap-protocols`

[]()

```
hopopt           0     HOPOPT      # IPv6 Hop-by-Hop Option
icmp             1     ICMP        # Internet Control Message
igmp             2     IGMP        # Internet Group Management
ggp              3     GGP         # Gateway-to-Gateway
ip               4     IP          # IP in IP (encapsulation)
st               5     ST          # Stream
tcp              6     TCP         # Transmission Control
cbt              7     CBT         # CBT
egp              8     EGP         # Exterior Gateway Protocol
[ ... ]
chaos           16     CHAOS       # Chaos
udp             17     UDP         # User Datagram

```

The first two fields are the protocol name or abbreviation and
the number in decimal format. Nmap doesn't care about anything after
the protocol number. It is used for IP protocol scanning, as
described at [the section called “IP Protocol Scan (`-sO`)”](https://nmap.org/book/scan-methods-ip-protocol-scan.html). Fewer
than 140 protocols are defined and users almost never modify this
file. The raw data is made available by the
IANA[]()at [`http://www.iana.org/assignments/protocol-numbers`](http://www.iana.org/assignments/protocol-numbers).

---

[Prev](https://nmap.org/book/nmap-mac-prefixes.html)MAC Address Vendor Prefixes: nmap-mac-prefixes

[Up](https://nmap.org/book/data-files.html)Chapter 14. Understanding and Customizing Nmap Data Files

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/data-files-nse.html)Files Related to Scripting