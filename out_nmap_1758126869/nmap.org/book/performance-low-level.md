---
title: "Low-Level Timing Controls | Nmap Network Scanning"
source_url: https://nmap.org/book/performance-low-level.html
fetched_at: 2025-09-17T16:41:35.059099+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 6. Optimizing Nmap Performance](https://nmap.org/book/performance.html)
* Low-Level Timing Controls

[Prev](https://nmap.org/book/performance-port-selection.html)

[Next](https://nmap.org/book/performance-timing-templates.html)

Low-Level Timing Controls
----------

[]()

Nmap offers many fine-grained options for controlling scan
speed. Most people use these options to speed Nmap up, but they can
also be useful for slowing Nmap down. People do that to evade IDS
systems, reduce network load, or even improve accuracy if network
conditions are so bad that even Nmap's conservative default is too
aggressive.

[Table 6.2](https://nmap.org/book/performance-low-level.html#performance-tbl-lowlevel-timing)lists each low-level timing control
option by function. For detailed usage information on every option,
read [the section called “Timing and Performance”](https://nmap.org/book/man-performance.html). It is assumed that the reader
is already familiar with the Nmap scanning algorithms described in[the section called “Scan Code and Algorithms”](https://nmap.org/book/port-scanning-algorithms.html).

Table 6.2. Low-level timing controls by function

[]()

|                              Function                              |                             Options                             |
|--------------------------------------------------------------------|-----------------------------------------------------------------|
|        Hostgroup (batch of hosts scanned concurrently) size        |              `--min-hostgroup`, `--max-hostgroup`               |
|               Number of probes launched in parallel                |            `--min-parallelism`, `--max-parallelism`             |
|                        Probe timeout values                        |`--min-rtt-timeout`, `--max-rtt-timeout`, `--initial-rtt-timeout`|
|          Maximum number of probe retransmissions allowed           |                         `--max-retries`                         |
|           Maximum time before giving up on a whole host            |                        `--host-timeout`                         |
|Control delay inserted between each probe against an individual host|               `--scan-delay`, `--max-scan-delay`                |
|               Rate of probe packets sent per second                |                   `--min-rate`, `--max-rate`                    |
|          Defeat RST packet response rate by target hosts           |                    `--defeat-rst-ratelimit`                     |

---

[Prev](https://nmap.org/book/performance-port-selection.html)Port Selection Data and Strategies

[Up](https://nmap.org/book/performance.html)Chapter 6. Optimizing Nmap Performance

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/performance-timing-templates.html)Timing Templates (-T)