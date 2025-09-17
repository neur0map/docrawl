---
title: "Chapter 14. Understanding and Customizing Nmap Data Files | Nmap Network Scanning"
source_url: https://nmap.org/book/data-files.html
fetched_at: 2025-09-17T16:47:54.638513+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* Chapter 14. Understanding and Customizing Nmap Data Files

[Prev](https://nmap.org/book/output-formats-grepable-output.html)

[Next](https://nmap.org/book/nmap-services.html)

Chapter 14. Understanding and Customizing Nmap Data Files
==========

Table of Contents

* [Introduction](https://nmap.org/book/data-files.html#data-files-intro)
* [Well Known Port List: `nmap-services`](https://nmap.org/book/nmap-services.html)
* [Version Scanning DB: `nmap-service-probes`](https://nmap.org/book/nmap-service-probes.html)
* [SunRPC Numbers: `nmap-rpc`](https://nmap.org/book/nmap-rpc.html)
* [Nmap OS Detection DB: `nmap-os-db`](https://nmap.org/book/nmap-os-db.html)
* [MAC Address Vendor Prefixes: `nmap-mac-prefixes`](https://nmap.org/book/nmap-mac-prefixes.html)
* [IP Protocol Number List: `nmap-protocols`](https://nmap.org/book/nmap-protocols.html)
* [Files Related to Scripting](https://nmap.org/book/data-files-nse.html)
* [Using Customized Data Files](https://nmap.org/book/data-files-replacing-data-files.html)

[]()

Introduction
----------

Nmap relies on seven data files for port scanning and other operations, all of which have names beginning with`nmap-`. One example is`nmap-services`, a registry of port names to their
corresponding port number and protocol. The others, which this chapter
describes one by one, are `nmap-service-probes` (version detection probe database),`nmap-rpc` (SunRPC program name to number database
for direct RPC scanning),`nmap-os-db` (OS detection database),`nmap-mac-prefixes` (ethernet MAC address prefix
(OUI) to vendor lookup table), and`nmap-protocols` (list of IP protocols for
protocol scan).
Additionally this chapter covers certain files related
to scripting with the Nmap Scripting Engine.
The source distribution installs
these files in `/usr/local/share/nmap/` and the
official Linux RPMs put them in `/usr/share/nmap/`.
Other distributions may install them elsewhere.

The latest versions of these files are kept at [`https://nmap.org/svn/`](https://nmap.org/svn/), though it is strongly
recommended that users upgrade to the most recent Nmap version rather
than grabbing newer data files à la carte. There are no guarantees
that newer files will work with older versions of Nmap (though they
almost always do), and the resulting Frankenstein versions of Nmap can
confuse the operating system and service fingerprint submission
process.

Most users never change the data files, but it can be handy for
advanced users who might want to add a version fingerprint or port
assignment for a custom daemon running at their company. This section
provides a description of each file and how they are commonly changed.
The general mechanism for replacing Nmap data files with custom
versions is then discussed. A couple of the files don't relate to
port scanning directly, but they are all discussed here for
convenience.

[]()

---

[Prev](https://nmap.org/book/output-formats-grepable-output.html)Grepable Output (-oG)

[Up](https://nmap.org/book/toc.html)Nmap Network Scanning

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nmap-services.html)Well Known Port List: nmap-services