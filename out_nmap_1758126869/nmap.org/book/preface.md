---
title: "Preface | Nmap Network Scanning"
source_url: https://nmap.org/book/preface.html
fetched_at: 2025-09-17T16:37:26.555533+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* Preface

[Prev](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/organization.html)

Preface
==========

Table of Contents

* [Introduction](https://nmap.org/book/preface.html#preface-intro)
* [Intended Audience and Organization](https://nmap.org/book/organization.html)
* [Conventions](https://nmap.org/book/conventions.html)
* [Other Resources](https://nmap.org/book/resources.html)
* [Request for Comments](https://nmap.org/book/pref-rfc.html)
* [Acknowledgements](https://nmap.org/book/acknowledgements.html)
  * [Technology Used to Create This Book](https://nmap.org/book/acknowledgements.html#acknowledgement-tech)

* [TCP/IP Reference](https://nmap.org/book/tcpip-ref.html)

Introduction
----------

On September 1, 1997, I released a security scanner named
Nmap in the fifty-first issue of *Phrack* magazine. My goal was to
consolidate the fragmented field of special-purpose port scanners into
one powerful and flexible free tool, providing a consistent interface
and efficient implementation of all practical port scanning
techniques. Nmap then consisted of three files (barely
2,000 lines of code) and supported only the Linux operating system.
It was written for my own purposes, and released in the hope that
others would find it useful.

From these humble beginnings, and through the power of open
source development, Nmap grew into the world's most popular network
security scanner, with millions of
users worldwide. Over the years, Nmap has continued to add advanced
functionality such as remote OS detection, version/service detection,
and the Nmap Scripting Engine. It now supports all major Unix, Windows, and Mac OS
platforms with both console and graphical interfaces.
Publications including *Linux Journal*,*Info World*,*LinuxQuestions.Org*, and the*Codetalker Digest* have recognized Nmap as“security tool of the year”. It was even [featured in
nine movies](https://nmap.org/movies.html), including *The Matrix
Reloaded*, *The Girl with the Dragon Tattoo*, *The Bourne Ultimatum*, and *Die Hard 4*.

Nmap (“Network Mapper”) is a free and open source utility for
network exploration and security auditing. Many systems and network
administrators also find it useful for tasks such as network
inventory, managing service upgrade schedules, and monitoring host or
service uptime. Nmap uses raw IP packets in novel ways to determine
what hosts are available on the network, what services (application
name and version) those hosts are offering, what operating systems
(and OS versions) they are running, what type of packet
filters/firewalls are in use, and dozens of other characteristics. It
was designed to rapidly scan large networks, but works fine against
single hosts.

While Nmap is extremely powerful, it is also complex. More than
100 command-line options add expressiveness for networking gurus, but
can confound novices. Some of its options have never even been
documented. This book documents all Nmap features and, more
importantly, teaches the most effective ways of using them. It has
taken nearly four years to write, with constant updating as Nmap has
evolved.

This book is dedicated to the Nmap community of users and developers.
Your passion, ideas, patches, feature requests, flame wars, bug
reports, and midnight rants have shaped Nmap into what it is today.

—Gordon “Fyodor” Lyon `<[fyodor@nmap.org](mailto:fyodor@nmap.org)>`

---

[Prev](https://nmap.org/book/toc.html)Nmap Network Scanning

[Up](https://nmap.org/book/toc.html)Nmap Network Scanning

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/organization.html)Intended Audience and Organization