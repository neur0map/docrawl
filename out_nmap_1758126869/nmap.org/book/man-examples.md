---
title: "Examples | Nmap Network Scanning"
source_url: https://nmap.org/book/man-examples.html
fetched_at: 2025-09-17T16:36:24.812488+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 15. Nmap Reference Guide](https://nmap.org/book/man.html)
* Examples

[Prev](https://nmap.org/book/man-runtime-interaction.html)

[Next](https://nmap.org/book/man-book.html)

Examples
----------

Here are some Nmap usage examples, from the simple and routine to a little more complex and esoteric. Some actual IP addresses and domain names are used to make things more concrete. In their place you should substitute addresses/names from *your own network*. While I don't think port scanning other networks is or should be illegal, some network administrators don't appreciate unsolicited scanning of their networks and may complain. Getting permission first is the best approach.

For testing purposes, you have permission to scan the host scanme.nmap.org.[]() This permission only includes scanning via Nmap and not testing exploits or denial of service attacks. To conserve bandwidth, please do not initiate more than a dozen scans against that host per day. If this free scanning target service is abused, it will be taken down and Nmap will report `Failed to resolve given hostname/IP: scanme.nmap.org`. These permissions also apply to the hosts scanme2.nmap.org, scanme3.nmap.org, and so on, though those hosts do not currently exist.

**nmap -v scanme.nmap.org** []()

This option scans all reserved TCP ports on the machine `scanme.nmap.org` . The `-v` option enables verbose mode.

**nmap -sS -O scanme.nmap.org/24** []() []()

Launches a stealth SYN scan against each machine that is up out of the 256 IPs on the /24 sized network where Scanme resides. It also tries to determine what operating system is running on each host that is up and running. This requires root privileges because of the SYN scan and OS detection.

**nmap -sV -p 22,53,110,143,4564 198.116.0-255.1-127** []()

Launches host enumeration and a TCP scan at the first half of each of the 255 possible eight-bit subnets in the 198.116.0.0/16 address space. This tests whether the systems run SSH, DNS, POP3, or IMAP on their standard ports, or anything on port 4564. For any of these ports found open, version detection is used to determine what application is running.

**nmap -v -iR 100000 -Pn -p 80** []() []()

Asks Nmap to choose 100,000 hosts at random and scan them for web servers (port 80). Host enumeration is disabled with `-Pn` since first sending a couple probes to determine whether a host is up is wasteful when you are only probing one port on each target host anyway.

**nmap -Pn -p80 -oX logs/pb-port80scan.xml -oG logs/pb-port80scan.gnmap 216.163.128.20/20** []() []()

This scans 4096 IPs for any web servers (without pinging them) and saves the output in grepable and XML formats.

---

[Prev](https://nmap.org/book/man-runtime-interaction.html)Runtime Interaction

[Up](https://nmap.org/book/man.html)Chapter 15. Nmap Reference Guide

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/man-book.html)Nmap Book