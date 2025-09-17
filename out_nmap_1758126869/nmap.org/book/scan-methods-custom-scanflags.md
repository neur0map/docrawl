---
title: "Custom Scan Types with --scanflags | Nmap Network Scanning"
source_url: https://nmap.org/book/scan-methods-custom-scanflags.html
fetched_at: 2025-09-17T16:40:29.307594+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 5. Port Scanning Techniques and Algorithms](https://nmap.org/book/scan-methods.html)
* Custom Scan Types with --scanflags

[Prev](https://nmap.org/book/scan-methods-null-fin-xmas-scan.html)

[Next](https://nmap.org/book/scan-methods-ack-scan.html)

Custom Scan Types with `--scanflags`
----------

[]()

Truly advanced Nmap users need not limit themselves to the
canned scanned types. The `--scanflags` option allows
you to design your own scan by specifying arbitrary TCP flags. Let
your creative juices flow, while evading intrusion detection systems whose vendors simply paged through the Nmap man page adding specific rules!

The `--scanflags` argument can be a numerical
flag value such as 9 (PSH and FIN), but using symbolic names is
easier. Just mash together any combination of `URG`,`ACK`, `PSH`,`RST`, `SYN`, and`FIN`. For example, `--scanflags
URGACKPSHRSTSYNFIN` sets everything, though it's not very
useful for scanning. The order these are specified in is
irrelevant.

In addition to specifying the desired flags, you can specify a
TCP scan type (such as `-sA` or `-sF`).
That base type tells Nmap how to interpret responses. For
example, a SYN scan considers no-response indicative of a`filtered` port, while a FIN scan treats the same as`open|filtered`. Nmap will behave the same way it
does for the base scan type, except that it will use the TCP flags you
specify instead. If you don't specify a base type, SYN scan is
used.

### Custom SYN/FIN Scan ###

One interesting custom scan type is SYN/FIN. Sometimes a
firewall administrator or device manufacturer will attempt to block incoming
connections with a rule such as “drop any incoming packets with
only the SYN flag set”. They limit it to*only* the SYN flag because they don't want to
block the SYN/ACK packets which are returned as the second step of an
outgoing connection.

The problem with this approach is that most end systems will
accept initial SYN packets which contain other (non-ACK) flags as well.
For example, the Nmap OS fingerprinting system sends a SYN/FIN/URG/PSH
packet to an open port. More than half of the fingerprints in the
database respond with a SYN/ACK. Thus they allow port scanning with
this packet and generally allow making a full TCP connection too.
Some systems have even been known to respond with SYN/ACK to a SYN/RST
packet! The [TCP RFC](http://www.rfc-editor.org/rfc/rfc793.txt) is ambiguous as to which flags are acceptable in
an initial SYN packet, though SYN/RST certainly seems bogus.

[Example 5.13](https://nmap.org/book/scan-methods-custom-scanflags.html#scan-methods-ex-custom-synfin-scan) shows Ereet conducting a successful SYN/FIN scan of Google. He is apparently getting bored with scanme.nmap.org.

Example 5.13. A SYN/FIN scan of Google

[]()[]()

```
krad# nmap -sS --scanflags SYNFIN -T4 www.google.com

Starting Nmap ( https://nmap.org )
Warning: Hostname www.google.com resolves to 4 IPs. Using 74.125.19.99.
Nmap scan report for cf-in-f99.google.com (74.125.19.99)
Not shown: 996 filtered ports
PORT    STATE  SERVICE
80/tcp  open   http
113/tcp closed auth
179/tcp closed bgp
443/tcp open   https

Nmap done: 1 IP address (1 host up) scanned in 7.58 seconds

```

Similar scan types, such as SYN/URG or SYN/PSH/URG/FIN will
generally work as well. If you aren't getting through, don't forget
the already mentioned SYN/RST option.

### PSH Scan ###

[]()

[the section called “TCP FIN, NULL, and Xmas Scans (`-sF`, `-sN`, `-sX`)”](https://nmap.org/book/scan-methods-null-fin-xmas-scan.html) noted that
RFC-compliant systems allow one to scan ports using any combination of
the FIN, PSH, and URG flags. While there are eight possible
permutations, Nmap only offers three canned modes (NULL, FIN, and
Xmas). Show some personal flair by trying a PSH/URG or FIN/PSH scan
instead. Results rarely differ from the three canned modes, but there
is a small chance of evading scan detection systems.

To perform such a scan, just specify your desired flags with`--scanflags` and specify FIN scan
(`-sF`) as the base type (choosing NULL or Xmas would
make no difference). [Example 5.14](https://nmap.org/book/scan-methods-custom-scanflags.html#scan-methods-ex-custom-psh-scan) demonstrates a PSH
scan against a Linux machine on a local network.

Example 5.14. A custom PSH scan

```
krad# nmap -sF --scanflags PSH  para

Starting Nmap ( https://nmap.org )
Nmap scan report for para (192.168.10.191)
(The 995 ports scanned but not shown below are in state: closed)
PORT     STATE         SERVICE
22/tcp   open|filtered ssh
53/tcp   open|filtered domain
111/tcp  open|filtered rpcbind
515/tcp  open|filtered printer
6000/tcp open|filtered X11
MAC Address: 00:60:1D:38:32:90 (Lucent Technologies)

Nmap done: 1 IP address (1 host up) scanned in 5.95 seconds

```

Because these scans all work the same way, I could keep just one
of `-sF`, `-sN`, and`-sX` options, letting users emulate the others with`--scanflags`. There are no plans to do this because
the shortcut options are easier to remember and use. You can still try the
emulated approach to show off your Nmap skills. Execute **nmap
-sF --scanflags FINPSHURG target** rather than the more
mundane **nmap -sX target**.

|                                                                    ![[Warning]](https://nmap.org/book/images/warning.png)                                                                     |Warning|
|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------|
|In my experience,<br/>needlessly complex Nmap command-lines don't impress girls. They usually<br/>respond with a condescending sneer, presumably recognizing<br/>that the command is redundant.|       |

[]()

---

[Prev](https://nmap.org/book/scan-methods-null-fin-xmas-scan.html)TCP FIN, NULL, and Xmas Scans (-sF, -sN, -sX)

[Up](https://nmap.org/book/scan-methods.html)Chapter 5. Port Scanning Techniques and Algorithms

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/scan-methods-ack-scan.html)TCP ACK Scan (-sA)