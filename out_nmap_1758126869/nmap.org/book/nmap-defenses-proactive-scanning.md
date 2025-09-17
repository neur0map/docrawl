---
title: "Scan Proactively, Then Close or Block Ports and Fix Vulnerabilities | Nmap Network Scanning"
source_url: https://nmap.org/book/nmap-defenses-proactive-scanning.html
fetched_at: 2025-09-17T16:44:59.992965+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 11. Defenses Against Nmap](https://nmap.org/book/defenses.html)
* Scan Proactively, Then Close or Block Ports and Fix Vulnerabilities

[Prev](https://nmap.org/book/defenses.html)

[Next](https://nmap.org/book/nmap-defenses-firewalls.html)

Scan Proactively, Then Close or Block Ports and Fix Vulnerabilities
----------

[]()

It is often said that the best defense is a good offense. An
excellent way to defend against attackers is to think like them. Scan
your networks regularly and carefully analyze the output for
vulnerabilities. Use crontab on Unix, or the Task Scheduler on
Windows, with a system such as[Ndiff](https://nmap.org/ndiff/)[]()or nmap-report[]()(see [the section called “MadHat in Wonderland”](https://nmap.org/book/nmap-overview-and-demos.html#madhat-story)) to notify you of any changes.

Proactive scanning provides the opportunity to find and fix
vulnerabilities before attackers do. Equally important is closing and
blocking unnecessarily available ports to prevent exploitation by
vulnerabilities you don't yet know about. Proactive scanning also
makes you better aware of what information attackers can obtain. When
you have reviewed the results yourself for weaknesses and are
comfortable with your security posture, port scanners become much less
threatening. The people who are most paranoid about port scanners and
employ the most defensive and detection software are often those who
have the least confidence in their network security. I do not want to
dissuade anyone from using the techniques described throughout this
chapter, but only to suggest that they first seek out and fix any
existing network risks and vulnerabilities. Fixing a hole is far more
effective than trying to hide it. That approach is also less
stressful than constantly worrying that attackers may find the
vulnerabilities.

Once proactive scanning is in place, the first step
is to fix any known vulnerabilities. Next comes audit every open port
available externally through the firewall or on the internal network.
Services which the public doesn't need to reach should be blocked at
the firewall. If employees need to reach them, perhaps they can use
the VPN instead. Internal services are often listening even when they
aren't being used. They might have been installed or enabled by
default, or were enabled due to past use and never disabled. Such
unnecessary services should be disabled. Even if you don't know of a
vulnerability in the service, attackers might. Security bugs might be
found for the service in the future too. A closed port is a much
smaller risk than an open one. Once known holes are fixed, private
services are blocked by the firewall, and unnecessary services
disabled, further defensive technology such as intrusion prevention
systems may be warranted to protect against zero-day exploits,
internal threats, and any holes that your vulnerability analysis
system misses.

Proactive network scanning and auditing should become a routine
rather than a one-off audit. On any complex network, hosts and
services are added and changed regularly. You must keep on top of
these if the network is to remain secure.

Remember that some poorly implemented and tested systems may
react adversely to port scans, OS detection, or version detection.
This is rarely a problem when scanning across the Internet, because
machines that crash when scanned do not last long in such a hostile
environment.[]()Internal machines are often more fragile. When beginning a
proactive scanning program, ensure that it is approved and
communicated to affected parties in advance. Start with a relatively
small part of the network and ensure there are no problems, then take
it further in stages. You may want to start with simple port
scanning, then move on to OS detection or version detection later as
desired.

---

[Prev](https://nmap.org/book/defenses.html)Chapter 11. Defenses Against Nmap

[Up](https://nmap.org/book/defenses.html)Chapter 11. Defenses Against Nmap

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nmap-defenses-firewalls.html)Block and Slow Nmap with Firewalls