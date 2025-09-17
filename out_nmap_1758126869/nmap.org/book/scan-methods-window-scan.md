---
title: "TCP Window Scan (-sW) | Nmap Network Scanning"
source_url: https://nmap.org/book/scan-methods-window-scan.html
fetched_at: 2025-09-17T16:40:28.060+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 5. Port Scanning Techniques and Algorithms](https://nmap.org/book/scan-methods.html)
* TCP Window Scan (-sW)

[Prev](https://nmap.org/book/scan-methods-ack-scan.html)

[Next](https://nmap.org/book/scan-methods-maimon-scan.html)

TCP Window Scan (`-sW`)
----------

[]()[]()

Window scan is exactly the same as ACK scan except that it
exploits an implementation detail of certain systems to differentiate
open ports from closed ones, rather than always printing`unfiltered` when a RST is returned. It does this by
examining the TCP Window value[]()of the RST packets returned.
On some
systems, open ports use a positive window size (even for RST packets)
while closed ones have a zero window. Window scan sends the same bare
ACK probe as ACK scan, interpreting the results as shown in [Table 5.6](https://nmap.org/book/scan-methods-window-scan.html#scan-methods-tbl-window-scan-responses).

Table 5.6. How Nmap interprets responses to a Window scan ACK probe

|                      Probe Response                       |Assigned State|
|-----------------------------------------------------------|--------------|
|        TCP RST response with non-zero window field        |    `open`    |
|          TCP RST response with zero window field          |   `closed`   |
|     No response received (even after retransmissions)     |  `filtered`  |
|ICMP unreachable error (type 3, code 1, 2, 3, 9, 10, or 13)|  `filtered`  |

This scan relies on an implementation detail of a minority of
systems out on the Internet, so you can't always trust it. Systems
that don't support it will usually return all ports`closed`. Of course, it is possible that the machine
really has no open ports. If most scanned ports are`closed` but a few common port numbers (such as 22,
25, and 53) are `open`, the system is most likely
susceptible. Occasionally, systems will even show the exact opposite
behavior. If your scan shows 997 open ports and three closed or filtered
ports, then those three may very well be the truly open ones.

While this scan is not suited for every situation, it can be
quite useful on occasion. Recall [Example 5.12, “FIN scan of Docsrv”](https://nmap.org/book/scan-methods-null-fin-xmas-scan.html#scan-methods-ex-sco-fin-scan), which shows many`open|filtered` ports not found in a basic SYN scan.
The problem is that we can't distinguish between open and filtered
ports with that FIN scan. The previous section showed that we could
distinguish them by combining FIN and ACK scan results. In this case,
a Window scan makes it even easier by not requiring the FIN scan
results, as shown in [Example 5.17](https://nmap.org/book/scan-methods-window-scan.html#scan-methods-ex-sco-window-scan).

Example 5.17. Window scan of docsrv.caldera.com

[]()

```
# nmap -sW -T4 docsrv.caldera.com

Starting Nmap ( https://nmap.org )
Nmap scan report for docsrv.caldera.com (216.250.128.247)
Not shown: 961 closed ports
PORT      STATE    SERVICE
7/tcp     open     echo
9/tcp     open     discard
11/tcp    open     systat
13/tcp    open     daytime
15/tcp    open     netstat
19/tcp    open     chargen
21/tcp    open     ftp
22/tcp    open     ssh
23/tcp    open     telnet
25/tcp    open     smtp
37/tcp    open     time
79/tcp    open     finger
80/tcp    open     http
110/tcp   open     pop3
111/tcp   open     rpcbind
135/tcp   filtered msrpc
[14 open ports omitted for brevity]
1434/tcp  filtered ms-sql-m
2000/tcp  open     callbook
2766/tcp  open     listen
3000/tcp  open     ppp
3306/tcp  open     mysql
6112/tcp  open     dtspc
32770/tcp open     sometimes-rpc3
32771/tcp open     sometimes-rpc5
32772/tcp open     sometimes-rpc7

Nmap done: 1 IP address (1 host up) scanned in 7.30 seconds

```

These results are exactly what
Ereet[]()wanted! The same 39
interesting ports are shown as with the FIN scan, but this time it
distinguishes between the two filtered ports (MS-SQL and MSRPC) and
the 37 that are actually open. These are the same results Ereet obtained
by combining FIN and ACK scan results together in the previous
section. Verifying results for consistency is another good reason for
trying multiple scan types against a target network.

[]()

---

[Prev](https://nmap.org/book/scan-methods-ack-scan.html)TCP ACK Scan (-sA)

[Up](https://nmap.org/book/scan-methods.html)Chapter 5. Port Scanning Techniques and Algorithms

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/scan-methods-maimon-scan.html)TCP Maimon Scan (-sM)