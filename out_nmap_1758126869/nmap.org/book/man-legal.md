---
title: "Legal Notices | Nmap Network Scanning"
source_url: https://nmap.org/book/man-legal.html
fetched_at: 2025-09-17T16:36:29.335187+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 15. Nmap Reference Guide](https://nmap.org/book/man.html)
* Legal Notices

[Prev](https://nmap.org/book/man-author.html)

[Next](https://nmap.org/book/ndiff-man.html)

Legal Notices
----------

### Nmap Copyright and Licensing ###

[]()[]()

The Nmap Security Scanner is (C) 1996–2022 Nmap Software
LLC ("The Nmap Project"). Nmap is also a registered trademark of the
Nmap Project. It is published under the [Nmap Public Source License](https://nmap.org/npsl). This
generally allows end users to download and use Nmap for free. It
doesn't allow Nmap to be used and redistributed within commercial
software or hardware products (including appliances, virtual machines,
and traditional applications). We fund the project by selling a
special Nmap OEM Edition for this purpose, as described at [`https://nmap.org/oem`](https://nmap.org/oem). Hundreds of large and small software
vendors have already purchased OEM licenses to embed Nmap technology
such as host discovery, port scanning, OS detection, version
detection, and the Nmap Scripting Engine within their products.

The Nmap Project has permission to redistribute Npcap, a packet
capturing driver and library for the Microsoft Windows platform.
Npcap is a separate work with it's own license rather than this Nmap
license. Since the Npcap license does not permit redistribution
without special permission, our Nmap Windows binary packages which
contain Npcap may not be redistributed without special
permission.

 Even though the NPSL is based on GPLv2, it contains different
provisions and is not directly compatible. It is incompatible with
some other open source licenses as well. In some cases we can
relicense portions of Nmap or grant special permissions to use it in
other open source software. Please contact fyodor@nmap.org with any
such requests. Similarly, we don't incorporate incompatible open
source software into Nmap without special permission from the
copyright holders.

If you have received a written license agreement or contract for
Nmap (such as an [Nmap OEM
license](https://nmap.org/oem/)) stating terms other than these, you may choose to use
and redistribute Nmap under those terms instead.

### Creative Commons License for this Nmap Guide ###

This *Nmap Reference Guide* is (C) 2005–2022 Nmap Software LLC. It is hereby placed under version 3.0 of the [Creative Commons Attribution License](http://creativecommons.org/licenses/by/3.0/). This allows you redistribute and modify the work as you desire, as long as you credit the original source. Alternatively, you may choose to treat this document as falling under the same license as Nmap itself (discussed previously).

### Source Code Availability and Community Contributions ###

Source is provided to this software because we believe users
have a right to know exactly what a program is going to do before they
run it. This also allows you to audit the software for security holes.

Source code also allows you to port Nmap to new platforms, fix
bugs, and add new features. You are highly encouraged to submit your
changes as Github Pull Requests (PR) or send them to`<[dev@nmap.org](mailto:dev@nmap.org)>` for possible incorporation into the main
distribution. By submitting such changes, it is assumed that you are
offering the Nmap Project the unlimited, non-exclusive right to reuse,
modify, and relicense the code. This is important because the
inability to relicense code has caused devastating problems for other
Free Software projects (such as KDE and NASM). We also sell commercial
licenses to [Nmap OEM](https://nmap.org/oem). If you
wish to specify special license conditions of your contributions, just
say so when you send them.

### No Warranty[]() ###

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

It should also be noted that Nmap has occasionally been known to crash
poorly written applications, TCP/IP stacks, and even operating
systems.[]()While this is extremely rare, it is important to keep in
mind. *Nmap should never be run against mission
critical systems* unless you are prepared to suffer
downtime. We acknowledge here that Nmap may crash your systems or
networks and we disclaim all liability for any damage or problems Nmap
could cause.

### Inappropriate Usage ###

Because of the slight risk of crashes and because a few black
hats like to use Nmap for reconnaissance prior to attacking systems,
there are administrators who become upset and may complain when their
system is scanned. Thus, it is often advisable to request permission
before doing even a light scan of a network.

Nmap should never be installed with special privileges
(e.g. suid root).[]()[]()That would open up a major security vulnerability as other users on the
system (or attackers) could use it for privilege escalation.

Nmap is not designed, manufactured, or intended for use in
hazardous environments requiring fail- safe performance where the
failure of the software could lead directly to death, personal injury,
or significant physical or environmental damage.

### Third-Party Software and Funding Notices ###

This product includes software developed by
the [Apache Software
Foundation](https://www.apache.org/). A modified version of the [Libpcap portable packet capture
library](https://www.tcpdump.org/)[]()is distributed along with Nmap.
The Windows version of Nmap utilizes the Libpcap-derived[Ncap library](https://npcap.com/)[]()instead.
Regular expression support is provided by the[PCRE library](https://pcre.org/),[]()which is open-source software, written by Philip Hazel.[]()Certain raw networking functions use the[Libdnet](http://libdnet.sourceforge.net/)[]()networking library, which was written by Dug Song.[]()A modified version is distributed with Nmap.
Nmap can optionally link with the[OpenSSL
cryptography toolkit](https://openssl.org/)[]()for SSL version detection support.
The Nmap Scripting Engine uses an embedded version of
the [Lua programming
language](https://lua.org/).[]() The [Liblinear
linear classification library](https://www.csie.ntu.edu.tw/~cjlin/liblinear/) is used for our IPv6 OS detection machine
learning techniques (see [the section called “IPv6 matching”](https://nmap.org/book/osdetect-guess.html#osdetect-guess-ipv6)). All of the third-party software described in this paragraph is freely
redistributable under BSD-style software licenses.

Binary packages for Windows and Mac OS X include support libraries
necessary to run Zenmap and Ndiff with Python and PyGTK. (Unix platforms
commonly make these libraries easy to install, so they are not part of
the packages.) A listing of these support libraries and their licenses
is included in the `LICENSES` files.

This software was supported in part through the [Google Summer of Code](https://nmap.org/soc/) and the [DARPA CINDER program](https://www.fbo.gov/index?s=opportunity&mode=form&id=585e02a51f77af5cb3c9e06b9cc82c48&tab=core&_cview=1) (DARPA-BAA-10-84).

### United States Export Control[]() ###

Nmap only uses encryption when compiled with the optional
OpenSSL support and linked with OpenSSL. When compiled without
OpenSSL support, the Nmap Project believes that Nmap is not subject to
U.S. [Export
Administration Regulations (EAR)](https://www.bis.doc.gov/index.php/regulations/export-administration-regulations-ear) export control. As such,
there is no applicable ECCN (export control classification number) and
exportation does not require any special license, permit, or other
governmental authorization.

When compiled with OpenSSL support or distributed as source
code, the Nmap Project believes that Nmap falls under
U.S. ECCN [5D002](https://www.bis.doc.gov/index.php/documents/regulations-docs/federal-register-notices/federal-register-2014/951-ccl5-pt2/file)(“Information Security Software”). We distribute Nmap
under the TSU exception for publicly available encryption
software defined
in [EAR
740.13(e)](https://www.bis.doc.gov/index.php/documents/regulations-docs/2341-740-2/file).

---

[Prev](https://nmap.org/book/man-author.html)Authors

[Up](https://nmap.org/book/man.html)Chapter 15. Nmap Reference Guide

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/ndiff-man.html)Chapter 16. Ndiff Reference Guide