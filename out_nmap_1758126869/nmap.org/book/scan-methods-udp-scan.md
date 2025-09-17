---
title: "UDP Scan (-sU) | Nmap Network Scanning"
source_url: https://nmap.org/book/scan-methods-udp-scan.html
fetched_at: 2025-09-17T16:40:14.391145+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 5. Port Scanning Techniques and Algorithms](https://nmap.org/book/scan-methods.html)
* UDP Scan (-sU)

[Prev](https://nmap.org/book/scan-methods-connect-scan.html)

[Next](https://nmap.org/book/scan-methods-null-fin-xmas-scan.html)

UDP Scan (`-sU`)
----------

[]()[]()

While most popular services on the Internet run over the TCP
protocol, [UDP](http://www.rfc-editor.org/rfc/rfc768.txt) services
are widely deployed. DNS, SNMP, and DHCP
(registered ports 53, 161/162, and 67/68) are three of the most
common.[]()Because UDP scanning is generally slower and more difficult
than TCP, some security auditors ignore these ports. This is a mistake, as
exploitable UDP services are quite common and attackers certainly
don't ignore the whole protocol. Fortunately, Nmap can help inventory
UDP ports.

UDP scan is activated with the `-sU` option. It
can be combined with a TCP scan type such as SYN scan
(`-sS`) to check both protocols during the same
run.

UDP scan works by sending a UDP packet to every
targeted port. For most ports, this packet will be empty (no payload),
but for a few of the more common ports a protocol-specific payload will
be sent.[]()Based on the response, or lack thereof, the port is
assigned to one of four states, as shown in[Table 5.3](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-tbl-udp-scan-responses).

Table 5.3. How Nmap interprets responses to a UDP probe

|                        Probe Response                         |Assigned State |
|---------------------------------------------------------------|---------------|
|          Any UDP response from target port (unusual)          |    `open`     |
|       No response received (even after retransmissions)       |`open|filtered`|
|         ICMP port unreachable error (type 3, code 3)          |   `closed`    |
|Other ICMP unreachable errors (type 3, code 1, 2, 9, 10, or 13)|  `filtered`   |

The most curious element of this table may be the`open|filtered` state.[]()It is a symptom of the
biggest challenges with UDP scanning: open ports rarely respond to empty
probes. Those ports for which Nmap has a protocol-specific payload are
more likely to get a response and be marked `open`, but
for the rest, the target TCP/IP stack simply passes the empty packet up
to a listening application, which usually discards it
immediately as invalid. If ports in all other states would respond,
then open ports could all be deduced by elimination. Unfortunately,
firewalls and filtering devices are *also* known to
drop packets without responding. So when Nmap receives no response after
several attempts, it cannot determine whether the port is`open` or `filtered`. When Nmap was
released, filtering devices were rare enough that Nmap could (and did)
simply assume that the port was `open`. The Internet
is better guarded now, so Nmap changed in 2004 (version
3.70) to report non-responsive UDP ports as`open|filtered` instead. We can see that in [Example 5.4](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-ex-udpscan-felix), which shows Ereet scanning
a Linux box named Felix.

Example 5.4. UDP scan example

[]()

```
krad# nmap -sU -v felix

Starting Nmap ( https://nmap.org )
Nmap scan report for felix.nmap.org (192.168.0.42)
(The 997 ports scanned but not shown below are in state: closed)
PORT    STATE         SERVICE
53/udp  open|filtered domain
67/udp  open|filtered dhcpserver
111/udp open|filtered rpcbind
MAC Address: 00:02:E3:14:11:02 (Lite-on Communications)

Nmap done: 1 IP address (1 host up) scanned in 999.25 seconds

```

This scan of Felix demonstrates the`open|filtered` ambiguity issue as well as another
problem: UDP scanning can be *slow*. Scanning a
thousand ports took almost 17 minutes in this case due to ICMP response rate limiting performed by Felix and most other Linux systems. Nmap provides
ways to work around both problems, as described by the following two sections.

### Distinguishing Open from Filtered UDP Ports ###

In the case of the Felix scan, all but the three`open|filtered` ports were `closed`.
So the scan was still successful in narrowing down potentially open
ports to a handful. That is not always the case. [Example 5.5](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-ex-udpscan-scanme) shows a UDP scan against
the heavily filtered site Scanme.

Example 5.5. UDP scan example

[]()

```
krad# nmap -sU -T4 scanme.nmap.org

Starting Nmap ( https://nmap.org )
All 1000 scanned ports on scanme.nmap.org (64.13.134.52) are open|filtered

Nmap done: 1 IP address (1 host up) scanned in 5.50 seconds

```

In this case, the scan didn't narrow down the open ports at all.
All 1000 are `open|filtered`. A new strategy is
called for.

[Table 5.3, “How Nmap interprets responses to a UDP probe”](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-tbl-udp-scan-responses) shows
that the `open|filtered` state occurs when Nmap fails
to receive any responses from its UDP probes to a particular port.
Yet it also shows that, on rare occasions, the UDP service
listening on a port will respond in kind, proving that the port is
open. The reason these services don't respond often is that the empty
packets Nmap sends are considered invalid. Unfortunately, UDP services
generally define their own packet structure rather than adhering to
some common general format that Nmap could always send. An SNMP
packet looks completely different than a SunRPC, DHCP, or DNS request
packet.

To send the proper packet for every popular UDP service, Nmap
would need a large database defining their probe formats.
Fortunately, Nmap has that in the form of`nmap-service-probes`,[]()which is part of the service and
version detection subsystem described in [Chapter 7, *Service and Application Version Detection*](https://nmap.org/book/vscan.html).

[]()

When version scanning is enabled with `-sV` (or`-A`), it will send UDP probes to every`open|filtered` port (as well as known`open` ones). If any of the probes elicit a response from an `open|filtered` port, the state is
changed to `open`. The results of adding`-sV` to the Felix scan are shown in [Example 5.6](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-ex-udpscan-felix2).

Example 5.6. Improving Felix's UDP scan results with version detection

[]()

```
krad# nmap -sUV -F felix.nmap.org

Starting Nmap ( https://nmap.org )
Nmap scan report for felix.nmap.org (192.168.0.42)
Not shown: 997 closed ports
PORT    STATE         SERVICE    VERSION
53/udp  open          domain     ISC BIND 9.2.1
67/udp  open|filtered dhcpserver
111/udp open          rpcbind    2 (rpc #100000)
MAC Address: 00:02:E3:14:11:02 (Lite-on Communications)

Nmap done: 1 IP address (1 host up) scanned in 1037.57 seconds

```

This new scan shows that port 111 and 53 are definitely open.
The system isn't perfect though—port 67 is still`open|filtered`. In this particular case, the port
is open but Nmap does not have a working version probe for DHCP.
Another tough service is SNMP, which usually only responds when the
correct community string is given. Many devices are configured with
the community string `public`, but not all are.
While these results aren't perfect, learning the true state of two out
of three tested ports is still helpful.

After the success in disambiguating Felix results,
Ereet[]()turns his attention back to Scanme, which listed all ports as `open|filtered` last time. He tries again with version detection, as shown in [Example 5.7](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-ex-udpscan-scanme2).

Example 5.7. Improving Scanme's UDP scan results with version detection

[]()

```
krad# nmap -sUV -T4 scanme.nmap.org

Starting Nmap ( https://nmap.org )
Nmap scan report for scanme.nmap.org (64.13.134.52)
Not shown: 999 open|filtered ports
PORT   STATE SERVICE VERSION
53/udp open  domain  ISC BIND 9.3.4

Nmap done: 1 IP address (1 host up) scanned in 3691.89 seconds

```

|                                                                                                                                                                                                                                       ![[Tip]](https://nmap.org/book/images/tip.png)                                                                                                                                                                                                                                       |Tip|
|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---|
|While Ereet eventually found the open port, he made a mistake in not updating his Nmap version first. Nmap version 5.10BETA1 and newer have a payload system which sends proper service protocol requests to more than three dozen well known UDP ports if they are selected for port scanning or host discovery. While it isn't as comprehensive as version detection, it would have quickly identified the open port 53 in [Example 5.5](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-ex-udpscan-scanme).|   |

This result took an hour, versus five seconds for the previous
Scanme scan, but these results are actually useful. Ereet's smile
widens and eyes sparkle at this evidence of an open ISC BIND
nameserver on a machine he wants to compromise. That software has a long
history of security holes, so perhaps he can find a flaw in this
recent version.

Ereet will focus his UDP attacks on port 53 since it is
confirmed open, but he does not forget about the other 999 ports listed as`open|filtered`. As we witnessed with
the dhcpserver port on Felix, certain open UDP services can hide even
from Nmap version detection. He has also only scanned the default ports so
far, there are 64529 others that could possibly be open. For the
record, 53 is the only open UDP port on Scanme.

While this version detection technique is the only way for Nmap
to automatically disambiguate `open|filtered` ports,
there are a couple tricks that can be tried manually. Sometimes a
specialized traceroute can help. You could do a traceroute against a
known-open TCP or UDP port with Nmap or a tool such as
Nping.[]()Then try the same against
the questionable UDP port. Differences in hop counts can
differentiate open from filtered ports. Ereet attempts this against
Scanme in [Example 5.8](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-ex-rtt-trick). The first command does a UDP traceroute against
known-open port 53. The second command does the same thing against
presumed-closed port 54. The first few hops have been omitted to save space.

Example 5.8. Attempting to disambiguate UDP ports with TTL discrepancies

```
krad# nping --udp --traceroute -c 13 -p 53 scanme.nmap.org

Starting Nping ( https://nmap.org/nping )
SENT (7.0370s) UDP 192.168.0.21:53 > 64.13.134.52:53 ttl=8 id=4826 iplen=28
RCVD (7.1010s) ICMP 4.69.134.222 > 192.168.0.21 TTL=0 during transit (type=11/code=0) ttl=248 id=38454 iplen=56
SENT (8.0400s) UDP 192.168.0.21:53 > 64.13.134.52:53 ttl=9 id=38166 iplen=28
RCVD (8.1050s) ICMP 4.68.18.204 > 192.168.0.21 TTL=0 during transit (type=11/code=0) ttl=247 id=39583 iplen=56
SENT (9.0420s) UDP 192.168.0.21:53 > 64.13.134.52:53 ttl=10 id=6788 iplen=28
RCVD (9.1080s) ICMP 4.59.4.78 > 192.168.0.21 TTL=0 during transit (type=11/code=0) ttl=246 id=59897 iplen=56
SENT (10.0440s) UDP 192.168.0.21:53 > 64.13.134.52:53 ttl=11 id=366 iplen=28
RCVD (10.1100s) ICMP 69.36.239.221 > 192.168.0.21 TTL=0 during transit (type=11/code=0) ttl=243 id=42710 iplen=56
SENT (11.0470s) UDP 192.168.0.21:53 > 64.13.134.52:53 ttl=12 id=63478 iplen=28
SENT (12.0490s) UDP 192.168.0.21:53 > 64.13.134.52:53 ttl=13 id=56653 iplen=28

Max rtt: 73.003ms | Min rtt: 0.540ms | Avg rtt: 48.731ms
Raw packets sent: 13 (364B) | Rcvd: 10 (560B) | Lost: 3 (23.08%)
Tx time: 12.02836s | Tx bytes/s: 30.26 | Tx pkts/s: 1.08
Rx time: 13.02994s | Rx bytes/s: 42.98 | Rx pkts/s: 0.77
Nping done: 1 IP address pinged in 13.05 seconds

krad# nping --udp --traceroute -c 13 -p 54 scanme.nmap.org

Starting Nping ( https://nmap.org/nping )
SENT (7.0370s) UDP 192.168.0.21:53 > 64.13.134.52:54 ttl=8 id=56481 iplen=28
RCVD (7.1130s) ICMP 4.69.134.214 > 192.168.0.21 TTL=0 during transit (type=11/code=0) ttl=248 id=22437 iplen=56
SENT (8.0400s) UDP 192.168.0.21:53 > 64.13.134.52:54 ttl=9 id=23264 iplen=28
RCVD (8.1060s) ICMP 4.68.18.76 > 192.168.0.21 TTL=0 during transit (type=11/code=0) ttl=247 id=50214 iplen=56
SENT (9.0430s) UDP 192.168.0.21:53 > 64.13.134.52:54 ttl=10 id=9101 iplen=28
RCVD (9.1070s) ICMP 4.59.4.78 > 192.168.0.21 TTL=0 during transit (type=11/code=0) ttl=246 id=880 iplen=56
SENT (10.0450s) UDP 192.168.0.21:53 > 64.13.134.52:54 ttl=11 id=35344 iplen=28
RCVD (10.1110s) ICMP 69.36.239.221 > 192.168.0.21 TTL=0 during transit (type=11/code=0) ttl=243 id=44617 iplen=56
SENT (11.0470s) UDP 192.168.0.21:53 > 64.13.134.52:54 ttl=12 id=53857 iplen=28
SENT (12.0490s) UDP 192.168.0.21:53 > 64.13.134.52:54 ttl=13 id=986 iplen=28

Max rtt: 76.488ms | Min rtt: 0.546ms | Avg rtt: 48.480ms
Raw packets sent: 13 (364B) | Rcvd: 11 (616B) | Lost: 2 (15.38%)
Tx time: 12.02908s | Tx bytes/s: 30.26 | Tx pkts/s: 1.08
Rx time: 13.03165s | Rx bytes/s: 47.27 | Rx pkts/s: 0.84
Nping done: 1 IP address pinged in 13.05 seconds

```

In this example,
Ereet[]()was only able to reach hop eleven of both
the open and closed ports. So these results can't be used to
distinguish port states against this host. It was worth a try, and
does work in a significant number of cases. It is more likely to work
in situations where the screening
firewall[]()is at least a hop or two before the target host. Scanme, on the other
hand, is running its own Linux
iptables[]()host-based firewall. So there is no difference in hop count
between filtered and open ports.

Another technique is to try application-specific tools against
common ports. For example, a brute force SNMP community string
cracker could be tried against port 161. As Nmap's version detection
probe database grows, the need to augment its results with external
specialized tools is reduced. They will still be useful for special
cases, such as SNMP devices with a custom community string.

### Speeding Up UDP Scans ###

[]()

The other big challenge with UDP scanning is doing it quickly.
Open and filtered ports rarely send any response, leaving Nmap to time
out and then conduct retransmissions just in case the probe or
response were lost. Closed ports are often an even bigger problem.
They usually send back an ICMP port unreachable error. But unlike the
RST packets sent by closed TCP ports in response to a SYN or connect
scan, many hosts rate limit ICMP port unreachable messages by default.
Linux and Solaris are particularly strict about
this.[]()For example, the
Linux 2.4.20 kernel on Felix limits destination unreachable messages to one per
second (in `net/ipv4/icmp.c`). This explains why
the scan in [Example 5.4, “UDP scan example”](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-ex-udpscan-felix) is so slow.

Nmap detects rate limiting and slows down accordingly to avoid
flooding the network with useless packets that the target machine will
drop. Unfortunately, a Linux-style limit of one packet per second
makes a 65,536-port scan take more than 18 hours. Here are some
suggestions for improving UDP scan performance. Also read [Chapter 6, *Optimizing Nmap Performance*](https://nmap.org/book/performance.html) for more detailed discussion and general advice.

Increase host parallelism

If Nmap receives just one port unreachable error from a single target host per second, it could receive 100/second just by scanning 100 such hosts at once. Implement this by passing a large value (such as 100) to [`--min-hostgroup`.]()

[Scan popular ports first]()

[Very few UDP port numbers are commonly used. A scan of the most common 100 UDP ports (using the `-F` option) will finish quickly. You can then investigate those results while you launch a multi-day 65K-port sweep of the network in the background.]()

[]()[Add `--version-intensity 0` to version detection scans]()

[As mentioned in the previous section, version detection (`-sV`) is often needed to differentiate open from filtered UDP ports. Version detection is relatively slow since it involves sending a large number of application protocol-specific probes to every `open` or `open|filtered` port found on the target machines. Specifying `--version-intensity 0` directs Nmap to try only the probes most likely to be effective against a given port number. It does this by using data from the `nmap-service-probes` file. The performance impact of this option is substantial, as will be demonstrated later in this section.]()

[Scan from behind the firewall]()

[As with TCP, packet filters can slow down scans dramatically. Many modern firewalls make setting packet rate limits easy. If you can bypass that problem by launching the scan from behind the firewall rather than across it, do so.]()

[]()[ Use `--host-timeout` to skip slow hosts]()

[ICMP-rate-limited hosts can take orders of magnitude more time to scan than those that respond to every probe with a quick destination unreachable packet. Specifying a maximum scan time (such as `15m` for 15 minutes) causes Nmap to give up on individual hosts if it hasn't completed scanning them in that much time. This allows you to scan all of the responsive hosts quickly. You can then work on the slow hosts in the background.]()

[]()[ Use `-v` and chill out]()

[With verbosity (`-v`) enabled, Nmap provides estimated time for scan completion of each host. There is no need to watch it closely. Get some sleep, head to your favorite pub, read a book, finish other work, or otherwise amuse yourself while Nmap tirelessly scans on your behalf. ]()

[A perfect example of the need to optimize UDP scans is]()[Example 5.7, “Improving Scanme's UDP scan results with version detection”](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-ex-udpscan-scanme2). The scan obtained
the desired data, but it took more than an hour to scan this one host!
In [Example 5.9](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-ex-udpscan-scanme3),
Ereet runs that scan
again. This time he adds the `-F --version-intensity 0`options and the hour long scan is reduced to 13 seconds! Yet the same
key information (an ISC Bind daemon running on port 53) is
detected.

Example 5.9. Optimizing UDP Scan Time

[]()[]()

```
krad# nmap -sUV -T4 -F --version-intensity 0 scanme.nmap.org

Starting Nmap ( https://nmap.org )
Nmap scan report for scanme.nmap.org (64.13.134.52)
Not shown: 99 open|filtered ports
PORT   STATE SERVICE VERSION
53/udp open  domain  ISC BIND 9.3.4

Nmap done: 1 IP address (1 host up) scanned in 12.92 seconds

```

[]()

---

[Prev](https://nmap.org/book/scan-methods-connect-scan.html)TCP Connect Scan (-sT)

[Up](https://nmap.org/book/scan-methods.html)Chapter 5. Port Scanning Techniques and Algorithms

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/scan-methods-null-fin-xmas-scan.html)TCP FIN, NULL, and Xmas Scans (-sF, -sN, -sX)