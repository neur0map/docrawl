---
title: "Chapter 10. Detecting and Subverting Firewalls and Intrusion Detection Systems | Nmap Network Scanning"
source_url: https://nmap.org/book/firewalls.html
fetched_at: 2025-09-17T16:37:45.877298+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* Chapter 10. Detecting and Subverting Firewalls and Intrusion Detection Systems

[Prev](https://nmap.org/book/nse-implementation.html)

[Next](https://nmap.org/book/firewalls-ids-justification.html)

Chapter 10. Detecting and Subverting Firewalls and Intrusion Detection Systems
==========

Table of Contents

* [Introduction](https://nmap.org/book/firewalls.html#firewalls-ids-intro)
* [Why Would Ethical Professionals (White-hats) Ever Do This?](https://nmap.org/book/firewalls-ids-justification.html)
* [Determining Firewall Rules](https://nmap.org/book/determining-firewall-rules.html)
  * [Standard SYN Scan](https://nmap.org/book/determining-firewall-rules.html#fw-rules-SYN)
    * [Sneaky firewalls that return RST](https://nmap.org/book/determining-firewall-rules.html#idm45818750620944)

  * [ACK Scan](https://nmap.org/book/determining-firewall-rules.html#defeating-firewalls-ids-ackscan)
  * [IP ID Tricks](https://nmap.org/book/determining-firewall-rules.html#defeating-firewalls-ipid-tricks)
  * [UDP Version Scanning](https://nmap.org/book/determining-firewall-rules.html#fw-udp-vscan)

* [Bypassing Firewall Rules](https://nmap.org/book/firewall-subversion.html)
  * [Exotic Scan Flags](https://nmap.org/book/firewall-subversion.html#fw-exotic-flags)
  * [Source Port Manipulation](https://nmap.org/book/firewall-subversion.html#defeating-firewalls-source-port)
  * [IPv6 Attacks](https://nmap.org/book/firewall-subversion.html#defeating-firewalls-ipv6)
  * [IP ID Idle Scanning](https://nmap.org/book/firewall-subversion.html#fw-ipid-scanning)
  * [Multiple Ping Probes](https://nmap.org/book/firewall-subversion.html#fw-multi-pings)
  * [Fragmentation](https://nmap.org/book/firewall-subversion.html#defeating-firewalls-fragmentation)
  * [Proxies](https://nmap.org/book/firewall-subversion.html#fw-proxies)
  * [MAC Address Spoofing](https://nmap.org/book/firewall-subversion.html#defeating-firewalls-mac-spoofing)
  * [Source Routing](https://nmap.org/book/firewall-subversion.html#fw-src-routing)
  * [FTP Bounce Scan](https://nmap.org/book/firewall-subversion.html#fw-ftp-bounce)
  * [Take an Alternative Path](https://nmap.org/book/firewall-subversion.html#fw-alternative-path)
  * [A Practical Real-life Example of Firewall Subversion](https://nmap.org/book/firewall-subversion.html#fw-subversion-example)

* [Subverting Intrusion Detection Systems](https://nmap.org/book/subvert-ids.html)
  * [Intrusion Detection System Detection](https://nmap.org/book/subvert-ids.html#subvert-ids-detection)
    * [Reverse probes](https://nmap.org/book/subvert-ids.html#idm45818750307872)
    * [Sudden firewall changes and suspicious packets](https://nmap.org/book/subvert-ids.html#idm45818750295184)
    * [Naming conventions](https://nmap.org/book/subvert-ids.html#idm45818750289904)
    * [Unexplained TTL jumps](https://nmap.org/book/subvert-ids.html#idm45818750282560)

  * [Avoiding Intrusion Detection Systems](https://nmap.org/book/subvert-ids.html#avoid-ids)
    * [Slow down](https://nmap.org/book/subvert-ids.html#idm45818750255776)
    * [Scatter probes across networks rather than scanning hosts consecutively](https://nmap.org/book/subvert-ids.html#idm45818750216416)
    * [Fragment packets](https://nmap.org/book/subvert-ids.html#idm45818750204336)
    * [Evade specific rules](https://nmap.org/book/subvert-ids.html#idm45818750200880)
    * [Avoid easily detected Nmap features](https://nmap.org/book/subvert-ids.html#idm45818750183520)

  * [Misleading Intrusion Detection Systems](https://nmap.org/book/subvert-ids.html#misleading-ids)
    * [Decoys](https://nmap.org/book/subvert-ids.html#defeating-firewalls-decoys)
    * [Port scan spoofing](https://nmap.org/book/subvert-ids.html#defeating-firewalls-port-scan-spoofing)
    * [Idle scan](https://nmap.org/book/subvert-ids.html#idm45818750147184)
    * [DNS proxying](https://nmap.org/book/subvert-ids.html#defeating-firewalls-dns-proxy)

  * [DoS Attacks Against Reactive Systems](https://nmap.org/book/subvert-ids.html#defeating-ids-dos-attack)
  * [Exploiting Intrusion Detection Systems](https://nmap.org/book/subvert-ids.html#exploit-ids)
  * [Ignoring Intrusion Detection Systems](https://nmap.org/book/subvert-ids.html#ignore-ids)

* [Detecting Packet Forgery by Firewall and Intrusion Detection Systems](https://nmap.org/book/firewall-ids-packet-forgery.html)
  * [Look for TTL Consistency](https://nmap.org/book/firewall-ids-packet-forgery.html#ttl-consistency)
  * [Look for IP ID and Sequence Number Consistency](https://nmap.org/book/firewall-ids-packet-forgery.html#ipid-seqnum-consistency)
  * [The Bogus TCP Checksum Trick](https://nmap.org/book/firewall-ids-packet-forgery.html#bogus-tcp-cksum)
  * [Round Trip Times](https://nmap.org/book/firewall-ids-packet-forgery.html#fw-rtt)
  * [Close Analysis of Packet Headers and Contents](https://nmap.org/book/firewall-ids-packet-forgery.html#fw-pkt-headers)
  * [Unusual Network Uniformity](https://nmap.org/book/firewall-ids-packet-forgery.html#fw-net-uniformity)

[]()

Introduction[]()
----------

Many Internet pioneers envisioned a global open network with a
universal IP address space allowing virtual connections between any
two nodes. This allows hosts to act as true peers, serving and
retrieving information from each other. People could access all of
their home systems from work, changing the climate control settings or
unlocking the doors for early guests. This vision of universal
connectivity has been stifled by address space shortages and security
concerns. In the early 1990s, organizations began deploying
firewalls for the express purpose of reducing connectivity. Huge
networks were cordoned off from the unfiltered Internet by application
proxies,
network address translation[]()devices, and packet filters. The
unrestricted flow of information gave way to tight regulation of
approved communication channels and the content that passes over
them.

Network obstructions such as firewalls can make mapping a
network exceedingly difficult. It will not get any easier, as
stifling casual reconnaissance is often a key goal of implementing the
devices. Nevertheless, Nmap offers many features to help understand these
complex networks, and to verify that filters are working as intended.
It even supports mechanisms for bypassing poorly implemented
defenses. One of the best methods of understanding your
network security posture is to try to defeat it. Place yourself in
the mind-set of an attacker and deploy techniques from this chapter
against your networks. Launch an FTP bounce scan, idle scan,
fragmentation attack, or try to tunnel through one of your own
proxies.

In addition to restricting network activity, companies are
increasingly monitoring traffic with intrusion detection systems
(IDS).[]()All of the major IDSs ship with rules designed to detect Nmap
scans because scans are sometimes a precursor to attacks. Many of
these products have morphed into intrusion*prevention* systems
(IPS)[]()that actively block
traffic deemed malicious. Unfortunately for network administrators
and IDS vendors, reliably detecting bad intentions by analyzing packet
data is a tough problem. Attackers with patience, skill, and the help
of certain Nmap options can usually pass by IDSs undetected.
Meanwhile, administrators must cope with large numbers of false
positive results where innocent activity is misdiagnosed and alerted
on or blocked.

---

[Prev](https://nmap.org/book/nse-implementation.html)Implementation Details

[Up](https://nmap.org/book/toc.html)Nmap Network Scanning

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/firewalls-ids-justification.html)Why Would Ethical Professionals (White-hats) Ever Do This?