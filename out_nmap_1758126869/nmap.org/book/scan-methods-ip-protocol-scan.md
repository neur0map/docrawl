---
title: "IP Protocol Scan (-sO) | Nmap Network Scanning"
source_url: https://nmap.org/book/scan-methods-ip-protocol-scan.html
fetched_at: 2025-09-17T16:40:44.858450+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 5. Port Scanning Techniques and Algorithms](https://nmap.org/book/scan-methods.html)
* IP Protocol Scan (-sO)

[Prev](https://nmap.org/book/idlescan.html)

[Next](https://nmap.org/book/scan-methods-ftp-bounce-scan.html)

IP Protocol Scan (`-sO`)
----------

[]()[]()[]()

IP protocol scan allows you to determine which IP protocols
(TCP, ICMP, IGMP, etc.) are supported by target machines. This isn't
technically a port scan, since it cycles through IP protocol numbers
rather than TCP or UDP port numbers. Yet it still uses the`-p` option to select scanned protocol numbers, reports
its results within the normal port table format, and even uses the same
underlying scan engine as the true port scanning methods. So it is
close enough to a port scan that it belongs here.

Besides being useful in its own right, protocol scan
demonstrates the power of
open-source software.[]()While the fundamental
idea is pretty simple, I had not thought to add it nor received any
requests for such functionality. Then in the summer of 2000,
Gerhard Rieger[]()conceived the idea, wrote an excellent patch implementing it,
and sent it to the*nmap-hackers* mailing list.[]()I incorporated that
patch into the Nmap tree and released a new version the next day. Few
pieces of commercial software have users enthusiastic enough to design
and contribute their own improvements!

Protocol scan works in a similar fashion to UDP scan. Instead
of iterating through the port number field of a UDP packet, it sends
IP packet headers and iterates through the eight-bit IP protocol field.
The headers are usually empty, containing no data and not even the
proper header for the claimed protocol. An exception is made for
certain popular protocols (including TCP, UDP, and ICMP). Proper
protocol headers for those are included since some systems won't send
them otherwise and because Nmap already has functions to create them.
Instead of watching for ICMP port unreachable messages, protocol scan
is on the lookout for ICMP *protocol*unreachable messages.[]()[Table 5.8](https://nmap.org/book/scan-methods-ip-protocol-scan.html#scan-methods-tbl-protocol-scan-responses)shows how responses to the IP probes are mapped to port states.

Table 5.8. How Nmap interprets responses to an IP protocol probe

|                        Probe Response                         |                              Assigned State                               |
|---------------------------------------------------------------|---------------------------------------------------------------------------|
|         Any response in any protocol from target host         |  `open` (for protocol used by response, not necessarily probe protocol)   |
|       ICMP protocol unreachable error (type 3, code 2)        |                                 `closed`                                  |
|Other ICMP unreachable errors (type 3, code 1, 3, 9, 10, or 13)|`filtered` (though they prove ICMP is open if sent from the target machine)|
|       No response received (even after retransmissions)       |                              `open|filtered`                              |

Like open ports in the TCP or UDP protocols, every open protocol
is a potential exploitation vector. In addition, protocol scan
results help determine the purpose of a machine and what sort of
packet filtering is in place. End hosts usually have little more than
TCP, UDP, ICMP, and (sometimes) IGMP open, while routers often offer
much more, including routing-related protocols such as GRE and EGP.
Firewalls and VPN gateways may show encryption-related protocols such as IPsec and
SWIPE.

Like the ICMP port unreachable messages received during a UDP
scan, ICMP protocol unreachable messages are often
rate limited.[]()For example, no more than one ICMP destination unreachable response is
sent per second from a default Linux 2.4.20 box. Since there are only
256 possible protocol numbers, this is less of a problem than with a
65,536-port UDP scan. The suggestions in [the section called “Speeding Up UDP Scans”](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-udp-optimizing) apply to speeding up IP
protocol scans as well.

Protocol scan is used the same way as most other scan
techniques on the command line. Simply specify `-sO` in addition to whatever
general Nmap options please you. The normal port
(`-p`)[]()option is used to select protocol numbers. Or you can use`-F`[]()to scan all protocols listed in the`nmap-protocols`[]()database. By default, Nmap scans all 256 possible values.[Example 5.20](https://nmap.org/book/scan-methods-ip-protocol-scan.html#scan-methods-ex-protocol-scanme) shows
Ereet[]()scanning a router in Poland followed by a typical Linux box on my local
network.

Example 5.20. IP protocol scan of a router and a typical Linux 2.4 box

[]()

```
# nmap -sO 62.233.173.90 para

Starting Nmap ( https://nmap.org )
Nmap scan report for ntwklan-62-233-173-90.devs.futuro.pl (62.233.173.90)
Not shown: 240 closed ports
PROTOCOL STATE         SERVICE
1        open          icmp                    
4        open|filtered ip                      
6        open          tcp                     
8        open|filtered egp                     
9        open|filtered igp                     
17       filtered      udp                     
47       open|filtered gre                     
53       filtered      swipe                   
54       open|filtered narp                    
55       filtered      mobile                  
77       filtered      sun-nd                  
80       open|filtered iso-ip                  
88       open|filtered eigrp                   
89       open|filtered ospfigp                 
94       open|filtered ipip                    
103      filtered      pim                     

Nmap scan report for para (192.168.10.191)
Not shown: 252 closed ports
PROTOCOL STATE         SERVICE
1        open          icmp                    
2        open|filtered igmp                    
6        open          tcp                     
17       filtered      udp                     
MAC Address: 00:60:1D:38:32:90 (Lucent Technologies)

Nmap done: 2 IP addresses (2 hosts up) scanned in 458.04 seconds

```

[]()

---

[Prev](https://nmap.org/book/idlescan.html)TCP Idle Scan (-sI)

[Up](https://nmap.org/book/scan-methods.html)Chapter 5. Port Scanning Techniques and Algorithms

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/scan-methods-ftp-bounce-scan.html)TCP FTP Bounce Scan (-b)