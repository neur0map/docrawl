---
title: "Post-processors | Nmap Network Scanning"
source_url: https://nmap.org/book/vscan-post-processors.html
fetched_at: 2025-09-17T16:41:59.638293+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 7. Service and Application Version Detection](https://nmap.org/book/vscan.html)
* Post-processors

[Prev](https://nmap.org/book/vscan-technique-demo.html)

[Next](https://nmap.org/book/vscan-fileformat.html)

Post-processors
----------

[]()

Nmap is usually finished working on a port once it has deduced
the service and version information as demonstrated above. However,
there are certain services for which Nmap performs additional work.
The post-processors presently available are Nmap
Scripting Engine integration, RPC grinding, and SSL tunneling. Windows
SMB interrogation is under consideration.

### Nmap Scripting Engine Integration ###

[]()

The regular-expression based approach of version detection is
powerful, but it cannot recognize everything. Some services cannot be recognized by simply sending a standard probe and matching a pattern to the response. Some services require custom probe strings or a complex multi-step handshaking process. Others require more advanced processing than a regular expression to recognize a response. For example, the Skype v2 service was designed to be difficult to detect due to the risk that incumbent carriers (such as phone companies providing DSL lines) would consider them a competitor and degrade or block the service from their subscribers. The only way we could find to detect this service involved analyzing responses to two different probes. Similarly, we could recognize more SNMP services if we
tried a few hundred different community names by brute force. Neither
of these tasks are well suited to traditional Nmap version detection,
but both were accomplished with the[Chapter 9, *Nmap Scripting Engine*](https://nmap.org/book/nse.html). For these reasons, version detection now calls NSE by default to handle some tricky services, as described in [the section called “Version Detection Using NSE”](https://nmap.org/book/nse-vscan.html).

### RPC Grinding ###

[]()

SunRPC (Sun Remote Procedure Call) is a common Unix protocol
used to implement many services including NFS.
Nmap ships with an`nmap-rpc`[]()database of almost 600 RPC programs.
Many RPC services use high-numbered ports and/or the UDP transport
protocol, making them available through many poorly configured
firewalls. RPC programs (and the infrastructure libraries themselves)
also have a long history of serious remotely exploitable security holes.
So network administrators and security auditors often wish to learn more
about any RPC programs on their networks.

If the portmapper (rpcbind) service (UDP or TCP port 111) is[]()[]()available, RPC services can be enumerated with the Unix**rpcinfo**[]() command. [Example 7.5](https://nmap.org/book/vscan-post-processors.html#ex-version-detection-rpcinfo) demonstrates this against a default Solaris 9 server.

Example 7.5. Enumerating RPC services with rpcinfo

```
> rpcinfo -p ultra
   program vers proto   port
    100000    4   tcp    111  rpcbind
    100000    4   udp    111  rpcbind
    100232   10   udp  32777  sadmind
    100083    1   tcp  32775  ttdbserverd
    100221    1   tcp  32777  kcms_server
    100068    5   udp  32778  cmsd
    100229    1   tcp  32779  metad
    100230    1   tcp  32781  metamhd
    100242    1   tcp  32783  rpc.metamedd
    100001    4   udp  32780  rstatd
    100002    3   udp  32782  rusersd
    100002    3   tcp  32785  rusersd
    100008    1   udp  32784  walld
    100012    1   udp  32786  sprayd
    100011    1   udp  32788  rquotad
    100024    1   udp  32790  status
    100024    1   tcp  32787  status
    100133    1   udp  32790  nsm_addrand
    100133    1   tcp  32787  nsm_addrand
    [ Dozens of lines cut for brevity ]

```

This example shows that hosts frequently offer many RPC
services, which increases the probability that one is exploitable.
You should also notice that most of the services are on strange
high-numbered ports (which may change for any number of reasons) and
split between UDP and TCP transport protocols.

Because the RPC information is so sensitive, many administrators
try to obscure this information by blocking the portmapper port[]()(111). Unfortunately, this does not close the hole. Nmap can
determine all of the same information by directly communicating with open RPC
ports through the following three-step process.

1. The TCP and/or UDP port scan finds all of the open ports.

2. Version detection determines which of the open ports use the SunRPC protocol.

3. The RPC brute force engine determines the program
   identity of each RPC port by trying a *null command* against each of
   the 600 programs numbers in `nmap-rpc`. Most of
   the time Nmap guesses wrong and receives an error message
   stating that the requested program number is not listening on the
   port. Nmap continues trying each
   number in its list until success is returned for one of them.
   Nmap gives up in the unlikely event
   that it exhausts all of its known program numbers or if the port sends
   malformed responses that suggest it is not really
   RPC.

The RPC program identification probes are done in parallel, and
retransmissions are handled for UDP ports. This feature is
automatically activated whenever version detection finds any RPC
ports.[Example 7.6](https://nmap.org/book/vscan-post-processors.html#ex-version-detection-rpcscan) demonstrates direct RPC scanning done as part of version detection.

Example 7.6. Nmap direct RPC scan

[]()

```
# nmap -F -A -sSU ultra

Starting Nmap ( https://nmap.org )
Nmap scan report for ultra.nmap.org (192.168.0.50)
(The 2171 ports scanned but not shown below are in state: closed)
PORT      STATE SERVICE            VERSION
[A whole bunch of ports cut for brevity]
32776/tcp open  kcms_server        1 (rpc #100221)
32776/udp open  sadmind            10 (rpc #100232)
32777/tcp open  kcms_server        1 (rpc #100221)
32777/udp open  sadmind            10 (rpc #100232)
32778/tcp open  metad              1 (rpc #100229)
32778/udp open  cmsd               2-5 (rpc #100068)
32779/tcp open  metad              1 (rpc #100229)
32779/udp open  rstatd             2-4 (rpc #100001)
32780/tcp open  metamhd            1 (rpc #100230)
32780/udp open  rstatd             2-4 (rpc #100001)
32786/tcp open  status             1 (rpc #100024)
32786/udp open  sprayd             1 (rpc #100012)
32787/tcp open  status             1 (rpc #100024)
32787/udp open  rquotad            1 (rpc #100011)
Device type: general purpose
Running: Sun Solaris 9
OS details: Sun Solaris 9

Nmap finished: 1 IP address (1 host up) scanned in 252.701 seconds

```

[]()

### SSL Post-processor Notes ###

[]()

As discussed in the technique section,
Nmap has the ability to detect the SSL
encryption protocol and then launch an encrypted session through which
it executes normal version detection. As with the RPC grinder
discussed previously, the SSL post-processor is automatically executed
whenever an appropriate (SSL) port is detected. This is demonstrated by [Example 7.7](https://nmap.org/book/vscan-post-processors.html#ex-version-detection-ssl).

Example 7.7. Version scanning through SSL

[]()

```
$ nmap -Pn -sSV -T4 -F www.amazon.com

Starting Nmap ( https://nmap.org )
Nmap scan report for 207-171-184-16.amazon.com (207.171.184.16)
(The 1214 ports scanned but not shown below are in state: filtered)
PORT    STATE SERVICE  VERSION
80/tcp  open  http     Apache Stronghold httpd 2.4.2 (based on Apache 1.3.6)
443/tcp open  ssl/http Apache Stronghold httpd 2.4.2 (based on Apache 1.3.6)

Nmap finished: 1 IP address (1 host up) scanned in 35.038 seconds

```

Note that the version information is the same for each of the
two open ports, but the service is `http` on
port 80 and `ssl/http` on port 443. The
common case of HTTPS on port 443 is not hard-coded—Nmap should be able to detect SSL on any
port and determine the underlying protocol for any service that Nmap can
detect in clear-text. If Nmap had not detected the server listening
behind SSL, the service listed would be `ssl/unknown`. If Nmap
had not been built with SSL support, the service listed would have
simply been `ssl`. The version field would be blank in both
of these cases.

The SSL support for Nmap depends on
the free [OpenSSL library](http://www.openssl.org/)[](). It is not included in the Linux
RPM binaries, to avoid breaking systems which lack these libraries.
The Nmap source code distribution attempts to detect OpenSSL on
a system and link to it when available. See [Chapter 2, *Obtaining, Compiling, Installing, and Removing Nmap*](https://nmap.org/book/install.html) for details on customizing the build process to include or exclude OpenSSL.

---

[Prev](https://nmap.org/book/vscan-technique-demo.html)Technique Demonstrated

[Up](https://nmap.org/book/vscan.html)Chapter 7. Service and Application Version Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/vscan-fileformat.html)nmap-service-probes File Format