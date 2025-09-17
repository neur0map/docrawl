---
title: "Nmap OS Detection DB: nmap-os-db | Nmap Network Scanning"
source_url: https://nmap.org/book/nmap-os-db.html
fetched_at: 2025-09-17T16:48:21.315988+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 14. Understanding and Customizing Nmap Data Files](https://nmap.org/book/data-files.html)
* Nmap OS Detection DB: nmap-os-db

[Prev](https://nmap.org/book/nmap-rpc.html)

[Next](https://nmap.org/book/nmap-mac-prefixes.html)

Nmap OS Detection DB: `nmap-os-db`
----------

[]()

The `nmap-os-db` data file contains hundreds of
examples of how different operating systems respond to Nmap's
specialized OS detection probes. It is divided into blocks known as*fingerprints*,[]()with each fingerprint containing an
operating system's name, its general classification, and response data.[Example 14.4](https://nmap.org/book/nmap-os-db.html#data-files-nmap-os-db-file) is an excerpt from the
file showing a couple of typical fingerprints.

Example 14.4. Excerpt from `nmap-os-db`

[]()

```
Fingerprint FreeBSD 7.0
Class FreeBSD | FreeBSD | 7.X | general purpose
SEQ(SP=100-10A%GCD=1-6%ISR=108-112%TI=I%II=I%SS=S%TS=21|22)
OPS(O1=M5B4NW8NNT11%O2=M578NW8NNT11%O3=M280NW8NNT11%O4=M5B4NW8NNT11%O5=M218NW8NNT11%O6=M109NNT11)
WIN(W1=FFFF%W2=FFFF%W3=FFFF%W4=FFFF%W5=FFFF%W6=FFFF)
ECN(R=Y%DF=Y%T=3B-45%TG=40%W=FFFF%O=M5B4NW8%CC=N%Q=)
T1(R=Y%DF=Y%T=3B-45%TG=40%S=O%A=S+%F=AS%RD=0%Q=)
T2(R=N)
T3(R=Y%DF=Y%T=3B-45%TG=40%W=FFFF%S=O%A=S+%F=AS%O=M109NW8NNT11%RD=0%Q=)
T4(R=Y%DF=Y%T=3B-45%TG=40%W=0%S=A%A=Z%F=R%O=%RD=0%Q=)
T5(R=Y%DF=Y%T=3B-45%TG=40%W=0%S=Z%A=S+%F=AR%O=%RD=0%Q=)
T6(R=Y%DF=Y%T=3B-45%TG=40%W=0%S=A%A=Z%F=R%O=%RD=0%Q=)
T7(R=Y%DF=Y%T=3B-45%TG=40%W=0%S=Z%A=S%F=AR%O=%RD=0%Q=)
U1(DF=N%T=3B-45%TG=40%IPL=38%UN=0%RIPL=G%RID=G%RIPCK=G%RUCK=G%RUD=G)
IE(DFI=S%T=3B-45%TG=40%CD=S)

Fingerprint Linux 2.6.17 - 2.6.24
Class Linux | Linux | 2.6.X | general purpose
SEQ(SP=A5-D5%GCD=1-6%ISR=A7-D7%TI=Z%II=I%TS=U)
OPS(O1=M400C%O2=M400C%O3=M400C%O4=M400C%O5=M400C%O6=M400C)
WIN(W1=8018%W2=8018%W3=8018%W4=8018%W5=8018%W6=8018)
ECN(R=Y%DF=Y%T=3B-45%TG=40%W=8018%O=M400C%CC=N%Q=)
T1(R=Y%DF=Y%T=3B-45%TG=40%S=O%A=S+%F=AS%RD=0%Q=)
T2(R=N)
T3(R=Y%DF=Y%T=3B-45%TG=40%W=8018%S=O%A=S+%F=AS%O=M400C%RD=0%Q=)
T4(R=Y%DF=Y%T=3B-45%TG=40%W=0%S=A%A=Z%F=R%O=%RD=0%Q=)
T5(R=Y%DF=Y%T=3B-45%TG=40%W=0%S=Z%A=S+%F=AR%O=%RD=0%Q=)
T6(R=Y%DF=Y%T=3B-45%TG=40%W=0%S=A%A=Z%F=R%O=%RD=0%Q=)
T7(R=Y%DF=Y%T=3B-45%TG=40%W=0%S=Z%A=S+%F=AR%O=%RD=0%Q=)
U1(DF=N%T=3B-45%TG=40%IPL=164%UN=0%RIPL=G%RID=G%RIPCK=G%RUCK=G%RUD=G)
IE(DFI=N%T=3B-45%TG=40%CD=S)

```

The `nmap-os-db` OS database is consulted when
remote OS detection is requested with the`-O` option.[]()In
short, Nmap sends special probes to a target system and compares the
responses with the entries in the OS database. If there is a match, the
database entry likely describes the target system. The process of OS
detection is described fully in [Chapter 8, *Remote OS Detection*](https://nmap.org/book/osdetect.html). See[the section called “Decoding the Subject Fingerprint Format”](https://nmap.org/book/osdetect-fingerprint-format.html#osdetect-fp-format) for a detailed description of the
reference fingerprint format.

`nmap-os-db` is rarely changed by users. Adding
or modifying a fingerprint is a moderately complex process and there is
usually no reason ever to remove one. The best way to get an updated
version of the OS database is to get the latest release of Nmap.

The OS database does not (yet) have information on every networked
operating system ever made. The database grows through the contributions
of Nmap users. If Nmap can't guess an OS but you know what it is, please
submit the fingerprint, following the instructions in[the section called “When Nmap Fails to Find a Match and Prints a Fingerprint”](https://nmap.org/book/osdetect-unidentified.html#osdetect-contrib). Occasionally fingerprints have
errors or become out of date. If you see this, consider submitting a
correction as described in [the section called “When Nmap Guesses Wrong”](https://nmap.org/book/osdetect-unidentified.html#osdetect-wrong). Everyone
benefits when the database is improved, and submitting your improvements
keeps you from having to maintain your own fork of the file.

[]()

---

[Prev](https://nmap.org/book/nmap-rpc.html)SunRPC Numbers: nmap-rpc

[Up](https://nmap.org/book/data-files.html)Chapter 14. Understanding and Customizing Nmap Data Files

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nmap-mac-prefixes.html)MAC Address Vendor Prefixes: nmap-mac-prefixes