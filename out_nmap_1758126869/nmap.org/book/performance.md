---
title: "Chapter 6. Optimizing Nmap Performance | Nmap Network Scanning"
source_url: https://nmap.org/book/performance.html
fetched_at: 2025-09-17T16:37:42.310148+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* Chapter 6. Optimizing Nmap Performance

[Prev](https://nmap.org/book/port-scanning-algorithms.html)

[Next](https://nmap.org/book/reduce-scantime.html)

Chapter 6. Optimizing Nmap Performance
==========

Table of Contents

* [Introduction](https://nmap.org/book/performance.html#performance-intro)
* [Scan Time Reduction Techniques](https://nmap.org/book/reduce-scantime.html)
  * [Omit Non-critical Tests](https://nmap.org/book/reduce-scantime.html#reduce-scantime-omit-tests)
  * [Optimize Timing Parameters](https://nmap.org/book/reduce-scantime.html#reduce-timing-param)
  * [Separate and Optimize UDP Scans](https://nmap.org/book/reduce-scantime.html#performance-udp)
  * [Upgrade Nmap](https://nmap.org/book/reduce-scantime.html#reduce-upgrade)
  * [Execute Concurrent Nmap Instances](https://nmap.org/book/reduce-scantime.html#reduce-concurrent)
  * [Scan From a Favorable Network Location](https://nmap.org/book/reduce-scantime.html#reduce-favorable-loc)
  * [Increase Available Bandwidth and CPU Time](https://nmap.org/book/reduce-scantime.html#reduce-more-bandwidth)

* [Coping Strategies for Long Scans](https://nmap.org/book/scantime-coping.html)
  * [Use a Multi-stage Approach](https://nmap.org/book/scantime-coping.html#reduce-multi-stage)
  * [Estimate and Plan for Scan Time](https://nmap.org/book/scantime-coping.html#reduce-estimate)

* [Port Selection Data and Strategies](https://nmap.org/book/performance-port-selection.html)
* [Low-Level Timing Controls](https://nmap.org/book/performance-low-level.html)
* [Timing Templates (`-T`)](https://nmap.org/book/performance-timing-templates.html)
* [Scanning 676,352 IP Addresses in 46 Hours](https://nmap.org/book/mayo-scan.html)

[]()

Introduction
----------

One of my highest Nmap development priorities has always been
performance. A default scan (**nmap*`<hostname>`***) of a host on my local
network takes a fifth of a second. That is barely enough time to
blink, but adds up when you are scanning hundreds or thousands
of hosts. Moreover, certain scan options such as UDP scanning and
version detection can increase scan times substantially. So can
certain firewall configurations, particularly response rate limiting.
While Nmap utilizes parallelism and many advanced algorithms to
accelerate these scans, the user has ultimate control over how Nmap
runs. Expert users carefully craft Nmap commands to obtain only the
information they care about while meeting their time
constraints.

While Nmap performance is a high priority, accuracy is even more
important. Authors of competing scanners have given high-profile
conference presentations about how their scanner only takes four
seconds to scan an entire class B address space. These scanners are
actually trivial to write, since they omit all the congestion
control[]()and packet loss[]()detection algorithms, leaving just a tight loop spewing probe packets
as fast as the system can generate or the wire can bear. Such
scanners are often promoted as stateless—meaning they have also
omitted the code to track and retransmit probes. You can achieve
similar behavior with Nmap by adding flags such as `--min-rate
1000` to request that Nmap send at least 1,000 packets per
second, and `--max-retries 0` to disable retransmission
of timed-out probes. Yet I rarely recommend this. Ninety-nine percent of the packets may be dropped by the next router upstream, and the scanner
will never know the difference.

Unmetered packet blasting scanners such as[Scanrand](https://sectools.org/tools2006.html#scanrand)[]() are useful in some situations,
but Nmap takes a much more conservative and accurate route. Nmap
assumes the worst (high latency and packet loss) of the target
networks at first, then speeds up as it gathers statistics showing
that it can safely do so. While this happens automatically, an
administrator can quicken the learning process by passing hints about
the network to Nmap. An example of such a hint would be`--max-rtt-timeout 200ms`[](), which allows Nmap to assume
that any responses to a target host probe will come within 200
milliseconds.

This chapter first discusses high-level methodologies for
improving scan times. Then it covers how timing templates and
low-level controls are used to speed up Nmap without impacting
accuracy. It finishes with a tutorial by Jack
Mogren[]() of the
Mayo Clinic, detailing how he improved scan time against his
676,352-IP network from nearly a week to 46 hours. Considering the
huge importance of scanner performance, this chapter may seem short.
This is because the chapter focuses on high-level general scanning
performance tips, while tips for optimizing specific scan techniques
are spread throughout this book where those techniques are
covered.

---

[Prev](https://nmap.org/book/port-scanning-algorithms.html)Scan Code and Algorithms

[Up](https://nmap.org/book/toc.html)Nmap Network Scanning

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/reduce-scantime.html)Scan Time Reduction Techniques