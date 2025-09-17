---
title: "Normal Output (-oN) | Nmap Network Scanning"
source_url: https://nmap.org/book/output-formats-normal-output.html
fetched_at: 2025-09-17T16:47:21.554639+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 13. Nmap Output Formats](https://nmap.org/book/output.html)
* Normal Output (-oN)

[Prev](https://nmap.org/book/output-formats-interactive.html)

[Next](https://nmap.org/book/output-formats-script-kiddie.html)

Normal Output (`-oN`)
----------

[]()[]()[]()

Normal output is printed to a file when the `-oN`option is specified with a filename argument. It is similar to interactive
output, except that notes which lose relevance once a scan completes
are removed. It is assumed that the file will be read after Nmap
completes, so estimated completion times and new open port alerts are
redundant to the actual completion time and the ordered port table. Since output
may be saved a long while and reviewed among many other logs, Nmap
prints the execution time, command-line arguments, and Nmap version
number on the first line. A similar line at the end of a scan
divulges final timing and a host count. Those two lines begin with a
pound character to identify them as comments. If your application
must parse normal output rather than XML/grepable formats, ensure that
it ignores comments that it doesn't recognize rather than treating
them as an error and aborting. [Example 13.7](https://nmap.org/book/output-formats-normal-output.html#output-formats-ex-normal) is a typical example of normal
output. Note that `-oN -` was used to prevent
interactive output and send normal output straight to
stdout.[]()

Example 13.7. A typical example of normal output

[]()

```
# nmap -T4 -A -p- -oN - scanme.nmap.org
# Nmap 5.35DC18 scan initiated Sun Jul 18 15:33:26 2010 as: ./nmap -T4 -A -oN - scanme.nmap.org
Nmap scan report for scanme.nmap.org (64.13.134.52)
Host is up (0.045s latency).
Not shown: 993 filtered ports
PORT      STATE  SERVICE VERSION
22/tcp    open   ssh     OpenSSH 4.3 (protocol 2.0)
| ssh-hostkey: 1024 60:ac:4d:51:b1:cd:85:09:12:16:92:76:1d:5d:27:6e (DSA)
|_2048 2c:22:75:60:4b:c3:3b:18:a2:97:2c:96:7e:28:dc:dd (RSA)
25/tcp    closed smtp
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
OS details: Linux 2.6.13 - 2.6.31, Linux 2.6.18
Network Distance: 13 hops

TRACEROUTE (using port 80/tcp)
HOP RTT       ADDRESS
[Cut first 10 hops for brevity]
11  45.16 ms  layer42.car2.sanjose2.level3.net (4.59.4.78)
12  43.97 ms  xe6-2.core1.svk.layer42.net (69.36.239.221)
13  45.15 ms  scanme.nmap.org (64.13.134.52)

OS and Service detection performed. Please report any incorrect results at https://nmap.org/submit/ .
# Nmap done at Sun Jul 18 15:33:48 2010 -- 1 IP address (1 host up) scanned in 22.47 seconds

```

[]()

---

[Prev](https://nmap.org/book/output-formats-interactive.html)Interactive Output

[Up](https://nmap.org/book/output.html)Chapter 13. Nmap Output Formats

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/output-formats-script-kiddie.html)$crIpT kIddI3 0uTPut (-oS)