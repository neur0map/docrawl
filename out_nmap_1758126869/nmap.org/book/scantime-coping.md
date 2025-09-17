---
title: "Coping Strategies for Long Scans | Nmap Network Scanning"
source_url: https://nmap.org/book/scantime-coping.html
fetched_at: 2025-09-17T16:41:17.098775+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 6. Optimizing Nmap Performance](https://nmap.org/book/performance.html)
* Coping Strategies for Long Scans

[Prev](https://nmap.org/book/reduce-scantime.html)

[Next](https://nmap.org/book/performance-port-selection.html)

Coping Strategies for Long Scans
----------

While optimizing scan options to speed up a scan can take you a
long way, there is a limit to how fast Nmap can run while preserving
accuracy and treating competing network flows fairly. Large scans
involving thousands of hosts, all 65K ports, UDP, or version detection
are likely to take a while even after optimization. This section
provides powerful strategies for coping with these long
scans.

### Use a Multi-stage Approach ###

A comprehensive security audit will need to include UDP and TCP
scanning of all 65,536 ports for each protocol, usually with `-Pn` just
in case a machine is up but heavily filtered. Yet fewer than 100 of
those port numbers are commonly used and most hosts are responsive
with moderate host discovery options. So specify `-F` to perform a quick scan popular ports on known-online hosts first. That lets you analyze the online hosts and most
of the open ports while you start the huge `-Pn` scan of all TCP and UDP
ports with version and OS detection in the background. Short cut
options for speeding up the quick scan are discussed in [the section called “Omit Non-critical Tests”](https://nmap.org/book/reduce-scantime.html#reduce-scantime-omit-tests). Once the slow scan is done,
compare it to the earlier results to find any newly discovered hosts
or ports.

### Estimate and Plan for Scan Time ###

[]()

In many cases, the most frustrating aspect of long scans is having no idea when they will complete. Nmap is now more helpful than it used to be in that it provides regular scan time estimates as long as verbose mode (`-v`) is enabled.

Example 6.2. Estimating scan time

[]()[]()[]()

```
# nmap -T4 -sS -p0- -iR 500 -n --min-hostgroup 100 -v

Starting Nmap ( https://nmap.org )
Initiating SYN Stealth Scan against 29 hosts [65536 ports/host] at 23:27
[...]
SYN Stealth Scan Timing: About 0.30% done; ETC: 09:45 (10:15:45 remaining)

```

[Example 6.2](https://nmap.org/book/scantime-coping.html#performance-ex-time-estimate) shows us that
the SYN scan is likely to take ten hours and eighteen minutes (23:27
to 9:45) to scan 29 hosts. So the total time Nmap will spend scanning
the network can be roughly extrapolated by multiplying 21 minutes per
host by the number of hosts online. If version detection or UDP are
being done as well, you'll also have to watch the timing estimates for
those.

Another option is to wait until Nmap has fully completed
scanning its first group of hosts. Then extrapolate the time taken for
the size of that set over the size of the entire target network. This
is simpler because you don't need to worry about individual scan
components. Basing your estimates on the number of target IP
addresses finished versus the target IP space size can be misleading,
as online hosts are rarely evenly distributed among that IP space.
They are usually found in clumps, often near the beginning of the IP
space. So if the scan itself includes host discovery (i.e. no`-Pn` option), a more accurate measure is to ping scan
the entire network first and then base your estimates on the number of
online hosts Nmap has completed scanning versus the number found
online by the ping scan.

[]()

While occasional estimates are printed automatically in verbose mode, you can always request the current estimate by pressing **`<enter>`** (see [the section called “Runtime Interaction”](https://nmap.org/book/man-runtime-interaction.html)). If the estimate is within your timeframe, you can schedule
something else to do while it proceeds. That beats checking whether
Nmap is done every 20 minutes. An estimate showing that Nmap won't
finish on time is even more valuable. You can immediately work on
optimizing the scan or lengthening the engagement. Your options are
much more limited if you only determine the scan is too slow after the
deadline passes and Nmap is still running.

---

[Prev](https://nmap.org/book/reduce-scantime.html)Scan Time Reduction Techniques

[Up](https://nmap.org/book/performance.html)Chapter 6. Optimizing Nmap Performance

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/performance-port-selection.html)Port Selection Data and Strategies