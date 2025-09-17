---
title: "Host Discovery Code Algorithms | Nmap Network Scanning"
source_url: https://nmap.org/book/host-discovery-algorithms.html
fetched_at: 2025-09-17T16:39:34.307778+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 3. Host Discovery (“Ping Scanning”)](https://nmap.org/book/host-discovery.html)
* Host Discovery Code Algorithms

[Prev](https://nmap.org/book/host-discovery-strategies.html)

[Next](https://nmap.org/book/port-scanning.html)

Host Discovery Code Algorithms
----------

[]()

One of the greatest benefits of open source software like Nmap
is that curious users are always able to study the source code when
they want answers about its operation. The highest level ping scanning
function is `nexthost` (in `targets.cc`, which calls `massping` to initialize a list of
targets. `Massping` in turn passes the list off to`ultra_scan`[]() (in `scan_engine.cc`). Ultra\_scan is
Nmap's general-purpose scanning function and
does all the hard work of sending, receiving, and interpreting
packets. For more on `ultra_scan` see[the section called “Scan Code and Algorithms”](https://nmap.org/book/port-scanning-algorithms.html).

While source code analysis is the only way to truly get the
complete picture of Nmap operation down to every trivial detail, it is
not always the easiest approach to understanding Nmap. In many cases,
the most effective way to explore Nmap's behavior given a set
of command-line options is to add the `--packet-trace`option, which prints out all of the packets sent and received by
Nmap.

Because the source code and the `--packet-trace`option are excellent resources for learning the nitty-gritty details of
Nmap operation, I'll only discuss how host discovery works at a high
level here. When Nmap is executed, it may be passed networks containing
hundreds of thousands or even millions of hosts. So Nmap breaks them
into blocks that are small enough to deal with at one time (dozens up
to a few thousand hosts). `ultra_scan` then works its
way through the block, sending packets as fast as its congestion
controls allow. Rather than sending all the probes requested by the
user to each host all at once, Nmap sends the first probe to all the
targets, then the second probe, and so on. When a conclusive response to
a probe is received, that host is marked as up or down as appropriate
and no further probes are sent to it. A target host which fails to respond to any probes, even after retransmissions, is marked as down. Nmap waits until every host has
either received a conclusive response or has timed out. Eventually,
Nmap runs out of new hosts in the block and the number of outstanding
probes dwindles to zero as retransmissions complete. The ping scanning
subsystem returns the results so that Nmap can begin port scanning or
any other requested probing of the target machines. When Nmap finishes
completely with a block of hosts, it prints the results and passes the next block to the ping scanner.

Multiple hosts, usually with multiple probes per
host, are handled in parallel. The number of outstanding probes and
timeout periods are modified in real-time based on network latency and
reliability. The `ultra_scan` performance algorithms
are further described in [the section called “Scan Code and Algorithms”](https://nmap.org/book/port-scanning-algorithms.html).

---

[Prev](https://nmap.org/book/host-discovery-strategies.html)Putting It All Together: Host Discovery Strategies

[Up](https://nmap.org/book/host-discovery.html)Chapter 3. Host Discovery (“Ping Scanning”)

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/port-scanning.html)Chapter 4. Port Scanning Overview