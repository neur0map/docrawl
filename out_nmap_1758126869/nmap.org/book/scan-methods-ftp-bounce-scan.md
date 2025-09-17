---
title: "TCP FTP Bounce Scan (-b) | Nmap Network Scanning"
source_url: https://nmap.org/book/scan-methods-ftp-bounce-scan.html
fetched_at: 2025-09-17T16:40:57.558119+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 5. Port Scanning Techniques and Algorithms](https://nmap.org/book/scan-methods.html)
* TCP FTP Bounce Scan (-b)

[Prev](https://nmap.org/book/scan-methods-ip-protocol-scan.html)

[Next](https://nmap.org/book/port-scanning-algorithms.html)

TCP FTP Bounce Scan (`-b`)
----------

[]()[]()

An interesting feature of the FTP protocol ([RFC 959](http://www.rfc-editor.org/rfc/rfc959.txt)) is
support for so-called proxy FTP connections. This allows a user to
connect to one FTP server, then ask that files be sent to a
third-party server. Such a feature is ripe for abuse on many levels,
so most servers have ceased supporting it. One of the abuses this
feature allows is causing the FTP server to port scan other hosts.
Simply ask the FTP server to send a file to each interesting port of a
target host in turn. The error message will describe whether the port
is open or not. This is a good way to bypass firewalls because
organizational FTP servers are often placed where they have
more access to other internal hosts than any old Internet host would. Nmap supports FTP
bounce scan with the `-b` option. It takes an argument
of the form*`<username>`*:*`<password>`*@*`<server>`*:*`<port>`*.*`<Server>`* is the name or IP address of a
vulnerable FTP server. As with a normal URL, you may omit*`<username>`*:*`<password>`*,
in which case anonymous login credentials (user:`anonymous` password:`-wwwuser@`)
are used. The port number (and preceding colon) may be omitted as
well, in which case the default FTP port (21) on*`<server>`* is used.

[Example 5.21](https://nmap.org/book/scan-methods-ftp-bounce-scan.html#scan-methods-ex-ftp-bounce-fixed) shows an attempt to scan google by bouncing off
the main Microsoft FTP server.

Example 5.21. Attempting an FTP bounce scan

[]()[]()

```
# nmap -Pn -b ftp.microsoft.com google.com

Starting Nmap ( https://nmap.org )
Your FTP bounce server doesn't allow privileged ports, skipping them.
Your FTP bounce server sucks, it won't let us feed bogus ports!

```

[]()

Frequent users of the FTP bounce scan better get used to that
error message. This vulnerability was widespread in 1997 when Nmap
was released, but has largely been fixed. Vulnerable servers are
still around, so it is worth trying when all else fails. If bypassing
a firewall is your goal, scan the target network for open port 21 (or
even for any FTP services if you scan all ports with version
detection), then try a bounce scan using each. Nmap will tell you
whether the host is vulnerable or not. If you are just trying to
cover your tracks, you don't need to (and, in fact, shouldn't) limit
yourself to hosts on the target network. Before you go scanning
random Internet addresses for vulnerable FTP servers, consider that
sysadmins may not appreciate you abusing their servers in this
way.

[Example 5.22](https://nmap.org/book/scan-methods-ftp-bounce-scan.html#scan-methods-ftp-bounce-working) shows a successful bounce
scan against a few interesting ports on Scanme. The verbose option
(`-v`) was given to provide extra detail. The given server
type of "JD FTP Server" means that this is an HP JetDirect print server.

Example 5.22. Successful FTP bounce scan

```
krad~> nmap -p 22,25,135 -Pn -v -b XXX.YY.111.2 scanme.nmap.org

Starting Nmap ( https://nmap.org )
Attempting connection to ftp://anonymous:-wwwuser@@XXX.YY.111.2:21
Connected:220 JD FTP Server Ready
Login credentials accepted by ftp server!
Initiating TCP ftp bounce scan against scanme.nmap.org (64.13.134.52)
Adding open port 22/tcp
Adding open port 25/tcp
Scanned 3 ports in 12 seconds via the Bounce scan.
Nmap scan report for scanme.nmap.org (64.13.134.52)
PORT    STATE    SERVICE
22/tcp  open     ssh
25/tcp  open     smtp
135/tcp filtered msrpc

Nmap done: 1 IP address (1 host up) scanned in 21.79 seconds

```

[]()

---

[Prev](https://nmap.org/book/scan-methods-ip-protocol-scan.html)IP Protocol Scan (-sO)

[Up](https://nmap.org/book/scan-methods.html)Chapter 5. Port Scanning Techniques and Algorithms

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/port-scanning-algorithms.html)Scan Code and Algorithms