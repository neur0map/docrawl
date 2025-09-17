---
title: "Technique Demonstrated | Nmap Network Scanning"
source_url: https://nmap.org/book/vscan-technique-demo.html
fetched_at: 2025-09-17T16:41:59.807296+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 7. Service and Application Version Detection](https://nmap.org/book/vscan.html)
* Technique Demonstrated

[Prev](https://nmap.org/book/vscan-technique.html)

[Next](https://nmap.org/book/vscan-post-processors.html)

Technique Demonstrated
----------

If the English description above is not clear enough, you can
see for yourself how it works by adding the`--version-trace`[]() (and usually `-d`(debugging)) options to your Nmap command
line. This shows all the connection and data read/write activity of the
service scan. An annotated real-world example follows.

[]()[]()[]()

```
# nmap -sSV -T4 -F -d --version-trace insecure.org

Starting Nmap ( https://nmap.org )
Host insecure.org (205.217.153.53) appears to be up ... good.
Initiating SYN Stealth Scan against insecure.org (205.217.153.53) at 19:53
Initiating service scan against 4 services on 1 host at 19:53

```

The SYN scan has found 4 open ports—now we are beginning a
service scan against each of them in parallel. We start with a TCP
connection for the NULL probe:

```
Starting probes against new service: 205.217.153.53:22 (tcp)
NSOCK (2.0750s) TCP connection requested to 205.217.153.53:22 (IOD #1) EID 8
Starting probes against new service: 205.217.153.53:25 (tcp)
NSOCK (2.0770s) TCP connection requested to 205.217.153.53:25 (IOD #2) EID 16
Starting probes against new service: 205.217.153.53:53 (tcp)
NSOCK (2.0830s) TCP connection requested to 205.217.153.53:53 (IOD #3) EID 24
Starting probes against new service: 205.217.153.53:80 (tcp)
NSOCK (2.0860s) TCP connection requested to 205.217.153.53:80 (IOD #4) EID 32
NSOCK (2.0870s) Callback: CONNECT SUCCESS for EID 32 [205.217.153.53:80]
NSOCK (2.0870s) Read request from IOD #4 [205.217.153.53:80]
                (timeout: 5000ms) EID 42
NSOCK (2.0870s) Callback: CONNECT SUCCESS for EID 24 [205.217.153.53:53]
NSOCK (2.0870s) Read request from IOD #3 [205.217.153.53:53]
                (timeout: 5000ms) EID 50
NSOCK (2.0870s) Callback: CONNECT SUCCESS for EID 16 [205.217.153.53:25]
NSOCK (2.0870s) Read request from IOD #2 [205.217.153.53:25]
                (timeout: 5000ms) EID 58
NSOCK (2.0870s) Callback: CONNECT SUCCESS for EID 8 [205.217.153.53:22]
NSOCK (2.0870s) Read request from IOD #1 [205.217.153.53:22]
                (timeout: 5000ms) EID 66

```

At this point, NULL probe connections have
successfully been made to all four services. It starts at 2 seconds
because that is how long the ping and SYN scans took.

```
NSOCK (2.0880s) Callback: READ SUCCESS for EID 66 [205.217.153.53:22]
                          (23 bytes): SSH-1.99-OpenSSH_3.1p1.
Service scan match: 205.217.153.53:22 is ssh.
                    Version: |OpenSSH|3.1p1|protocol 1.99|

```

SSH was nice enough to fully identify itself immediately upon
connection as OpenSSH 3.1p1. One down, three to go.

```
NSOCK (2.0880s) Callback: READ SUCCESS for EID 58 [205.217.153.53:25]
                          (27 bytes): 220 core.lnxnet.net ESMTP..
Service scan soft match: 205.217.153.53:25 is smtp

```

The mail server on port 25 also gave us a useful banner. We do
not know what type of mail server it is, but starting with `220` and
including the word `ESMTP` tells us it is a mail (SMTP) server. So
Nmap softmatches `smtp`, meaning that only probes able to match SMTP
servers are tried from now on. Note that non-printable characters are
represented by dots—so the “`..`” after ESMTP is really the “`\r\n`”line termination sequence.

```
NSOCK (2.0880s) Read request from IOD #2 [205.217.153.53:25]
                (timeout: 4996ms) EID 74
NSOCK (7.0880s) Callback: READ TIMEOUT for EID 74 [205.217.153.53:25]
NSOCK (7.0880s) Write request for 6 bytes to IOD #2 EID 83
                [205.217.153.53:25]: HELP..
NSOCK (7.0880s) Read request from IOD #2 [205.217.153.53:25]
                (timeout: 5000ms) EID 90

```

Nmap listens a little longer on the SMTP connection, just in
case the server has more to say. The read request times out after five
seconds. Nmap then finds the next probe which is registered to port
25 and has SMTP signatures. That probe simply consists of`HELP\r\n`, which Nmap writes into the
connection.

```
NSOCK (7.0880s) Callback: READ TIMEOUT for EID 50 [205.217.153.53:53]
NSOCK (7.0880s) Write request for 32 bytes to IOD #3 EID 99
                [205.217.153.53:53]: ...............version.bind.....
NSOCK (7.0880s) Read request from IOD #3 [205.217.153.53:53]
                (timeout: 5000ms) EID 106

```

The DNS server on port 53 does not return anything at all. The
first probe registered to port 53 in`nmap-service-probes` is DNSVersionBindReq, which
queries a DNS server for its version number. This is sent onto the
wire.

```
NSOCK (7.0880s) Callback: READ TIMEOUT for EID 42 [205.217.153.53:80]
NSOCK (7.0880s) Write request for 18 bytes to IOD #4 EID 115
                [205.217.153.53:80]: GET / HTTP/1.0....
NSOCK (7.0880s) Read request from IOD #4 [205.217.153.53:80]
                (timeout: 5000ms) EID 122

```

The port 80 NULL probe also failed to return any data. An HTTP
GET request is sent, since that probe is registered to port 80.

```
NSOCK (7.0920s) Callback: READ SUCCESS for EID 122
                [205.217.153.53:80] [EOF](15858 bytes)
Service scan match: insecure.org (205.217.153.53):80 is http.  
                    Version: |Apache httpd|2.0.39|(Unix) mod_perl/1.99_07-dev..

```

Apache returned a huge (15KB) response, so it is not printed.
That response provided detailed configuration information, which Nmap
picks out of the response. There are no other probes registered for
port 80. So if this had failed, Nmap would have tried the first TCP
probe in `nmap-service-probes`. That probe simply sends blank lines
(“`\r\n\r\n`”). A new connection would have been made in case the GET
probe confused the service.

```
NSOCK (7.0920s) Callback: READ SUCCESS for EID 106 [205.217.153.53:53]
                        (50 bytes): .0.........version.bind.......9.2.1
Service scan match: insecure.org (205.217.153.53):53 is domain.
                    Version: |ISC BIND|9.2.1||

```

Port 53 responded to our DNS version request. Most of the
response (as with the probe) is binary, but you can clearly see the
version 9.2.1 there. If this probe had failed, the next probe
registered to port 53 is a DNS server status request (14 bytes:`\0\x0C\0\0\x10\0\0\0\0\0\0\0\0\0`). Having this
backup probe helps because many more servers respond to a status
request than a version number request.

```
NSOCK (7.0920s) Callback: READ SUCCESS for EID 90 [205.217.153.53:25]
                (55 bytes): 214 qmail home page: http...
Service scan match: insecure.org (205.217.153.53):25 is smtp.
                    Version: |qmail smtpd|||

```

Port 25 gives a very helpful response to the
Help probe. Other SMTP servers such as Postfix,
Courier, and Exim can often be identified by this probe as well. If
the response did not match, Nmap would have given up on this service
because it had already softmatched `smtp` and there are no more SMTP
probes in `nmap-service-probes`.

```
The service scan took 5 seconds to scan 4 services on 1 host.

```

This service scan run went pretty well. No service required more than
one connection. It took five seconds because Qmail and Apache hit the
five-second NULL probe timeout before Nmap sent the first real probes.
Here is the reward for these efforts:

```
Interesting ports on insecure.org (205.217.153.53):
(The 1212 ports scanned but not shown below are in state: closed)
PORT   STATE SERVICE VERSION
22/tcp open  ssh     OpenSSH 3.1p1 (protocol 1.99)
25/tcp open  smtp    qmail smtpd
53/tcp open  domain  ISC BIND 9.2.1
80/tcp open  http    Apache httpd 2.0.39 ((Unix) mod_perl/1.99_07-dev)

Nmap finished: 1 IP address (1 host up) scanned in 7.104 seconds

```

---

[Prev](https://nmap.org/book/vscan-technique.html)Technique Described

[Up](https://nmap.org/book/vscan.html)Chapter 7. Service and Application Version Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/vscan-post-processors.html)Post-processors