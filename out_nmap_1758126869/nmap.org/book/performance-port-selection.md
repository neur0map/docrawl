---
title: "Port Selection Data and Strategies | Nmap Network Scanning"
source_url: https://nmap.org/book/performance-port-selection.html
fetched_at: 2025-09-17T16:41:37.617268+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 6. Optimizing Nmap Performance](https://nmap.org/book/performance.html)
* Port Selection Data and Strategies

[Prev](https://nmap.org/book/scantime-coping.html)

[Next](https://nmap.org/book/performance-low-level.html)

Port Selection Data and Strategies
----------

Port scanning can be the most time consuming portion of an Nmap
scan, even when the scan includes version detection or NSE scripts.
Port scan time is roughly proportional to the number of ports scanned,
so reducing the number of ports provides a significant performance
boost. The down side is that reduced scans are less comprehensive,
so you might miss open ports.

The reality is that there are 65,536 ports in each protocol, and
most of them are almost never open. I spent a summer
conducting large-scale scans to determine the prevalence of each TCP
and UDP port. The results include data from scanning tens of
millions of Internet IP addresses as well as enterprise networks
scanned from within. This section provides empirical results you
can rely on to strike the right balance between speed and
effectiveness in your scans.

[]()

While more than a hundred thousand (total) TCP and UDP ports
exist, the vast majority of open ports fall within a much smaller
set. According to our research, the top 10 TCP ports and top 1,075
UDP ports represent half of the open ports for their protocol. To
catch 90% of the open ports, you need to scan 576 TCP ports and 11,307
UDP ports. By default, Nmap scans the top 1,000 ports for each scan
protocol requested. This catches roughly 93% of the TCP ports and 49% of the UDP ports. With the`-F`[]()(fast) option, only
the top 100 ports are scanned, providing 78% TCP effectiveness and 39% for UDP. To specify a different number of ports, specify that value to the `--top-ports` option.[Table 6.1](https://nmap.org/book/performance-port-selection.html#tbl-performance-top-ports-effectiveness)provides an approximation of the number of TCP or
UDP ports you must scan to reach a given effectiveness rate for that
protocol.

Table 6.1. Required `--top-ports` values for reaching various effectiveness levels

|Effectiveness|TCP ports required|UDP ports required|
|-------------|------------------|------------------|
|     10%     |        1         |        5         |
|     20%     |        2         |        12        |
|     30%     |        4         |        27        |
|     40%     |        6         |       135        |
|     50%     |        10        |      1,075       |
|     60%     |        18        |      2,618       |
|     70%     |        44        |      5,157       |
|     80%     |       122        |      7,981       |
|     85%     |       236        |      9,623       |
|     90%     |       576        |      11,307      |
|     95%     |      1,558       |      13,035      |
|     99%     |      3,328       |      15,094      |
|    100%     |      65,536      |      65,536      |

While Nmap can handle port selection for you automatically (when
you rely on defaults or use options such as `-F`or `--top-ports`), specifying ports explicitly
with `-p` is often useful. In either case, familiarity
with the most commonly seen open ports is important. The top ports
according to our data are described in[the section called “What Are the Most Popular Ports?”](https://nmap.org/book/port-scanning.html#most-popular-ports).

---

[Prev](https://nmap.org/book/scantime-coping.html)Coping Strategies for Long Scans

[Up](https://nmap.org/book/performance.html)Chapter 6. Optimizing Nmap Performance

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/performance-low-level.html)Low-Level Timing Controls