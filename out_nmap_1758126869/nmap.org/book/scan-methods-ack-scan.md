---
title: "TCP ACK Scan (-sA) | Nmap Network Scanning"
source_url: https://nmap.org/book/scan-methods-ack-scan.html
fetched_at: 2025-09-17T16:40:37.558712+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 5. Port Scanning Techniques and Algorithms](https://nmap.org/book/scan-methods.html)
* TCP ACK Scan (-sA)

[Prev](https://nmap.org/book/scan-methods-custom-scanflags.html)

[Next](https://nmap.org/book/scan-methods-window-scan.html)

TCP ACK Scan (`-sA`)
----------

[]()[]()

This scan is different than the others discussed so far in that
it never determines `open` (or even`open|filtered`) ports. It is used to map out
firewall rulesets, determining whether they are stateful or not and
which ports are filtered.

ACK scan is enabled by specifying the `-sA` option. Its
probe packet has only the ACK flag set (unless you use`--scanflags`). When scanning unfiltered systems,`open` and `closed` ports will both
return a RST packet. Nmap then labels them as`unfiltered`,[]()meaning that they are reachable by the
ACK packet, but whether they are `open` or`closed` is undetermined. Ports that don't respond,
or send certain ICMP error messages back, are labeled`filtered`. [Table 5.5](https://nmap.org/book/scan-methods-ack-scan.html#scan-methods-tbl-ack-scan-responses) provides the full
details.

Table 5.5. How Nmap interprets responses to an ACK scan probe

|                      Probe Response                       |Assigned State|
|-----------------------------------------------------------|--------------|
|                     TCP RST response                      | `unfiltered` |
|     No response received (even after retransmissions)     |  `filtered`  |
|ICMP unreachable error (type 3, code 1, 2, 3, 9, 10, or 13)|  `filtered`  |

ACK scan usage is similar to most other scan types in that you
simply add a single option flag, `-sA` in this case. [Example 5.15](https://nmap.org/book/scan-methods-ack-scan.html#scan-methods-ex-ack-scan) shows an ACK scan against
Scanme.

Example 5.15. A typical ACK Scan

[]()

```
krad# nmap -sA -T4 scanme.nmap.org

Starting Nmap ( https://nmap.org )
Nmap scan report for scanme.nmap.org (64.13.134.52)
Not shown: 994 filtered ports
PORT    STATE      SERVICE
22/tcp  unfiltered ssh
25/tcp  unfiltered smtp
53/tcp  unfiltered domain
70/tcp  unfiltered gopher
80/tcp  unfiltered http
113/tcp unfiltered auth

Nmap done: 1 IP address (1 host up) scanned in 4.01 seconds

```

One of the most interesting uses of ACK scanning is to
differentiate between stateful and stateless firewalls. See [the section called “ACK Scan”](https://nmap.org/book/determining-firewall-rules.html#defeating-firewalls-ids-ackscan) for how to do this
and why you would want to.

[]()

Sometimes a combination of scan types can be used to glean extra
information from a system. As an example, start by reviewing the FIN
scan of Docsrv in [Example 5.12, “FIN scan of Docsrv”](https://nmap.org/book/scan-methods-null-fin-xmas-scan.html#scan-methods-ex-sco-fin-scan). Nmap
finds the closed ports in that case, but 39 of them are listed as`open|filtered` because Nmap cannot determine between
those two states with a FIN scan. Now look at the ACK scan of the
same host in [Example 5.16, “An ACK scan of Docsrv”](https://nmap.org/book/scan-methods-ack-scan.html#scan-methods-ex-sco-ack-scan). Two of
those 39 previously unidentified ports are shown to be`filtered`. The other 37 (based on the default port
line above the table) are in the state `unfiltered`.
That means `open` or `closed`. If
one scan type identifies a port as `open` or`filtered` and another identifies it as`open` or `closed`, logic dictates
that it must be `open`. By combining both scan
types, we have learned that 37 ports on Docsrv are`open`, two are `filtered`, and 961
are `closed`. While logical deduction worked well
here to determine port states, that technique can't always be counted
on. It assumes that different scan types always return a consistent
state for the same port, which is inaccurate. Firewalls and TCP stack
properties can cause different scans against the same machine to
differ markedly. Against Docsrv, we have seen that a SYN scan
considers the SSH port (`tcp/22`) `filtered`, while an
ACK scan considers it `unfiltered`. When exploring
boundary conditions and strangely configured networks, interpreting
Nmap results is an art that benefits from experience and
intuition.

Example 5.16. An ACK scan of Docsrv

[]()

```
# nmap -sA -T4 docsrv.caldera.com

Starting Nmap ( https://nmap.org )
Nmap scan report for docsrv.caldera.com (216.250.128.247)
Not shown: 998 unfiltered ports
PORT     STATE    SERVICE
135/tcp  filtered msrpc
1434/tcp filtered ms-sql-m

Nmap done: 1 IP address (1 host up) scanned in 7.20 seconds

```

[]()

---

[Prev](https://nmap.org/book/scan-methods-custom-scanflags.html)Custom Scan Types with --scanflags

[Up](https://nmap.org/book/scan-methods.html)Chapter 5. Port Scanning Techniques and Algorithms

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/scan-methods-window-scan.html)TCP Window Scan (-sW)