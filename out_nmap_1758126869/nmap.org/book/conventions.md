---
title: "Conventions | Nmap Network Scanning"
source_url: https://nmap.org/book/conventions.html
fetched_at: 2025-09-17T16:37:41.056957+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Preface](https://nmap.org/book/preface.html)
* Conventions

[Prev](https://nmap.org/book/organization.html)

[Next](https://nmap.org/book/resources.html)

Conventions
----------

Nmap output is used throughout this book to demonstrate
principles and features. The output is often edited to cut out lines
which are irrelevant to the point being made. The dates/times and
version numbers printed by Nmap are generally removed as well, since
some readers find them distracting. Sensitive information such as
hostnames, IP addresses, and MAC addresses may be changed or
removed. Other information may be cut or lines wrapped so that they
fit on a printed page. Similar editing is done for the output of
other applications. [Example 1](https://nmap.org/book/conventions.html#preface-ex-typical-scan) gives a
glimpse at Nmap's capabilities while also demonstrating output
formatting.

Example 1. A typical Nmap scan

```
# nmap -A -T4 scanme.nmap.org

Starting Nmap ( https://nmap.org )
Nmap scan report for scanme.nmap.org (64.13.134.52)
Host is up (0.034s latency).
Not shown: 994 filtered ports
PORT      STATE  SERVICE VERSION
22/tcp    open   ssh     OpenSSH 4.3 (protocol 2.0)
| ssh-hostkey: 1024 60:ac:4d:51:b1:cd:85:09:12:16:92:76:1d:5d:27:6e (DSA)
|_2048 2c:22:75:60:4b:c3:3b:18:a2:97:2c:96:7e:28:dc:dd (RSA)
53/tcp    open   domain
70/tcp    closed gopher
80/tcp    open   http    Apache httpd 2.2.3 ((CentOS))
| http-methods: Potentially risky methods: TRACE
|_See https://nmap.org/nsedoc/scripts/http-methods.html
|_html-title: Go ahead and ScanMe!
113/tcp   closed auth
31337/tcp closed Elite
Device type: general purpose
Running: Linux 2.6.X
OS details: Linux 2.6.18 (CentOS 5.4)
Network Distance: 10 hops

TRACEROUTE (using port 113/tcp)
HOP RTT      ADDRESS
[Cut first eight hops for brevity]
9   20.29 ms xe6-2.core1.svk.layer42.net (69.36.239.221)
10  19.58 ms scanme.nmap.org (64.13.134.52)

Nmap done: 1 IP address (1 host up) scanned in 25.97 seconds

```

Special formatting is provided for certain tokens, such as filenames and application commands. [Table 1](https://nmap.org/book/conventions.html#preface-tbl-token-formatting) demonstrates the most common formatting conventions.

Table 1. Formatting style conventions

|     Token type      |                                                     Example                                                     |
|---------------------|-----------------------------------------------------------------------------------------------------------------|
|   Literal string    |       I get much more excited by ports in the `open` state than those reported as `closed` or `filtered`.       |
|Command-line options |                   One of the coolest, yet least understood Nmap options is `--packet-trace`.                    |
|      Filenames      |Follow the `-iL` option with the input filename such as `C:\net\dhcp-leases.txt` or `/home/h4x/hosts-to-pwn.lst`.|
|      Emphasis       |       Using Nmap from your work or school computer to attack banks and military targets is a *bad* idea.        |
|Application commands |                    Trinity scanned the Matrix with the command **nmap -v -sS -O 10.2.2.2**.                     |
|Replaceable variables|                Let *`<source>`* be the machine running Nmap and *`<target>`* be `microsoft.com`.                |

---

[Prev](https://nmap.org/book/organization.html)Intended Audience and Organization

[Up](https://nmap.org/book/preface.html)Preface

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/resources.html)Other Resources