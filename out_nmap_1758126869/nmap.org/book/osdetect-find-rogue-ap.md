---
title: "SOLUTION: Detect Rogue Wireless Access Points on an Enterprise Network | Nmap Network Scanning"
source_url: https://nmap.org/book/osdetect-find-rogue-ap.html
fetched_at: 2025-09-17T16:43:34.524902+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 8. Remote OS Detection](https://nmap.org/book/osdetect.html)
* SOLUTION: Detect Rogue Wireless Access Points on an Enterprise Network

[Prev](https://nmap.org/book/osdetect-unidentified.html)

[Next](https://nmap.org/book/nse.html)

SOLUTION: Detect Rogue Wireless Access Points on an Enterprise Network
----------

[]()

### Problem ###

With the ubiquity of mobile devices and cheap commodity
networking equipment, companies are increasingly finding that
employees are extending their networks in undesirable ways. Among the
most dangerous devices are 802.11 wireless access points (WAPs). Users may
install a $20 WAP in their cubicle so they can work from the break
room, without realizing (or caring) that they just opened the
protected corporate network to potential attackers in the parking lot
or nearby buildings.[]()

Some WAP installations are even worse than those installed by naive users.
Breaching a building's security is much riskier for an attacker than
accessing corporate data from far away through a network. It carries the risk of
being arrested on the spot. So attackers have been known to install
compact WAPs so they can then intrude on the network at will from the
relative safety of a car down the street. A WAP taped under a desk or
otherwise hidden is unlikely to be noticed for a while.

While the focus of this solution is finding WAPs, the same
strategy can be used to find just about anything. You might need to
locate all Cisco routers to apply a new patch or Solaris boxes to
determine whether you have enough systems to warrant paying for
support.

One way to find unauthorized wireless devices is to sweep the
area with a wireless sniffer such as [Kismet](http://www.kismetwireless.net/)[]()or [NetStumbler](http://www.netstumbler.com/).[]()Another
approach is to scan the wired side with Nmap. Not surprisingly, this
solution focuses exclusively on the latter approach. Each technique
can miss certain WAPs, so the best approach is to do both and merge the results.

### Solution ###

Scan your whole address space using the `-A`option. You can speed it up by limiting scanned ports to
1–85, 113, 443, and 8080–8100. Those should find both an open and closed
port on most WAPs, which improves OS detection accuracy. If your
network spans multiple ethernet segments, scan each segment from a
designated machine on the same segment. This speeds up the scan
(especially since you can do them in parallel), and also gives you the
MAC address of each device.[]()Scanning from the same segment also
allows you to spot stealth devices. Even a WAP with all ports
filtered will generally respond to an ARP request. Results should be
saved in at least normal and XML formats, so you might as well use`-oA`.[]()Consider all of the performance-enhancing
options described in [Chapter 6, *Optimizing Nmap Performance*](https://nmap.org/book/performance.html). A good and
relatively safe start for performance options is `-T4
--min-hostgroup 50 --max-rtt-timeout 1000ms --initial-rtt-timeout 300ms
--max-retries 3 --host-timeout 20m --max-scan-delay 1000ms`.
Put this all together for a command like:

[]()[]()[]()[]()[]()[]()[]()[]()

**nmap -A -oA \~/nmap-logs/wapscan -p
1-85,113,443,8080-8100 -T4 --min-hostgroup 50 --max-rtt-timeout 1000ms
--initial-rtt-timeout 300ms --max-retries 3 --host-timeout 20m
--max-scan-delay 1000ms*`<target_network>`***

When the scan completes, search for WAP characteristics. On a
network of fewer than a couple hundred live hosts, your best bet is to
look at each one individually. For larger networks, you will likely
need to automate the task. Searching for individual characteristics
can be done with grep, though a Perl script which analyzes the XML
output is preferable. This is pretty easy thanks to existing modules,
such asNmap::Scanner[]()and Nmap::Parser,[]()for parsing Nmap
XML output.[]()See [the section called “Manipulating XML Output with Perl”](https://nmap.org/book/output-formats-xml-with-perl.html) for
examples.

Once you determine a list of candidates, it is probably best to
open the normal Nmap output file and examine each one to eliminate false
positives. For example, a Linksys device may be flagged as a possible
WAP even though it could be one of their plain switches
without any wireless functionality.

Once you find the WAPs, it is time to track them down. This can
usually be done by querying the switch they connect to for their physical
ethernet port number.

### WAP Characteristics ###

Now it is time to discuss the WAP characteristics to look for.
Understanding these is useful for manual inspections or for modifying
the WAP finder script to search for something else. You will probably see many of them immediately by looking at the scan of a typical WAP in [Example 8.11](https://nmap.org/book/osdetect-find-rogue-ap.html#osdetect-ex-wapscan).

Example 8.11. Scan results against a consumer WAP

```
# nmap -A -v wap.nmap.org

Starting Nmap ( https://nmap.org )
Nmap scan report for wap.nmap.org (192.168.0.6)
Not shown: 999 closed ports
PORT   STATE SERVICE VERSION
80/tcp open  http    Netgear MR-series WAP (MR814; Embedded HTTPD 1.00)
MAC Address: 00:09:5B:3F:7D:5E (Netgear)
Device type: WAP
Running: Compaq embedded, Netgear embedded
OS details: WAP: Compaq iPAQ Connection Point or Netgear MR814
Service Info: Device: WAP

Nmap done: 1 IP address (1 host up) scanned in 10.90 seconds
           Raw packets sent: 1703 (75.706KB) | Rcvd: 1686 (77.552KB)

```

[]()

This device shows many obvious clues to being a WAP
(`Device type: WAP` is pretty blatant) and some more
subtle ones. But WAPs aren't always so easy to discover. This
section provides a list of WAP characteristics, starting with the most
powerful and ending with heuristics that are long shots or more likely
to produce false positives. Each characteristic listed is accompanied
by an[XPath](http://www.w3.org/TR/xpath)[]()expression
that shows where to find it in Nmap XML output. Since this is security
related, I suggest trying all of them and removing false positives
manually.

TCP/IP fingerprinting device type

As described in [the section called “Device and OS classification (`Class` lines)”](https://nmap.org/book/osdetect-fingerprint-format.html#osdetect-class),
every reference fingerprint has at least one classification (which
includes device type) associated with it. Because WAPs are so
controversial, we try to use that (or give two classifications) when
multiple types would fit. So devices like the D-Link DI-624 wireless
broadband router is classified as `WAP` rather than `switch` or `router`.
The device type can be found in XML output using the XPath expression`/nmaprun/host/os/osclass/@type`. (That is, the`type` attribute of the `osclass`element of the `os` element of any of the`host` elements inside the root`nmaprun` element).

TCP/IP fingerprinting details

While devices with Wireless capability*should* be classified as device type`WAP`, it is worth searching the detailed OS
description for terms such as `wireless` or`wap` just to be sure. The description is in`/nmaprun/host/os/osmatch/@name` in XML output.

Version detection device type

Version detection also tries to determine device
types, but by fingerprinting the target's running services rather than
its IP stack. Check whether the XML `devicetype`attribute located at`/nmaprun/host/ports/port/service/@devicetype` is `WAP`.
To be completely safe, checking the`/nmaprun/host/ports/port/service/@extrainfo` field for
the substrings `wap` or `wireless` is
worthwhile.

Vendor (from MAC address, TCP/IP fingerprinting, and version detection)

Certain vendors specialize in producing the low-cost
consumer networking devices which are most likely to covertly find their way onto
office networks. Examples are Linksys, Netgear, Belkin, SMC, D-Link,
Motorola, Trendnet, Zyxel, and Gateway. You can check for these
vendors based on the MAC address lookup
(which is at `/nmaprun/host/address/@vendor` in XML
output), OS detection
(`/nmaprun/host/os/osclass/@vendor` in XML output), or
version detection
(`/nmaprun/host/ports/port/service/@product` in XML
output) results. Be sure to search for the vendor as a substring of the
fields, since the field may contain incorporation type (e.g. Inc.) or
other information.

This test may lead to many false positives. If you use a vendor
heavily for authorized devices, such as putting Netgear NICs in your
desktop machines, you may have to remove that vendor and rerun the
script.

Hostname

It doesn't hurt to check hostnames (reverse DNS
resolution)[]()[for terms such as `wap`,`wireless`, or `airport`. These can
be found at `/nmaprun/host/hostnames/hostname/@name` in
XML output.
Non-administrative employees rarely change DNS names, but this can be
useful for pen-testers, new administrators, and others who may be
scanning a new network looking for authorized access points.]()

---

[Prev](https://nmap.org/book/osdetect-unidentified.html)Dealing with Misidentified and Unidentified Hosts

[Up](https://nmap.org/book/osdetect.html)Chapter 8. Remote OS Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nse.html)Chapter 9. Nmap Scripting Engine