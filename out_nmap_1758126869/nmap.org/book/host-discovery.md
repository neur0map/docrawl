---
title: "Chapter 3. Host Discovery (“Ping Scanning”) | Nmap Network Scanning"
source_url: https://nmap.org/book/host-discovery.html
fetched_at: 2025-09-17T16:38:56.811014+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* Chapter 3. Host Discovery (“Ping Scanning”)

[Prev](https://nmap.org/book/inst-removing-nmap.html)

[Next](https://nmap.org/book/host-discovery-specify-targets.html)

Chapter 3. Host Discovery (“Ping Scanning”)
==========

Table of Contents

* [Introduction](https://nmap.org/book/host-discovery.html#host-discovery-intro)
* [Specifying Target Hosts and Networks](https://nmap.org/book/host-discovery-specify-targets.html)
  * [Input From List (`-iL`)](https://nmap.org/book/host-discovery-specify-targets.html#host-discovery-iL)
  * [Choose Targets at Random (`-iR *`<numtargets>`*`)](https://nmap.org/book/host-discovery-specify-targets.html#host-discovery-iR)
  * [Excluding Targets (`--exclude`,`--excludefile *`<filename>`*`)](https://nmap.org/book/host-discovery-specify-targets.html#host-discovery--exclude)
  * [Practical Examples](https://nmap.org/book/host-discovery-specify-targets.html#host-discovery-practical)

* [Finding an Organization's IP Addresses](https://nmap.org/book/host-discovery-find-ips.html)
  * [DNS Tricks](https://nmap.org/book/host-discovery-find-ips.html#host-discovery-dns-tricks)
  * [Whois Queries Against IP Registries](https://nmap.org/book/host-discovery-find-ips.html#host-discovery-whois)
  * [Internet Routing Information](https://nmap.org/book/host-discovery-find-ips.html#host-discovery-routing)

* [DNS Resolution](https://nmap.org/book/host-discovery-dns.html)
* [Host Discovery Controls](https://nmap.org/book/host-discovery-controls.html)
  * [List Scan (`-sL`)](https://nmap.org/book/host-discovery-controls.html#host-discovery-list-scan)
  * [Disable Port Scan (`-sn`)](https://nmap.org/book/host-discovery-controls.html#host-discovery-sn)
  * [Disable Ping (`-Pn`)](https://nmap.org/book/host-discovery-controls.html#host-enum-p0)

* [Host Discovery Techniques](https://nmap.org/book/host-discovery-techniques.html)
  * [TCP SYN Ping (`-PS*`<port list>`*`)](https://nmap.org/book/host-discovery-techniques.html#host-discovery-PS)
  * [TCP ACK Ping (`-PA*`<port list>`*`)](https://nmap.org/book/host-discovery-techniques.html#host-discovery-PA)
  * [UDP Ping (`-PU*`<port list>`*`)](https://nmap.org/book/host-discovery-techniques.html#host-discovery-PU)
  * [ICMP Ping Types (`-PE`, `-PP`, and `-PM`)](https://nmap.org/book/host-discovery-techniques.html#host-discovery-icmpping)
  * [IP Protocol Ping (`-PO*`<protocol list>`*`)](https://nmap.org/book/host-discovery-techniques.html#host-discovery-PO)
  * [ARP Scan (`-PR`)](https://nmap.org/book/host-discovery-techniques.html#arp-scan)
  * [Default Combination](https://nmap.org/book/host-discovery-techniques.html#host-discovery-default)

* [Putting It All Together: Host Discovery Strategies](https://nmap.org/book/host-discovery-strategies.html)
  * [Related Options](https://nmap.org/book/host-discovery-strategies.html#host-discovery-related)
  * [Choosing and Combining Ping Options](https://nmap.org/book/host-discovery-strategies.html#host-discovery-combine)
    * [Most valuable probes](https://nmap.org/book/host-discovery-strategies.html#idm45818755883584)
    * [TCP probe and port selection](https://nmap.org/book/host-discovery-strategies.html#idm45818755819712)
    * [UDP port selection](https://nmap.org/book/host-discovery-strategies.html#idm45818755783008)
    * [ICMP probe selection](https://nmap.org/book/host-discovery-strategies.html#idm45818755780992)
    * [Designing the ideal combinations of probes](https://nmap.org/book/host-discovery-strategies.html#host-discovery-ideal-probes)

* [Host Discovery Code Algorithms](https://nmap.org/book/host-discovery-algorithms.html)

[]()

Introduction
----------

[]()

One of the very first steps in any network reconnaissance
mission is to reduce a (sometimes huge) set of IP ranges into a list
of active or interesting hosts. Scanning every port of every single
IP address is slow and usually unnecessary. Of course what makes a
host interesting depends greatly on the scan purposes. Network
administrators may only be interested in hosts running a certain
service, while security auditors may care about every single device
with an IP address. An administrator may be comfortable using just an
ICMP ping to locate hosts on his internal network, while an external
penetration tester may use a diverse set of dozens of probes in an
attempt to evade firewall restrictions.

Because host discovery needs are so diverse, Nmap offers a
wide variety of options for customizing the techniques used. Despite
the name ping scan, this goes well beyond the simple ICMP echo request
packets associated with the ubiquitous pingtool. Users can skip the ping step entirely with a list scan
(`-sL`) or by disabling ping (`-Pn`), or
engage the network with arbitrary combinations of multi-port TCP
SYN/ACK, UDP, and ICMP probes. The goal of these probes is to solicit
responses which demonstrate that an IP address is actually active (is
being used by a host or network device). On many networks, only a
small percentage of IP addresses are active at any given time. This
is particularly common with private address space such
as 10.0.0.0/8. That network has 16.8 million IPs, but I have seen it
used by companies with fewer than a thousand machines. Host
discovery can find those machines in a sparsely allocated sea of
IP addresses.

This chapter first discusses how Nmap ping scanning works
overall, with high-level control options. Then specific techniques
are covered, including how they work and when each is most
appropriate. Nmap offers many ping techniques because it often takes
carefully crafted combinations to get through a series of firewalls and
router filters leading to a target network. Effective overall ping
scanning strategies are discussed, followed by a low-level look
at the algorithms used.

[]()

---

[Prev](https://nmap.org/book/inst-removing-nmap.html)Removing Nmap

[Up](https://nmap.org/book/toc.html)Nmap Network Scanning

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/host-discovery-specify-targets.html)Specifying Target Hosts and Networks