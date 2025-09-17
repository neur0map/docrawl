---
title: "Chapter 8. Remote OS Detection | Nmap Network Scanning"
source_url: https://nmap.org/book/osdetect.html
fetched_at: 2025-09-17T16:42:29.393870+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* Chapter 8. Remote OS Detection

[Prev](https://nmap.org/book/vscan-hack-it.html)

[Next](https://nmap.org/book/osdetect-usage.html)

Chapter 8. Remote OS Detection
==========

Table of Contents

* [Introduction](https://nmap.org/book/osdetect.html#osdetect-intro)
  * [Reasons for OS Detection](https://nmap.org/book/osdetect.html#osdetect-reasons)
    * [Determining vulnerability of target hosts](https://nmap.org/book/osdetect.html#idm45818753528704)
    * [Tailoring exploits](https://nmap.org/book/osdetect.html#idm45818753525216)
    * [Network inventory and support](https://nmap.org/book/osdetect.html#idm45818753523648)
    * [Detecting unauthorized and dangerous devices](https://nmap.org/book/osdetect.html#idm45818753521344)
    * [Social engineering](https://nmap.org/book/osdetect.html#idm45818753517200)

* [Usage and Examples](https://nmap.org/book/osdetect-usage.html)
* [TCP/IP Fingerprinting Methods Supported by Nmap](https://nmap.org/book/osdetect-methods.html)
  * [Probes Sent](https://nmap.org/book/osdetect-methods.html#osdetect-probes)
    * [Sequence generation (`SEQ`, `OPS`, `WIN`, and `T1`)](https://nmap.org/book/osdetect-methods.html#osdetect-probes-seq)
    * [ICMP echo (`IE`)](https://nmap.org/book/osdetect-methods.html#idm45818753307184)
    * [TCP explicit congestion notification (`ECN`)](https://nmap.org/book/osdetect-methods.html#idm45818753287840)
    * [TCP (`T2`–`T7`)](https://nmap.org/book/osdetect-methods.html#osdetect-probes-t)
    * [UDP (`U1`)](https://nmap.org/book/osdetect-methods.html#idm45818753238560)

  * [Response Tests](https://nmap.org/book/osdetect-methods.html#osdetect-response-tests)
    * [TCP ISN greatest common divisor (`GCD`)](https://nmap.org/book/osdetect-methods.html#osdetect-gcd)
    * [TCP ISN counter rate (`ISR`)](https://nmap.org/book/osdetect-methods.html#osdetect-isr)
    * [TCP ISN sequence predictability index (`SP`)](https://nmap.org/book/osdetect-methods.html#osdetect-sp)
    * [IP ID sequence generation algorithm (`TI`, `CI`, `II`)](https://nmap.org/book/osdetect-methods.html#osdetect-ti)
    * [Shared IP ID sequence Boolean (`SS`)](https://nmap.org/book/osdetect-methods.html#osdetect-ss)
    * [TCP timestamp option algorithm (`TS`)](https://nmap.org/book/osdetect-methods.html#osdetect-ts)
    * [TCP options (`O`, `O1–O6`)](https://nmap.org/book/osdetect-methods.html#osdetect-o)
    * [TCP initial window size (`W`, `W1`–`W6`)](https://nmap.org/book/osdetect-methods.html#osdetect-w)
    * [Responsiveness (`R`)](https://nmap.org/book/osdetect-methods.html#osdetect-resp)
    * [IP don't fragment bit (`DF`)](https://nmap.org/book/osdetect-methods.html#osdetect-df)
    * [Don't fragment (ICMP) (`DFI`)](https://nmap.org/book/osdetect-methods.html#osdetect-dfi)
    * [IP initial time-to-live (`T`)](https://nmap.org/book/osdetect-methods.html#osdetect-t)
    * [IP initial time-to-live guess (`TG`)](https://nmap.org/book/osdetect-methods.html#osdetect-tg)
    * [Explicit congestion notification (`CC`)](https://nmap.org/book/osdetect-methods.html#osdetect-ecn)
    * [TCP miscellaneous quirks (`Q`)](https://nmap.org/book/osdetect-methods.html#osdetect-q)
    * [TCP sequence number (`S`)](https://nmap.org/book/osdetect-methods.html#osdetect-s)
    * [TCP acknowledgment number (`A`)](https://nmap.org/book/osdetect-methods.html#osdetect-ack)
    * [TCP flags (`F`)](https://nmap.org/book/osdetect-methods.html#osdetect-flags)
    * [TCP RST data checksum (`RD`)](https://nmap.org/book/osdetect-methods.html#osdetect-rd)
    * [IP total length (`IPL`)](https://nmap.org/book/osdetect-methods.html#osdetect-ipl)
    * [Unused port unreachable field nonzero (`UN`)](https://nmap.org/book/osdetect-methods.html#osdetect-un)
    * [Returned probe IP total length value (`RIPL`)](https://nmap.org/book/osdetect-methods.html#osdetect-ripl)
    * [Returned probe IP ID value (`RID`)](https://nmap.org/book/osdetect-methods.html#osdetect-rid)
    * [Integrity of returned probe IP checksum value (`RIPCK`)](https://nmap.org/book/osdetect-methods.html#osdetect-ripck)
    * [Integrity of returned probe UDP checksum (`RUCK`)](https://nmap.org/book/osdetect-methods.html#osdetect-ruck)
    * [Integrity of returned UDP data (`RUD`)](https://nmap.org/book/osdetect-methods.html#osdetect-rud)
    * [ICMP response code (`CD`)](https://nmap.org/book/osdetect-methods.html#osdetect-cd)

* [IPv6 fingerprinting](https://nmap.org/book/osdetect-ipv6-methods.html)
  * [Probes Sent](https://nmap.org/book/osdetect-ipv6-methods.html#osdetect-ipv6-probes)
    * [Sequence generation (`S1`–`S6`)](https://nmap.org/book/osdetect-ipv6-methods.html#idm45818752887440)
    * [ICMPv6 echo (`IE1`)](https://nmap.org/book/osdetect-ipv6-methods.html#idm45818752881584)
    * [ICMPv6 echo (`IE2`)](https://nmap.org/book/osdetect-ipv6-methods.html#idm45818752879520)
    * [Node Information Query (`NI`)](https://nmap.org/book/osdetect-ipv6-methods.html#idm45818752874416)
    * [Neighbor Solicitation (`NS`)](https://nmap.org/book/osdetect-ipv6-methods.html#idm45818752871056)
    * [UDP (`U1`)](https://nmap.org/book/osdetect-ipv6-methods.html#idm45818752867504)
    * [TCP explicit congestion notification (`TECN`)](https://nmap.org/book/osdetect-ipv6-methods.html#idm45818752865600)
    * [TCP (`T2`–`T7`)](https://nmap.org/book/osdetect-ipv6-methods.html#idm45818752862768)

  * [Feature extraction](https://nmap.org/book/osdetect-ipv6-methods.html#idm45818752855936)
    * [List of all features](https://nmap.org/book/osdetect-ipv6-methods.html#osdetect-features-ipv6)

  * [Differences from IPv4](https://nmap.org/book/osdetect-ipv6-methods.html#idm45818752809712)

* [Fingerprinting Methods Avoided by Nmap](https://nmap.org/book/osdetect-other-methods.html)
  * [Passive Fingerprinting](https://nmap.org/book/osdetect-other-methods.html#osdetect-passive)
  * [Exploit Chronology](https://nmap.org/book/osdetect-other-methods.html#osdetect-exploit)
  * [Retransmission Times](https://nmap.org/book/osdetect-other-methods.html#osdetect-rtt)
  * [IP Fragmentation](https://nmap.org/book/osdetect-other-methods.html#osdetect-ipfrag)
  * [Open Port Patterns](https://nmap.org/book/osdetect-other-methods.html#osdetect-openports)
  * [Retired Tests](https://nmap.org/book/osdetect-other-methods.html#osdetect-retired)

* [Understanding an Nmap Fingerprint](https://nmap.org/book/osdetect-fingerprint-format.html)
  * [Decoding the Subject Fingerprint Format](https://nmap.org/book/osdetect-fingerprint-format.html#osdetect-fp-format)
    * [Decoding the `SCAN` line of a subject fingerprint](https://nmap.org/book/osdetect-fingerprint-format.html#idm45818752711344)

  * [Decoding the Reference Fingerprint Format](https://nmap.org/book/osdetect-fingerprint-format.html#osdetect-ref-format)
    * [Free-form OS description (`Fingerprint` line)](https://nmap.org/book/osdetect-fingerprint-format.html#osdetect-description)
    * [Device and OS classification (`Class` lines)](https://nmap.org/book/osdetect-fingerprint-format.html#osdetect-class)
    * [CPE name (`CPE` lines)](https://nmap.org/book/osdetect-fingerprint-format.html#osdetect-cpe)
    * [Test expressions](https://nmap.org/book/osdetect-fingerprint-format.html#osdetect-test-expressions)

  * [IPv6 fingerprints](https://nmap.org/book/osdetect-fingerprint-format.html#osdetect-ipv6-fingerprints)

* [Device Types](https://nmap.org/book/osdetect-device-types.html)
* [OS Matching Algorithms](https://nmap.org/book/osdetect-guess.html)
  * [IPv4 matching](https://nmap.org/book/osdetect-guess.html#osdetect-guess-ipv4)
  * [IPv6 matching](https://nmap.org/book/osdetect-guess.html#osdetect-guess-ipv6)

* [Dealing with Misidentified and Unidentified Hosts](https://nmap.org/book/osdetect-unidentified.html)
  * [When Nmap Guesses Wrong](https://nmap.org/book/osdetect-unidentified.html#osdetect-wrong)
  * [When Nmap Fails to Find a Match and Prints a Fingerprint](https://nmap.org/book/osdetect-unidentified.html#osdetect-contrib)
  * [Modifying the `nmap-os-db` Database Yourself](https://nmap.org/book/osdetect-unidentified.html#osdetect-db-modification)

* [SOLUTION: Detect Rogue Wireless Access Points on an Enterprise Network](https://nmap.org/book/osdetect-find-rogue-ap.html)
  * [Problem](https://nmap.org/book/osdetect-find-rogue-ap.html#osdetect-problem)
  * [Solution](https://nmap.org/book/osdetect-find-rogue-ap.html#osdetect-solution)
  * [WAP Characteristics](https://nmap.org/book/osdetect-find-rogue-ap.html#osdetect-wap)

[]()[]()

Introduction
----------

When exploring a network for security auditing or
inventory/administration, you usually want to know more than
the bare IP addresses of identified machines. Your reaction to
discovering a printer may be very different than to finding a router,
wireless access point, telephone PBX, game console, Windows desktop,
or Unix server. Finer grained detection (such as distinguishing Mac
OS X 10.4 from 10.3) is useful for determining vulnerability to
specific flaws and for tailoring effective exploits for those
vulnerabilities.

In part due to its value to attackers, many systems are
tight-lipped about their exact nature and operating system
configuration. Fortunately, Nmap includes a huge database of
heuristics for identifying thousands of different systems based on how
they respond to a selection of TCP/IP probes. Another system (part of
version detection) interrogates open TCP or UDP ports to determine
device type and OS details. Results of these two systems are reported
independently so that you can identify combinations such as a
Checkpoint firewall forwarding port 80 to a Windows IIS server.

[]()

While Nmap has supported OS detection since 1998, this chapter
describes the 2nd generation system released in 2006.

### Reasons for OS Detection[]() ###

While some benefits of discovering the underlying OS and device types on a network are obvious, others are more obscure. This section lists the top reasons I hear for discovering this extra information.

#### Determining vulnerability of target hosts ####

It is sometimes very difficult to determine remotely whether an
available service is susceptible or patched for a certain
vulnerability. Even obtaining the application version number doesn't
always help, since OS distributors often back-port security fixes
without changing the version number. The surest way to verify that a
vulnerability is real is to exploit it, but that risks crashing the
service and can lead to wasted hours or even days of frustrating
exploitation efforts if the service turns out to be patched.

OS detection can help reduce these false positives. For
example, the Rwho daemon on unpatched Sun Solaris 7 through 9 may be
remotely exploitable (Sun alert #57659). Remotely determining
vulnerability is difficult, but you can rule it out by finding that a
target system is running Solaris 10.

Taking this from the perspective of a systems administrator
rather than a pen-tester, imagine you run a large Sun shop when alert
#57659 comes out. Scan your whole network with OS detection to find
machines which need patching before the bad guys do.

#### Tailoring exploits ####

Even after you discover a vulnerability in a target system, OS
detection can be helpful in exploiting it. Buffer overflows,
format-string exploits, and many other vulnerabilities often require
custom-tailored shellcode with offsets and assembly payloads generated
to match the target OS and hardware architecture. In some cases, you
only get one try because the service crashes if you get the
shellcode wrong. Use OS detection first or you may end up sending
Linux shellcode to a FreeBSD server.

#### Network inventory and support ####

[]()

While it isn't as exciting as busting root through a specially
crafted format string exploit, there are many administrative reasons
to keep track of what is running on your network. Before you renew
that IRIX support contract for another year, scan to see if anyone
still uses such machines. An inventory can also be useful for IT
budgeting and ensuring that all company equipment is accounted
for.

#### Detecting unauthorized and dangerous devices ####

With the ubiquity of mobile devices and cheap commodity
networking equipment, companies are increasingly finding that
employees are extending their networks in undesirable ways. They may
install a $20
wireless access point (WAP)[]()[]()in their cubicle without
realizing (or caring) that they just opened up the protected corporate
network to potential attackers in the parking lot or nearby buildings.
WAPs can be so dangerous that Nmap has a special category for
detecting them, as demonstrated in [the section called “SOLUTION: Detect Rogue Wireless Access Points on an Enterprise Network”](https://nmap.org/book/osdetect-find-rogue-ap.html). Users may also cause sysadmins grief by connecting insecure and/or
worm-infected laptops to the corporate network. Regular scanning can detect unauthorized
devices for investigation and containment.

#### Social engineering ####

[]()

Another possible use is social engineering. Lets say that you
are scanning a target company and Nmap reports a “Datavoice
TxPORT PRISM 3000 T1 CSU/DSU 6.22/2.06”. You could call up the
target pretending to be Datavoice support and discuss some issues with
their PRISM 3000. Tell them you are about to announce a big security
hole, but are first providing the patch to valued customers. Some
naive administrators might assume that only an authorized engineer
from Datavoice would know so much about their CSU/DSU. Of course the
patch you send them is a Trojan horse that gives you remote access to
sniff and traipse through their network. Be sure to read the rest of
this chapter for detection accuracy and verification advice before
trying this. If you guess the target system wrong and they call the
police, that will be an embarrassing story to tell your
cellmates.

[]()

---

[Prev](https://nmap.org/book/vscan-hack-it.html)SOLUTION: Hack Version Detection to Suit Custom Needs, such as Open Proxy Detection

[Up](https://nmap.org/book/toc.html)Nmap Network Scanning

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/osdetect-usage.html)Usage and Examples