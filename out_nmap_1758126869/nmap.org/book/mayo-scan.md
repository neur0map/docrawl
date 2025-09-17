---
title: "Scanning 676,352 IP Addresses in 46 Hours | Nmap Network Scanning"
source_url: https://nmap.org/book/mayo-scan.html
fetched_at: 2025-09-17T16:41:25.059018+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 6. Optimizing Nmap Performance](https://nmap.org/book/performance.html)
* Scanning 676,352 IP Addresses in 46 Hours

[Prev](https://nmap.org/book/performance-timing-templates.html)

[Next](https://nmap.org/book/vscan.html)

Scanning 676,352 IP Addresses in 46 Hours[]()
----------

This story was submitted by Jack L. Mogren of the Mayo Clinic.[]()It functions as a tutorial, demonstrating the steps he took to
implement a regular Nmap scanning regime and reduce scan time of this
huge network from a week to 46 hours.

The Mayo Clinic has built a relatively large private network, with ARP
tables indicating over 70,000 IP addresses in use. Our network
management used to focus on creating and maintaining the physical
architecture across three major campuses and several dozen satellites
across the country. Our motto was “You need it? We'll build
it”. There was little regard for what was actually connected
to the network. Network management conveniently ended at the data jack
and suffered from the candy bar syndrome. It was crunchy and secure
from the outside, but soft and chewy on the inside. We had well
protected boundaries but few internal controls.

This attitude changed abruptly in January 2003 when the Slammer worm
(W32.SQLExp) and its variants broke into our environment. Suddenly it
became very important to know what was connected to our network. In
the case of Slammer, we needed to know where all the devices running
MS SQL Server 2000 or MSDE 2000 were located and who the
administrators were. Lacking this information, the effort to
eradicate Slammer lasted several months.

Thus was born the effort to “Know what's on the
network”. It sounds simplistic, but given size, complexity and
network history, this was a major step forward and a new direction for
our network management services.

Nmap has proven to be a valuable tool in this effort. You can't
beat the price, and I appreciate the advantages that the
open-source[]()community brings to its development. Especially OS fingerprinting and
the many contributions provided by end users.

I began experimenting with Nmap. My goal was to create a
meaningful network inventory by using the Nmap `-O`option to quickly perform remote host identification via TCP/IP
fingerprinting.

Let me start with a few words about our IP environment and my
scanning platform. We currently own one class B and 44 class C ranges
as well as using most of the private address space. That adds up to
676,352 possible IP addresses. I performed my scans from a Compaq
DL380 running Red Hat Linux 8.0. My first attempt was this vanilla TCP
SYN scan with OS detection (`-O`) and only ICMP echo requests for host discovery (`-PE`):

```
# nmap -O -PE -v -oX mayo.xml -iL ip_networks.txt

```

Unfortunately, that proceeded so slowly that it would have taken
a week to scan our entire network. Given that all significant parts
of our network were connected by at least a T1 line (1.54 Mbps), I
added the `insane` canned timing policy
(`-T5`). I also added fast scan mode
(`-F`), which cut the number of ports scanned from
about 1600 to 1200[<sup id="idm45818754172336" class="footnote">[11]</sup>](https://nmap.org/book/mayo-scan.html#ftn.idm45818754172336). I also added `--osscan-limit` so that Nmap doesn't
waste time OS scanning hosts with no ports open. This resulted in the
following command:

```
# nmap -O -T5 -PE -F --osscan-limit -v -oX mayo.xml -iL ip_networks.txt

```

Unfortunately, this looked like it would still take a few days. So I edited the `nmap-services` file to trim down the number of ports to 270. The scan then finished in a little over 49 hours and found 66,558 devices. Tweaking the timing variables, removing the verbose option, and redirecting output to `/dev/null` reduced that time to 46 hours. That left me with this final command:

[]()[]()[]()[]()[]()[]()[]()[]()[]()

```
# nmap -O -T5 -PE -F --osscan-limit --max-rtt-timeout 100ms     \
       --max-parallelism 100 --min-hostgroup 100 -oX mayo.xml \
       -iL ip_networks.txt

```

I plan to perform this scan on a weekly basis and provide the
output in the XML format to an MS SQL database. Our other scan
methods already feed into this database and we are able to create
reports that help us meet our original goal of knowing what's on the
network. I may decide to distribute the load by running subsets of
the scanning on several systems.

---

[<sup class="para">[11] </sup>](https://nmap.org/book/mayo-scan.html#idm45818754172336)With Nmap version 4.75 or higher, `-F` is even more effective in that it cuts the number of scanned ports to 100.

---

[Prev](https://nmap.org/book/performance-timing-templates.html)Timing Templates (-T)

[Up](https://nmap.org/book/performance.html)Chapter 6. Optimizing Nmap Performance

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/vscan.html)Chapter 7. Service and Application Version Detection