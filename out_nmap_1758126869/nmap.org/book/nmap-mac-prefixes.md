---
title: "MAC Address Vendor Prefixes: nmap-mac-prefixes | Nmap Network Scanning"
source_url: https://nmap.org/book/nmap-mac-prefixes.html
fetched_at: 2025-09-17T16:48:21.841129+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 14. Understanding and Customizing Nmap Data Files](https://nmap.org/book/data-files.html)
* MAC Address Vendor Prefixes: nmap-mac-prefixes

[Prev](https://nmap.org/book/nmap-os-db.html)

[Next](https://nmap.org/book/nmap-protocols.html)

MAC Address Vendor Prefixes: `nmap-mac-prefixes`
----------

[]()

Users rarely modify this file, which maps MAC address prefixes
to vendor names. Read on for the complete treatment.

Ethernet devices, which have become the dominant network
interface type, are each programmed with a unique 48-bit identifier
known as a MAC address.[]()This address is placed in ethernet headers to
identify which machine on a local network sent a packet, and which
machine the packet is destined for. Humans usually represent it as a
hex string, such as 00:60:1D:38:32:90.

To assure that MAC addresses are unique in a world with
thousands of vendors, the IEEE assigns an Organizationally Unique
Identifier (OUI)[]()[]()to each company manufacturing ethernet devices. The
company must use its own OUI for the first three bytes of MAC
addresses for equipment it produces. For example, the OUI of 00:60:1D:38:32:90
is 00601D. It can choose the remaining
three bytes however it wishes, as long as they are unique. A counter is the
simple approach. Companies that assign all 16.8 million possible values
can obtain more OUIs.`nmap-mac-prefixes` maps each assigned OUI to the
name of the vendor that sells them.[Example 14.5](https://nmap.org/book/nmap-mac-prefixes.html#data-files-nmap-mac-prefixes-file)is a typical excerpt.

Example 14.5. Excerpt from `nmap-mac-prefixes`

[]()

```
006017 Tokimec
006018 Stellar ONE
006019 Roche Diagnostics
00601A Keithley Instruments
00601B Mesa Electronics
00601C Telxon
00601D Lucent Technologies
00601E Softlab
00601F Stallion Technologies
006020 Pivotal Networking
006021 DSC
006022 Vicom Systems
006023 Pericom Semiconductor
006024 Gradient Technologies
006025 Active Imaging PLC
006026 Viking Modular Solutions

```

The first value is the three-byte OUI as 6 hex digits. It is
followed by the company name. This file is created from the complete list at[`http://standards.ieee.org/regauth/oui/oui.txt`](http://standards.ieee.org/regauth/oui/oui.txt)by transforming it with a simple Perl script.
The IEEE[]()also offers an OUI FAQ at [`http://standards.ieee.org/faqs/OUI.html`](http://standards.ieee.org/faqs/OUI.html).

Nmap can determine the MAC address of hosts on a local ethernet
LAN by reading the headers off the wire. It uses this table to look
up and report the manufacturer name based on the OUI. This can be
useful for roughly identifying the type of machine you are dealing
with. A device with a Cisco, Hewlett Packard, or Sun OUI probably
identifies a router, printer, or SPARCstation, respectively. [Example 14.5, “Excerpt from `nmap-mac-prefixes`”](https://nmap.org/book/nmap-mac-prefixes.html#data-files-nmap-mac-prefixes-file) shows that the device
at 00:60:1D:38:32:90 was made by Lucent. It is in
fact the Lucent Orinoco wireless card in my laptop.

[]()

---

[Prev](https://nmap.org/book/nmap-os-db.html)Nmap OS Detection DB: nmap-os-db

[Up](https://nmap.org/book/data-files.html)Chapter 14. Understanding and Customizing Nmap Data Files

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nmap-protocols.html)IP Protocol Number List: nmap-protocols