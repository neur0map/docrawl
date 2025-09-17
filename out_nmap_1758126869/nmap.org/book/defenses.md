---
title: "Chapter 11. Defenses Against Nmap | Nmap Network Scanning"
source_url: https://nmap.org/book/defenses.html
fetched_at: 2025-09-17T16:44:57.365221+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* Chapter 11. Defenses Against Nmap

[Prev](https://nmap.org/book/firewall-ids-packet-forgery.html)

[Next](https://nmap.org/book/nmap-defenses-proactive-scanning.html)

Chapter 11. Defenses Against Nmap
==========

Table of Contents

* [Introduction](https://nmap.org/book/defenses.html#nmap-defenses-intro)
* [Scan Proactively, Then Close or Block Ports and Fix Vulnerabilities](https://nmap.org/book/nmap-defenses-proactive-scanning.html)
* [Block and Slow Nmap with Firewalls](https://nmap.org/book/nmap-defenses-firewalls.html)
* [Detect Nmap Scans](https://nmap.org/book/nmap-defenses-detection.html)
* [Clever Trickery](https://nmap.org/book/nmap-defenses-trickery.html)
  * [Hiding Services on Obscure Ports](https://nmap.org/book/nmap-defenses-trickery.html#nmap-defenses-hiding)
  * [Port Knocking](https://nmap.org/book/nmap-defenses-trickery.html#nmap-defenses-knock)
  * [Honeypots and Honeynets](https://nmap.org/book/nmap-defenses-trickery.html#nmap-defenses-honeypot)
  * [OS Spoofing](https://nmap.org/book/nmap-defenses-trickery.html#nmap-defenses-os-spoofing)
  * [Tar Pits](https://nmap.org/book/nmap-defenses-trickery.html#nmap-defenses-tarpit)
  * [Reactive Port Scan Detection](https://nmap.org/book/nmap-defenses-trickery.html#nmap-defense-reactive-port-sentry)
  * [Escalating Arms Race](https://nmap.org/book/nmap-defenses-trickery.html#nmap-defenses-arms-race)

[]()

Introduction
----------

[Chapter 10, *Detecting and Subverting Firewalls and Intrusion Detection Systems*](https://nmap.org/book/firewalls.html) discussed the myriad ways that Nmap (along with a few other
open-source security tools) can be used to slip through firewalls and
outsmart intrusion detection systems. Now we look at the situation
from the other side of the fence: How technology such as firewalls
and IDSs can defend against Nmap. Possible defenses include
blocking the probes, restricting information returned, slowing down
the Nmap scan, and returning misleading information. The dangers of
some defenses are covered as well. Obfuscating your network to the
extent that attackers cannot understand what is going on is not a net
win if your administrators no longer understand it either. Similarly,
defensive software meant to confuse or block port scanners is not
beneficial if it opens up more serious vulnerabilities itself. Many
of the techniques described herein protect against active probes in
general, not just those produced with Nmap.

---

[Prev](https://nmap.org/book/firewall-ids-packet-forgery.html)Detecting Packet Forgery by Firewall and Intrusion Detection Systems

[Up](https://nmap.org/book/toc.html)Nmap Network Scanning

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nmap-defenses-proactive-scanning.html)Scan Proactively, Then Close or Block Ports and Fix Vulnerabilities