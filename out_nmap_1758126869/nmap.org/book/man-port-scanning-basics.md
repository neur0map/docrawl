---
title: "Port Scanning Basics | Nmap Network Scanning"
source_url: https://nmap.org/book/man-port-scanning-basics.html
fetched_at: 2025-09-17T16:35:57.807932+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 15. Nmap Reference Guide](https://nmap.org/book/man.html)
* Port Scanning Basics

[Prev](https://nmap.org/book/man-host-discovery.html)

[Next](https://nmap.org/book/man-port-scanning-techniques.html)

Port Scanning Basics
----------

While Nmap has grown in functionality over the years, it began as an efficient port scanner, and that remains its core function. The simple command **nmap *`<target>`*** scans 1,000 TCP ports on the host *`<target>`*. While many port scanners have traditionally lumped all ports into the open or closed states, Nmap is much more granular. It divides ports into six states: `open`, `closed`, `filtered`, `unfiltered`, `open|filtered`, or `closed|filtered`.

These states are not intrinsic
properties of the port itself, but describe how Nmap sees them. For
example, an Nmap scan from the same network as the target may show
port `135/tcp` as open, while a scan at the same time with the same
options from across the Internet might show that port as `filtered`.

The six port states recognized by Nmap

open []()

[An application is actively accepting TCP connections, UDP datagrams or SCTP associations on this port. Finding these is often the primary goal of port scanning. Security-minded people know that each open port is an avenue for attack. Attackers and pen-testers want to exploit the open ports, while administrators try to close or protect them with firewalls without thwarting legitimate users. Open ports are also interesting for non-security scans because they show services available for use on the network. ]()

[closed ]()[ ]()

[A closed port is accessible (it receives and responds to Nmap probe packets), but there is no application listening on it. They can be helpful in showing that a host is up on an IP address (host discovery, or ping scanning), and as part of OS detection. Because closed ports are reachable, it may be worth scanning later in case some open up. Administrators may want to consider blocking such ports with a firewall. Then they would appear in the filtered state, discussed next. ]()

[filtered ]()[ ]()

[Nmap cannot determine whether the port is open because packet filtering prevents its probes from reaching the port. The filtering could be from a dedicated firewall device, router rules, or host-based firewall software. These ports frustrate attackers because they provide so little information. Sometimes they respond with ICMP error messages such as type 3 code 13 (destination unreachable: communication administratively prohibited), but filters that simply drop probes without responding are far more common. This forces Nmap to retry several times just in case the probe was dropped due to network congestion rather than filtering. This slows down the scan dramatically.]()

[unfiltered ]()[ ]()

[The unfiltered state means that a port is accessible, but Nmap is unable to determine whether it is open or closed. Only the ACK scan, which is used to map firewall rulesets, classifies ports into this state. Scanning unfiltered ports with other scan types such as Window scan, SYN scan, or FIN scan, may help resolve whether the port is open. ]()

[open|filtered ]()[ ]()

[Nmap places ports in this state when it is unable to determine whether a port is open or filtered. This occurs for scan types in which open ports give no response. The lack of response could also mean that a packet filter dropped the probe or any response it elicited. So Nmap does not know for sure whether the port is open or being filtered. The UDP, IP protocol, FIN, NULL, and Xmas scans classify ports this way.]()

[closed|filtered ]()[ ]()

[This state is used when Nmap is unable to determine whether a port is closed or filtered. It is only used for the IP ID idle scan.]()

---

[Prev](https://nmap.org/book/man-host-discovery.html)Host Discovery

[Up](https://nmap.org/book/man.html)Chapter 15. Nmap Reference Guide

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/man-port-scanning-techniques.html)Port Scanning Techniques