---
title: "TCP Connect Scan (-sT) | Nmap Network Scanning"
source_url: https://nmap.org/book/scan-methods-connect-scan.html
fetched_at: 2025-09-17T16:40:03.376605+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 5. Port Scanning Techniques and Algorithms](https://nmap.org/book/scan-methods.html)
* TCP Connect Scan (-sT)

[Prev](https://nmap.org/book/synscan.html)

[Next](https://nmap.org/book/scan-methods-udp-scan.html)

TCP Connect Scan (`-sT`)
----------

[]()[]()

TCP connect scan is the default TCP scan type when SYN scan is
not an option. This is the case when a user does not have raw packet
privileges or is scanning IPv6 networks. Instead of writing raw
packets as most other scan types do, Nmap asks the underlying
operating system to establish a connection with the target machine and
port by issuing the `connect` system call. This is
the same high-level system call that web browsers, P2P clients, and
most other network-enabled applications use to establish a connection.
It is part of a programming interface known as the Berkeley Sockets
API. Rather than read raw packet responses off the wire, Nmap uses
this API to obtain status information on each connection
attempt.[]()This and the FTP bounce scan ([the section called “TCP FTP Bounce Scan (`-b`)”](https://nmap.org/book/scan-methods-ftp-bounce-scan.html)) are the only scan types
available to unprivileged users.

When SYN scan is available, it is usually a better choice. Nmap
has less control over the high level `connect` call
than with raw packets, making it less efficient. The system call
completes connections to open target ports rather than performing the
half-open reset that SYN scan does. Not only does this take longer
and require more packets to obtain the same information, but target
machines are more likely to log the connection. A decent IDS will
catch either, but most machines have no such alarm system. Many services
on your average Unix system will add a note to syslog, and sometimes a
cryptic error message, when Nmap connects and then closes the
connection without sending data. Truly pathetic services crash when
this happens, though that is uncommon. An administrator who sees a
bunch of connection attempts in her logs from a single system should know
that she has been connect scanned.

[Figure 5.5](https://nmap.org/book/scan-methods-connect-scan.html#scan-methods-fig-connect-scan-open) shows a
connect scan in action against open port 22 of scanme.nmap.org.
Recall that this only required three packets in [Figure 5.2, “SYN scan of open port 22”](https://nmap.org/book/synscan.html#scan-methods-fig-syn-scan-open). The exact behavior against an
open port depends on the platform Nmap runs on and the service
listening at the other end, but this five packet example is
typical.

Figure 5.5. Connect scan of open port 22

[

![Connect scan of open port 22](images/ereet/Ereet_Packet_Trace_Connect_Open.png)

]()

[  

The first two steps (SYN and SYN/ACK) are exactly the same as
with a SYN scan. Then, instead of aborting the half-open connection
with a RST packet, krad acknowledges the SYN/ACK with its own ACK
packet, completing the connection. In this case, Scanme even had time
to send its SSH banner string
(`SSH-1.99-OpenSSH_3.1p1\n`) through the now-open
connection. As soon as Nmap hears from its host OS that the connection was
successful, it terminates the connection. TCP connections usually end with
another handshake involving the FIN flag, but Nmap asks the
host OS to terminate the connection immediately with a RST
packet.

]()

[While this connect scan example took almost twice as many packets as
a SYN scan, the bandwidth differences are rarely so
substantial. The vast majority of ports in a large scan will be`closed` or `filtered`. The packet
traces for those are the same as described for SYN scan in ]()[Figure 5.3, “SYN scan of closed port 113”](https://nmap.org/book/synscan.html#scan-methods-fig-syn-scan-closed) and [Figure 5.4, “SYN scan of filtered port 139”](https://nmap.org/book/synscan.html#scan-methods-fig-syn-scan-filtered). Only open ports
generate more network traffic.

The output of a connect scan doesn't differ significantly from
a SYN scan. [Example 5.3](https://nmap.org/book/scan-methods-connect-scan.html#scan-methods-ex-connectscan-scanme)shows a connect scan of Scanme. The `-sT` option
could have been omitted since Nmap is being run from a non-privileged
account so connect scan is the default type.

Example 5.3. Connect scan example

[]()

```
krad~> nmap -T4 -sT scanme.nmap.org

Starting Nmap ( https://nmap.org )
Nmap scan report for scanme.nmap.org (64.13.134.52)
Not shown: 994 filtered ports
PORT    STATE  SERVICE
22/tcp  open   ssh
25/tcp  closed smtp
53/tcp  open   domain
70/tcp  closed gopher
80/tcp  open   http
113/tcp closed auth

Nmap done: 1 IP address (1 host up) scanned in 4.74 seconds

```

[]()

---

[Prev](https://nmap.org/book/synscan.html)TCP SYN (Stealth) Scan (-sS)

[Up](https://nmap.org/book/scan-methods.html)Chapter 5. Port Scanning Techniques and Algorithms

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/scan-methods-udp-scan.html)UDP Scan (-sU)