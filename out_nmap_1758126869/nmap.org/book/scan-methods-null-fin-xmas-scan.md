---
title: "TCP FIN, NULL, and Xmas Scans (-sF, -sN, -sX) | Nmap Network Scanning"
source_url: https://nmap.org/book/scan-methods-null-fin-xmas-scan.html
fetched_at: 2025-09-17T16:40:30.556756+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 5. Port Scanning Techniques and Algorithms](https://nmap.org/book/scan-methods.html)
* TCP FIN, NULL, and Xmas Scans (-sF, -sN, -sX)

[Prev](https://nmap.org/book/scan-methods-udp-scan.html)

[Next](https://nmap.org/book/scan-methods-custom-scanflags.html)

TCP FIN, NULL, and Xmas Scans (`-sF`, `-sN`, `-sX`)
----------

[]()[]()[]()[]()[]()[]()

These three scan types (even more are possible with the`--scanflags`[]()option described in the next section)
exploit a subtle loophole in the [TCP RFC](http://www.rfc-editor.org/rfc/rfc793.txt) to
differentiate between `open` and`closed` ports. Page 65 of RFC 793 says that “if the
[destination] port state is CLOSED .... an incoming segment not
containing a RST causes a RST to be sent in response.” Then the next
page discusses packets sent to open ports without the SYN, RST, or ACK
bits set, stating that: “you are unlikely to get here, but if you do, drop the
segment, and return.”

When scanning systems compliant with this RFC text, any packet
not containing SYN, RST, or ACK bits will result in a returned RST if
the port is closed and no response at all if the port is open. As
long as none of those three bits are included, any combination of the
other three (FIN, PSH, and URG) are OK. Nmap exploits this with three
scan types:

Null scan (`-sN`)

Does not set any bits (TCP flag header is 0)

FIN scan (`-sF`)

Sets just the TCP FIN bit.

Xmas scan (`-sX`)

Sets the FIN, PSH, and URG flags, lighting the packet up like a Christmas tree.

These three scan types are exactly the same in behavior except
for the TCP flags set in probe packets. Responses are treated as
shown in [Table 5.4](https://nmap.org/book/scan-methods-null-fin-xmas-scan.html#scan-methods-tbl-nullfinxmas-scan-responses).

Table 5.4. How Nmap interprets responses to a NULL, FIN, or Xmas scan probe

|                      Probe Response                       |Assigned State |
|-----------------------------------------------------------|---------------|
|     No response received (even after retransmissions)     |`open|filtered`|
|                      TCP RST packet                       |   `closed`    |
|ICMP unreachable error (type 3, code 1, 2, 3, 9, 10, or 13)|  `filtered`   |

The key advantage to these scan types is that they can sneak
through certain non-stateful firewalls and packet filtering routers.
Such firewalls try to prevent incoming TCP connections (while
allowing outbound ones) by blocking any TCP packets with the SYN bit
set and ACK cleared. This configuration is common enough that the Linux
iptables[]()firewall command offers a special `--syn` option to
implement it. The NULL, FIN, and Xmas scans clear the SYN bit and thus
fly right through those rules.

Another advantage is that these scan types are a little more
stealthy than even a SYN scan. Don't count on this though—most
modern IDS products can be configured to detect them.

The big downside is that not all systems follow [RFC 793](http://www.rfc-editor.org/rfc/rfc793.txt) to the
letter. A number of systems send RST responses to the probes
regardless of whether the port is open or not. This causes all of the
ports to be labeled `closed`. Major operating
systems that do this are Microsoft Windows, many Cisco devices,
and IBM OS/400. This scan does work against most Unix-based systems though.
Since Nmap OS detection tests for this quirk, you can learn whether
the scan works against a particular type of system by examining the`nmap-os-db` file. Test T2 sends a NULL
packet to an open port. So if you see a line like`T2(R=N)`, that system seems to support the RFC
and one of these scans should work against it. If the T2 line is
longer, the system violated the RFC by sending a response and these
scans won't work. [Chapter 8, *Remote OS Detection*](https://nmap.org/book/osdetect.html) explains OS
fingerprinting in further detail.

Another downside of these scans is that they can't distinguish
open ports from certain filtered ones. If the packet filter sends an
ICMP destination prohibited error, Nmap knows that a port is filtered.
But most filters simply drop banned probes without any response,
making the ports appear open. Since Nmap cannot be sure which is the
case, it marks non-responsive ports as`open|filtered`.[]()Adding version detection (`-sV`) can
disambiguate as it does with UDP scans, but that defeats much of the
stealthy nature of this scan. If you are willing and able to connect to the
ports anyway, you might as well use a SYN scan.

Using these scan methods is simple. Just add the `-sN`,`-sF`, or`-sX` options to specify the scan type.[Example 5.10](https://nmap.org/book/scan-methods-null-fin-xmas-scan.html#scan-methods-ex-fin-xmas-scan) shows two examples.
The first one, a FIN scan against Para, identifies all five open ports
(as `open|filtered`). The next execution, an Xmas
scan against scanme.nmap.org doesn't work so well. It detects the closed port, but is unable
to differentiate the 995 filtered ports from the four open ones, all
999 are listed as `open|filtered`. This
demonstrates why Nmap offers so many scan methods. No single
technique is preferable in all cases. Ereet will simply have to try
another method to learn more about Scanme.

Example 5.10. Example FIN and Xmas scans

[]()[]()

```
krad# nmap -sF -T4 para

Starting Nmap ( https://nmap.org )
Nmap scan report for para (192.168.10.191)
Not shown: 995 closed ports
PORT     STATE         SERVICE
22/tcp   open|filtered ssh
53/tcp   open|filtered domain
111/tcp  open|filtered rpcbind
515/tcp  open|filtered printer
6000/tcp open|filtered X11
MAC Address: 00:60:1D:38:32:90 (Lucent Technologies)

Nmap done: 1 IP address (1 host up) scanned in 4.64 seconds

krad# nmap -sX -T4 scanme.nmap.org

Starting Nmap ( https://nmap.org )
Nmap scan report for scanme.nmap.org (64.13.134.52)
Not shown: 999 open|filtered ports
PORT    STATE  SERVICE
113/tcp closed auth

Nmap done: 1 IP address (1 host up) scanned in 23.11 seconds

```

Demonstrating the full, firewall-bypassing power of these scans
requires a rather lame target firewall configuration. Unfortunately,
those are easy to find. [Example 5.11](https://nmap.org/book/scan-methods-null-fin-xmas-scan.html#scan-methods-ex-sco-syn-scan) shows a SYN scan of a SCO/Caldera
machine named Docsrv.

Example 5.11. SYN scan of Docsrv

[]()

```
# nmap -sS -T4 docsrv.caldera.com

Starting Nmap ( https://nmap.org )
Nmap scan report for docsrv.caldera.com (216.250.128.247)
(The 997 ports scanned but not shown below are in state: filtered)
PORT    STATE  SERVICE
80/tcp  open   http
113/tcp closed auth
507/tcp open   crs

Nmap done: 1 IP address (1 host up) scanned in 28.62 seconds

```

This example looks OK. Only two ports are open and the rest
(except for 113) are filtered. With a modern stateful firewall, a FIN
scan should not produce any extra information. Yet Ereet tries it anyway,
obtaining the output in [Example 5.12](https://nmap.org/book/scan-methods-null-fin-xmas-scan.html#scan-methods-ex-sco-fin-scan).

Example 5.12. FIN scan of Docsrv

[]()

```
# nmap -sF -T4 docsrv.caldera.com

Starting Nmap ( https://nmap.org )
Nmap scan report for docsrv.caldera.com (216.250.128.247)
Not shown: 961 closed ports
PORT      STATE         SERVICE
7/tcp     open|filtered echo
9/tcp     open|filtered discard
11/tcp    open|filtered systat
13/tcp    open|filtered daytime
15/tcp    open|filtered netstat
19/tcp    open|filtered chargen
21/tcp    open|filtered ftp
22/tcp    open|filtered ssh
23/tcp    open|filtered telnet
25/tcp    open|filtered smtp
37/tcp    open|filtered time
79/tcp    open|filtered finger
80/tcp    open|filtered http
110/tcp   open|filtered pop3
111/tcp   open|filtered rpcbind
135/tcp   open|filtered msrpc
143/tcp   open|filtered imap
360/tcp   open|filtered scoi2odialog
389/tcp   open|filtered ldap
465/tcp   open|filtered smtps
507/tcp   open|filtered crs
512/tcp   open|filtered exec
513/tcp   open|filtered login
514/tcp   open|filtered shell
515/tcp   open|filtered printer
636/tcp   open|filtered ldapssl
712/tcp   open|filtered unknown
955/tcp   open|filtered unknown
993/tcp   open|filtered imaps
995/tcp   open|filtered pop3s
1434/tcp  open|filtered ms-sql-m
2000/tcp  open|filtered callbook
2766/tcp  open|filtered listen
3000/tcp  open|filtered ppp
3306/tcp  open|filtered mysql
6112/tcp  open|filtered dtspc
32770/tcp open|filtered sometimes-rpc3
32771/tcp open|filtered sometimes-rpc5
32772/tcp open|filtered sometimes-rpc7

Nmap done: 1 IP address (1 host up) scanned in 7.64 seconds

```

Wow! That is a lot of apparently open ports. Most of them are
probably open, because having just these 39 filtered and the other
961 closed (sending a RST packet) would be unusual. Yet it is still
possible that some or all are filtered instead of open. FIN scan
cannot determine for sure. We will revisit this case and learn more
about Docsrv later in this chapter.

[]()[]()[]()

---

[Prev](https://nmap.org/book/scan-methods-udp-scan.html)UDP Scan (-sU)

[Up](https://nmap.org/book/scan-methods.html)Chapter 5. Port Scanning Techniques and Algorithms

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/scan-methods-custom-scanflags.html)Custom Scan Types with --scanflags