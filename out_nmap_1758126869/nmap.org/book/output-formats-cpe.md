---
title: "Common Platform Enumeration (CPE) | Nmap Network Scanning"
source_url: https://nmap.org/book/output-formats-cpe.html
fetched_at: 2025-09-17T16:47:42.323359+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 13. Nmap Output Formats](https://nmap.org/book/output.html)
* Common Platform Enumeration (CPE)

[Prev](https://nmap.org/book/output-formats-xml-with-perl.html)

[Next](https://nmap.org/book/output-formats-output-to-database.html)

Common Platform Enumeration (CPE)
----------

[]()

[Common Platform Enumeration](http://cpe.mitre.org/)(CPE) is a standardized way to name software applications, operating
systems, and hardware platforms. Nmap includes CPE output for service
and OS detection.

### Structure of a CPE Name ###

A CPE name is a URL that encodes seven ordered fields:

`cpe:/*`<part>`*:*`<vendor>`*:*`<product>`*:*`<version>`*:*`<update>`*:*`<edition>`*:*`<language>`*`

Some of the fields may be left blank, and empty fields may be left off
the end of the URL. The main division of CPE names is in the`*`<part>`*` field; this can take
on only three values:

|    `a` for applications,     |
|------------------------------|
|`h` for hardware platforms, or|
|  `o` for operating systems.  |

By looking at the beginning of the URL you can easily see that`cpe:/a:microsoft:sql_server:6.5` names an application,`cpe:/h:asus:rt-n16` names a kind of hardware, and`cpe:/o:freebsd:freebsd:3.5.1` names an operating
system.

Nmap can output all three kinds of CPE names: OS detection can print`h` and `o`; and service detection can
potentially output all three. The CPE names are mixed in with normal OS
and service output, for example:

Example 13.13. Normal output with CPE highlighted

```
Running: Linux 2.6.X
OS CPE: cpe:/o:linux:linux_kernel:2.6.39
OS details: Linux 2.6.39
Network Distance: 10 hops
Service Info: OS: Linux; CPE: cpe:/o:linux:kernel

```

CPE names for applications (with part `a`) are not
shown in normal output, but they are present in XML. CPE is represented
as a `cpe` element that can be a child of`service` or `osclass`.

[]()

---

[Prev](https://nmap.org/book/output-formats-xml-with-perl.html)Manipulating XML Output with Perl

[Up](https://nmap.org/book/output.html)Chapter 13. Nmap Output Formats

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/output-formats-output-to-database.html)Output to a Database