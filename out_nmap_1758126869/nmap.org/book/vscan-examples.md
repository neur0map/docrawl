---
title: "Usage and Examples | Nmap Network Scanning"
source_url: https://nmap.org/book/vscan-examples.html
fetched_at: 2025-09-17T16:41:43.384699+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 7. Service and Application Version Detection](https://nmap.org/book/vscan.html)
* Usage and Examples

[Prev](https://nmap.org/book/vscan.html)

[Next](https://nmap.org/book/vscan-technique.html)

Usage and Examples
----------

[]()Before delving into the technical details of how version detection is implemented,
here are some examples demonstrating its usage and capabilities. To enable version detection, just add `-sV` to[]()whatever Nmap flags you normally use. Or use the `-A` option,[]()which turns on version detection and other*A*dvanced and *A*ggressive features later. It is really
that simple, as shown in [Example 7.2](https://nmap.org/book/vscan-examples.html#ex-version-detection-scan2).

Example 7.2. Version detection against www.microsoft.com

[]()

```
# nmap -A -T4 -F www.microsoft.com

Starting Nmap ( https://nmap.org )
Nmap scan report for 80.67.68.30
(The 1208 ports scanned but not shown below are in state: closed)
PORT    STATE    SERVICE     VERSION
22/tcp  open     ssh         Akamai-I SSH (protocol 1.5)
80/tcp  open     http        AkamaiGHost (Akamai's HTTP Acceleration service)
443/tcp open     ssl/http    AkamaiGHost (Akamai's HTTP Acceleration service)
Device type: general purpose
Running: Linux 2.1.X|2.2.X
OS details: Linux 2.1.19 - 2.2.25

Nmap finished: 1 IP address (1 host up) scanned in 19.223 seconds

```

This preceding scan demonstrates a couple things. First of all,
it is gratifying to see www.Microsoft.Com served off one of Akamai's
Linux boxes. More relevant to this chapter is that the listed service for
port 443 is `ssl/http`. That means that service detection first
discovered that the port was SSL, then it loaded up
OpenSSL and[]()performed service detection again through SSL connections to discover
a web server running AkamiGHost behind the encryption. Recall that `-T4` causes Nmap to go faster (more aggressive
timing) and `-F` tells Nmap to scan only ports registered in `nmap-services`.

[Example 7.3](https://nmap.org/book/vscan-examples.html#ex-version-detection-scan3) is a longer and more diverse example.

Example 7.3. Complex version detection

[]()

```
# nmap -A -T4 localhost

Starting Nmap ( https://nmap.org )
Nmap scan report for felix (127.0.0.1)
(The 1640 ports scanned but not shown below are in state: closed)
PORT     STATE SERVICE    VERSION
21/tcp   open  ftp        WU-FTPD wu-2.6.1-20
22/tcp   open  ssh        OpenSSH 3.1p1 (protocol 1.99)
53/tcp   open  domain     ISC BIND 9.2.1
79/tcp   open  finger     Linux fingerd
111/tcp  open  rpcbind    2 (rpc #100000)
443/tcp  open  ssl/http   Apache httpd 2.0.39 ((Unix) mod_perl/1.99_04-dev)
515/tcp  open  printer
631/tcp  open  ipp        CUPS 1.1
953/tcp  open  rndc?
5000/tcp open  ssl/ftp    WU-FTPD wu-2.6.1-20
5001/tcp open  ssl/ssh    OpenSSH 3.1p1 (protocol 1.99)
5002/tcp open  ssl/domain ISC BIND 9.2.1
5003/tcp open  ssl/finger Linux fingerd
6000/tcp open  X11        (access denied)
8000/tcp open  http-proxy Junkbuster webproxy
8080/tcp open  http       Apache httpd 2.0.39 ((Unix) mod_perl/1.99_04-dev)
8081/tcp open  http       Apache httpd 2.0.39 ((Unix) mod_perl/1.99_04-dev)
Device type: general purpose
Running: Linux 2.4.X|2.5.X
OS details: Linux Kernel 2.4.0 - 2.5.20

Nmap finished: 1 IP address (1 host up) scanned in 42.494 seconds

```

You can see here the way RPC services are treated, with the
brute-force RPC scanner[]()being used to determine that port 111 is
rpcbind[]()version 2. You may also notice that port 515 gives the service as`printer`, but that version field is empty.
Nmap determined the service name by probing, but was not able to
determine anything else. On the other hand, port 953 gives the
service as “`rndc?`”. The question mark tells us that Nmap was not even
able to determine the service name through probing. As a fallback,
rndc is mentioned because that has port 953 registered in`nmap-services`.[]()Unfortunately, none of
Nmap's probes elicited any sort of response from rndc. If they had,
Nmap would have printed a service fingerprint and a submission URL so
that it could be recognized in the next version. As it is, Nmap
requires a special
probe. One might even be available by the time you
read this. [the section called “Community Contributions”](https://nmap.org/book/vscan-community.html) provides details on writing your own probes.

It is also worth noting that some services provide much
more[]()information than just the version number. Examples above include
whether X11 permits connections, the SSH protocol number, and theApache module versions list. Some of theApache modules even had to be cut from the
output to fit on this page.

A few early reviewers questioned the sanity of running services
such as SSH and finger over SSL. This was actually just fun with[stunnel](http://www.stunnel.org/), in part to ensure that parallel SSL scans actually work.

[]()

---

[Prev](https://nmap.org/book/vscan.html)Chapter 7. Service and Application Version Detection

[Up](https://nmap.org/book/vscan.html)Chapter 7. Service and Application Version Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/vscan-technique.html)Technique Described