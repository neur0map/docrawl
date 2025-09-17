---
title: "IPv6 Scanning (-6) | Nmap Network Scanning"
source_url: https://nmap.org/book/port-scanning-ipv6.html
fetched_at: 2025-09-17T16:39:49.310031+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 4. Port Scanning Overview](https://nmap.org/book/port-scanning.html)
* IPv6 Scanning (-6)

[Prev](https://nmap.org/book/port-scanning-options.html)

[Next](https://nmap.org/book/solution-find-open-port.html)

IPv6 Scanning (`-6`)
----------

[]()

Since 2002, Nmap has offered IPv6 support for its most popular
features. In particular, ping scanning (TCP-only), connect
scanning, and version detection all support IPv6. The command syntax
is the same as usual except that you also add the `-6`option. Of course, you must use IPv6 syntax if you specify an address
rather than a hostname. An address might look like`3ffe:7501:4819:2000:210:f3ff:fe03:14d0`, so hostnames
are recommended. [Example 4.4](https://nmap.org/book/port-scanning-ipv6.html#port-scanning-ex-ipv6) shows a
typical port scanning session. The output looks the same as it usually
does, with the IPv6 address on the “interesting ports”line being the only IPv6 give away.

Example 4.4. A simple IPv6 scan

[]()[]()

```
# nmap -6 -sV www.eurov6.org

Starting Nmap ( https://nmap.org )
Nmap scan report for ns1.euro6ix.com (2001:800:40:2a03::3)
Not shown: 996 closed ports
PORT   STATE SERVICE VERSION
21/tcp open  ftp     Pure-FTPd
22/tcp open  ssh     OpenSSH 3.5p1 (protocol 2.0)
53/tcp open  domain  ISC BIND 9.2.1
80/tcp open  http    Apache httpd

Nmap done: 1 IP address (1 host up) scanned in 56.78 seconds

```

While IPv6 hasn't exactly taken the world by storm, it gets
significant use in some countries and most modern
operating systems support it. To use Nmap with IPv6, both the source
and target of your scan must be configured for IPv6. If your ISP
(like most of them) does not allocate IPv6 addresses to you, free
tunnel brokers[]()are widely available and work fine with Nmap.
I use the free IPv6 tunnel broker service at[`http://www.tunnelbroker.net`](http://www.tunnelbroker.net/). Other tunnel brokers are[listed
at Wikipedia](http://en.wikipedia.org/wiki/List_of_IPv6_tunnel_brokers). 6to4 tunnels are another popular, free
approach.

Systems that support IPv6 don't always have their IPv4 and IPv6
firewall rules in sync. [the section called “IPv6 Attacks”](https://nmap.org/book/firewall-subversion.html#defeating-firewalls-ipv6)shows a real-life example of reaching ports through IPv6 that are
filtered in IPv4.

---

[Prev](https://nmap.org/book/port-scanning-options.html)Command-line Flags

[Up](https://nmap.org/book/port-scanning.html)Chapter 4. Port Scanning Overview

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/solution-find-open-port.html)SOLUTION: Scan a Large Network for a Certain Open TCP Port