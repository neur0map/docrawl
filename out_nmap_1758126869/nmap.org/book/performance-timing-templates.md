---
title: "Timing Templates (-T) | Nmap Network Scanning"
source_url: https://nmap.org/book/performance-timing-templates.html
fetched_at: 2025-09-17T16:41:21.887236+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 6. Optimizing Nmap Performance](https://nmap.org/book/performance.html)
* Timing Templates (-T)

[Prev](https://nmap.org/book/performance-low-level.html)

[Next](https://nmap.org/book/mayo-scan.html)

Timing Templates (`-T`)
----------

[]()[]()

While the fine-grained timing controls discussed in the previous
section are powerful and effective, some people find them confusing.
Moreover, choosing the appropriate values can sometimes take more time
than the scan you are trying to optimize. So Nmap offers a simpler
approach, with six timing templates. You can specify them with the`-T` option and their number (0–5) or their name.
The template names are`paranoid` (`0`),[]()`sneaky` (`1`),[]()`polite` (`2`),[]()`normal` (`3`),[]()`aggressive` (`4`), and[]()`insane` (`5`).[]()The first two are for
IDS evasion.[]()Polite mode slows down the scan to use less bandwidth and
target machine resources. Normal mode is the default and so`-T3` does nothing. Aggressive mode speeds scans up by
making the assumption that you are on a reasonably fast and reliable
network. Finally insane mode assumes that you are on an
extraordinarily fast network or are willing to sacrifice some accuracy
for speed.

These templates allow the user to specify how aggressive they
wish to be, while leaving Nmap to pick the exact timing values. The
templates also make some minor speed adjustments for which fine-grained control options do not currently exist. For example,`-T4` prohibits the dynamic scan delay from exceeding
10 ms for TCP ports and `-T5` caps that value at 5 ms.
Templates can be used in combination with fine-grained
controls, and the granular options will override the general timing
templates for those specific values. I recommend
using `-T4` when scanning reasonably modern and
reliable networks. Keep that option (at the beginning of the command
line) even when you add fine-grained controls so that you benefit from
those extra minor optimizations that it enables.

[Table 6.3](https://nmap.org/book/performance-timing-templates.html#tbl-performance-timing-template-values) shows how the timing variables vary for each `-T` value. All time values are in milliseconds.

Table 6.3. Timing templates and their effects

[]()

|                                                 |                   T0                    |    T1    |    T2    |    T3    |    T4    |    T5    |
|-------------------------------------------------|-----------------------------------------|----------|----------|----------|----------|----------|
|                      Name                       |                Paranoid                 |  Sneaky  |  Polite  |  Normal  |Aggressive|  Insane  |
|                `min-rtt-timeout`                |                 100 ms                  |  100 ms  |  100 ms  |  100 ms  |  100 ms  |  50 ms   |
|                `max-rtt-timeout`                |                5 minutes                |15 seconds|10 seconds|10 seconds| 1250 ms  |  300 ms  |
|              `initial-rtt-timeout`              |                5 minutes                |15 seconds| 1 second | 1 second |  500 ms  |  250 ms  |
|                  `max-retries`                  |                   10                    |    10    |    10    |    10    |    6     |    2     |
|Initial (and minimum) scan delay (`--scan-delay`)|                5 minutes                |15 seconds|  400 ms  |    0     |    0     |    0     |
|             Maximum TCP scan delay              |                5 minutes                |  15,000  | 1 second | 1 second |  10 ms   |   5 ms   |
|             Maximum UDP scan delay              |                5 minutes                |15 seconds| 1 second | 1 second | 1 second | 1 second |
|                 `host-timeout`                  |                    0                    |    0     |    0     |    0     |    0     |15 minutes|
|                `script-timeout`                 |                    0                    |    0     |    0     |    0     |    0     |10 minutes|
|                `min-parallelism`                |Dynamic, not affected by timing templates|          |          |          |          |          |
|                `max-parallelism`                |                    1                    |    1     |    1     | Dynamic  | Dynamic  | Dynamic  |
|                 `min-hostgroup`                 |Dynamic, not affected by timing templates|          |          |          |          |          |
|                 `max-hostgroup`                 |Dynamic, not affected by timing templates|          |          |          |          |          |
|                   `min-rate`                    |          No minimum rate limit          |          |          |          |          |          |
|                   `max-rate`                    |          No maximum rate limit          |          |          |          |          |          |
|             `defeat-rst-ratelimit`              |         Not enabled by default          |          |          |          |          |          |

If you are on a decent broadband or ethernet connection, I would
recommend always using`-T4`.[]()Some people love`-T5`[]()though it is too aggressive for my taste. People sometimes specify`-T2`[]()because they think it is less
likely to crash hosts or because they consider themselves to be polite
in general. They often don't realize just how slow `-T
polite` really is. They scan may take ten times longer than a
default scan.
Machine crashes and bandwidth problems are rare with the
default timing options
(`-T3`)[]()and so I normally
recommend that for cautious scanners. Omitting version detection is
far more effective than playing with timing values for reducing these
problems.

While`-T0`[]()and `-T1`[]()may be
useful for avoiding IDS alerts, they will take an extraordinarily long
time to scan thousands of machines or ports. For such a long scan,
you may prefer to set the exact timing values you need rather than
rely on the canned `-T0` and `-T1`values.

[]()

---

[Prev](https://nmap.org/book/performance-low-level.html)Low-Level Timing Controls

[Up](https://nmap.org/book/performance.html)Chapter 6. Optimizing Nmap Performance

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/mayo-scan.html)Scanning 676,352 IP Addresses in 46 Hours