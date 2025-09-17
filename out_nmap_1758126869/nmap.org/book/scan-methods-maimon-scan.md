---
title: "TCP Maimon Scan (-sM) | Nmap Network Scanning"
source_url: https://nmap.org/book/scan-methods-maimon-scan.html
fetched_at: 2025-09-17T16:40:56.396011+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 5. Port Scanning Techniques and Algorithms](https://nmap.org/book/scan-methods.html)
* TCP Maimon Scan (-sM)

[Prev](https://nmap.org/book/scan-methods-window-scan.html)

[Next](https://nmap.org/book/idlescan.html)

TCP Maimon Scan (`-sM`)
----------

[]()[]()

The Maimon scan is named after its discoverer,
Uriel Maimon.[]()He described the technique in*Phrack* Magazine issue #49 (November 1996).[]()Nmap, which included this technique, was released two issues later.
This technique is exactly the same as NULL, FIN, and Xmas scan, except
that the probe is FIN/ACK. According to [RFC 793](http://www.rfc-editor.org/rfc/rfc793.txt) (TCP), a RST packet
should be generated in response to such a probe whether the port is
open or closed. However, Uriel noticed that many BSD-derived systems
simply drop the packet if the port is open. Nmap takes
advantage of this to determine open ports, as shown in [Table 5.7](https://nmap.org/book/scan-methods-maimon-scan.html#scan-methods-tbl-maimon-scan-responses).

Table 5.7. How Nmap interprets responses to a Maimon scan probe

|                      Probe Response                       |Assigned State |
|-----------------------------------------------------------|---------------|
|     No response received (even after retransmissions)     |`open|filtered`|
|                      TCP RST packet                       |   `closed`    |
|ICMP unreachable error (type 3, code 1, 2, 3, 9, 10, or 13)|  `filtered`   |

The Nmap flag for a Maimon scan is `-sM`. While
this option was quite useful in 1996, modern systems rarely exhibit
this bug. They send a RST back for all ports, making every port
appear closed. This result is shown in [Example 5.18](https://nmap.org/book/scan-methods-maimon-scan.html#scan-methods-ex-maimon-scan).

Example 5.18. A failed Maimon scan

[]()

```
# nmap -sM -T4 para

Starting Nmap ( https://nmap.org )
All 1000 scanned ports on para (192.168.10.191) are: closed
MAC Address: 00:60:1D:38:32:90 (Lucent Technologies)

Nmap done: 1 IP address (1 host up) scanned in 4.19 seconds

```

---

[Prev](https://nmap.org/book/scan-methods-window-scan.html)TCP Window Scan (-sW)

[Up](https://nmap.org/book/scan-methods.html)Chapter 5. Port Scanning Techniques and Algorithms

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/idlescan.html)TCP Idle Scan (-sI)