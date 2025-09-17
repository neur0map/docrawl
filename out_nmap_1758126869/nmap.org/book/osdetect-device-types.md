---
title: "Device Types | Nmap Network Scanning"
source_url: https://nmap.org/book/osdetect-device-types.html
fetched_at: 2025-09-17T16:43:11.334309+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 8. Remote OS Detection](https://nmap.org/book/osdetect.html)
* Device Types

[Prev](https://nmap.org/book/osdetect-fingerprint-format.html)

[Next](https://nmap.org/book/osdetect-guess.html)

Device Types
----------

[]()

As stated in [the section called “Device and OS classification (`Class` lines)”](https://nmap.org/book/osdetect-fingerprint-format.html#osdetect-class), every reference
fingerprint is classified with one or more device types. This list
contains the device types used by Nmap and the criteria for classifying
a device as each type. These same rules are used to classify the device
types in version detection; see the description of the
d//[]()field in [the section called “`match` Directive”](https://nmap.org/book/vscan-fileformat.html#vscan-db-match).

general purpose

This category contains general-purpose operating systems like Linux and
Windows. In the nmap-service-probes file this class is indicated by a
lack of a d// field.

bridge

A bridge combines two or more subnetworks into one. With a bridge this
happens at a lower level than with a router. This category also includes
things like Ethernet-to-serial bridges.

broadband router

Devices in this category connect a network to the Internet via cable,
ADSL, fiber optics, etc. Some of these devices provide network address
translation, a firewall, port forwarding, or other services.

firewall

A firewall controls what traffic is allowed into or out of a network.
Some also have additional capabilities. This category doesn't include
general-purpose operating systems that happen to come with a firewall,
but it does include OS distributions purpose-built to work only as a
firewall.

game console

A video game console like the Xbox or PlayStation.

hub

A hub joins network segments by re-broadcasting all traffic. Hubs are
distinct from switches, which selectively transmit packets only to
relevant destinations.

load balancer

A device that distributes inbound traffic to multiple devices to ease
the load on those devices.

media device

This category includes all kinds of audiovisual equipment, including
portable music players, home audio systems, TVs, and projectors.

PBX

A private branch exchange, or PBX, routes telephone calls within a
private organization and connects them to the public telephone network
or VoIP.

PDA

A handheld computer. Devices that are also telephones go in the "phone"
category.

phone

A network-capable telephone that is not a VoIP phone. Devices in this
category are typically mobile phones.

power-device

Miscellaneous power devices like uninterruptable power supplies and
surge protectors.

printer

Network-enabled printers, including printers with an embedded print
server.

print server

A print server connects a printer to a network. Printers that contain
their own print server go in the "printer" category instead.

proxy server

Any kind of proxy, including web proxies and other servers that cache
data or understand high-level protocols.

remote management

Devices that allow servers or other equipment to be monitored or managed
remotely.

router

Routers connect multiple networks. They are distinct from hubs and
switches because they route packets between different networks as
opposed to extending one network.

security-misc

Any security device that doesn't fall into the “firewall”category belongs in this category. This includes intrusion detection and
prevention systems.

specialized

The catch-all category. If a device doesn't fall into one of the other
categories, it is specialized. Examples in this category are diverse and
include such things as clocks, oscilloscopes, climate sensors, and more.

storage-misc

Data storage devices like tape decks and network-attached storage
appliances.

switch

A device that extends a network by selectively re-broadcasting packets.
Switches are distinct from hubs, which broadcast all packets.

telecom-misc

Devices used by telephone systems that aren't PBXs, like voicemail and
ISDN systems.

terminal

A device with a keyboard and monitor with the primary purpose of
communicating directly with a terminal server or mainframe.

terminal server

A device providing terminal facilities to clients over a network.

VoIP adapter

A device that converts between voice over IP (VoIP) protocols and normal
telephone traffic. Also may convert different VoIP protocols.

VoIP phone

A phone capable of a VoIP protocol.

WAP

Wireless access points offer a wireless connection to a network. Most
work with radio technology like 802.11b but some use infra-red or
something else. Devices that could also be put in another category, like
wireless broadband routers, are put in the WAP category because WAPs
require special network considerations.

webcam

Any kind of camera that stores or transmits pictures or video. This
includes everything from consumer webcams to security system cameras.

---

[Prev](https://nmap.org/book/osdetect-fingerprint-format.html)Understanding an Nmap Fingerprint

[Up](https://nmap.org/book/osdetect.html)Chapter 8. Remote OS Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/osdetect-guess.html)OS Matching Algorithms