---
title: "Version Scanning DB: nmap-service-probes | Nmap Network Scanning"
source_url: https://nmap.org/book/nmap-service-probes.html
fetched_at: 2025-09-17T16:48:16.318684+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 14. Understanding and Customizing Nmap Data Files](https://nmap.org/book/data-files.html)
* Version Scanning DB: nmap-service-probes

[Prev](https://nmap.org/book/nmap-services.html)

[Next](https://nmap.org/book/nmap-rpc.html)

Version Scanning DB: `nmap-service-probes`
----------

[]()

This file contains the probes that the Nmap service/version
detection system (`-sV` or `-A`options)[]()[]()uses
during port interrogation to determine what program is listening on a
port. [Example 14.2](https://nmap.org/book/nmap-service-probes.html#data-files-nmap-service-probes-file) offers a
typical excerpt.

Example 14.2. Excerpt from `nmap-service-probes`

[]()

```
##############################NEXT PROBE##############################
# DNS Server status request: http://www.rfc-editor.org/rfc/rfc1035.txt
Probe UDP DNSStatusRequest q|\0\0\x10\0\0\0\0\0\0\0\0\0|
ports 53,135
match domain m|^\0\0\x90\x04\0\0\0\0\0\0\0\0|
# This one below came from 2 tested Windows XP boxes
match msrpc m|^\x04\x06\0\0\x10\0\0\0\0\0\0\0|
[...]
##############################NEXT PROBE##############################
Probe UDP Help q|help\r\n\r\n|
ports 7,13,37
match chargen m|@ABCDEFGHIJKLMNOPQRSTUVWXYZ|
match echo m|^help\r\n\r\n$|
match time m|^[\xc0-\xc5]...$|

```

The grammar of this file is fully described in [Chapter 7, *Service and Application Version Detection*](https://nmap.org/book/vscan.html). While `nmap-service-probes` is more complex than `nmap-services`, the benefits of improving it can also be greater. Nmap can be taught to actually recognize a company's custom services, rather than simply guess based on `nmap-services` port registration.

The probes in this file are also used in UDP port scanning as[]()protocol-specific payloads sent with some UDP probes. UDP scanning is
difficult because most services don't send a reply to an empty probe,
making it impossible to distinguish `open` and`filtered` ports. The probes here are designed to
be safe to send and to elicit a positive response.

Additionally, some administrators have been using version
detection for tasks well beyond its original intended purpose. A
short probe can cause Nmap to print the title of web pages, recognize
worm-infected machines, locate open proxies, and more. A practical
example of this is provided in [the section called “SOLUTION: Hack Version Detection to Suit Custom Needs, such as Open Proxy Detection”](https://nmap.org/book/vscan-hack-it.html).

[]()

---

[Prev](https://nmap.org/book/nmap-services.html)Well Known Port List: nmap-services

[Up](https://nmap.org/book/data-files.html)Chapter 14. Understanding and Customizing Nmap Data Files

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nmap-rpc.html)SunRPC Numbers: nmap-rpc