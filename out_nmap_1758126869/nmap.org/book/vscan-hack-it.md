---
title: "SOLUTION: Hack Version Detection to Suit Custom Needs, such as Open Proxy Detection | Nmap Network Scanning"
source_url: https://nmap.org/book/vscan-hack-it.html
fetched_at: 2025-09-17T16:42:31.056692+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 7. Service and Application Version Detection](https://nmap.org/book/vscan.html)
* SOLUTION: Hack Version Detection to Suit Custom Needs, such as Open Proxy Detection

[Prev](https://nmap.org/book/vscan-find-service-fast.html)

[Next](https://nmap.org/book/osdetect.html)

SOLUTION: Hack Version Detection to Suit Custom Needs, such as Open Proxy Detection
----------

### Problem ###

An important part of securing any network is identifying
dangerous hosts. Nmap's service detection system is a flexible,
reliable way to do this. It can help identify vulnerable versions of
software, find misconfigured servers, and more. But sometimes actually
trying to misuse services in ways the stock version scan doesn't dare
to is the best way to determine if they are actually
vulnerable.

Open proxies are servers that will blindly relay requests from untrusted
hosts to servers of their choosing. Running these inside a network can be
extremely dangerous for many reasons, as attackers can:

* Launch attacks that appear to come from your network

* Steal bandwidth or other network services from you

* Pretend to be an internal client to further escalate their privileges inside your organization

This provides good motivation for hacking version detection to specifically try to exploit open proxies. We could probably map out which
ports are proxies by using Nmap's normal proxy
match lines, but the best, and only real way to prove an application is
vulnerable is to actually exploit it yourself.

|       ![[Note]](https://nmap.org/book/images/note.png)        |Note|
|---------------------------------------------------------------|----|
|This solution was contributed by Nmap developer Doug Hoyte.[]()|    |

### Solution ###

The first thing we do is copy the `nmap-service-probes` file so we can
work on a temporary copy:

```
mkdir ~/proxydetect
cp /usr/local/share/nmap/nmap-service-probes ~/proxydetect

```

Next we want to temporarily force Nmap to use
our temporary file:

```
export NMAPDIR=$HOME/proxydetect

```

Now we need to add a probe and match line to the file, so open up
your favorite editor and place the following text into your copy of`nmap-service-probes`. A good place to put it is after all the match
lines in the NULL probe, but immediately before the next Probe line (GenericLines).

```
Probe TCP ProxyProbe q|GET https://insecure.org/ HTTP/1.1\r\nHost: insecure.org\r\n\r\n|
rarity 1
ports 1-65535
totalwaitms 20000
match proxy m|^HTTP/1.[01] 200 OK\r?\n.*TITLE>Insecure.O|s p/Open HTTP Proxy!!/

```

Now Nmap will actually try to request an
HTTP download from `insecure.org` by treating any
scanned ports as proxies. We will start to see the following in scans
of networks containing open proxies:

```
PORT   STATE SERVICE VERSION
80/tcp open  proxy   Open HTTP Proxy!!

```

### Discussion ###

The placement of our probe, the low `rarity` value, and
extensive `ports` range help ensure that our custom probe
is tried very soon into the service scan so that other probes like GetRequest
don't simply identify this as a proxy before we've had a chance to use our
active probe.

We also used a `totalwaitms` directive to makeNmap wait longer for this probe to time out. This
can be necessary because not only are we dealing with the latency and
unreliability of the connection between us and the proxy, but also the
latency and unreliability of the connection between the proxy and the server
containing the page we requested (`insecure.org`).

Keep in mind that many other protocols can be proxied in addition to HTTP.
Version detection will identify proxies for many of them including FTP, POP3, IMAP, and SMTP.
SOCKS proxies have special match lines that determine information on the authentication
options the proxy has configured. As we did in this solution, often we can use
version detection to tell whether such proxies are open or not by using custom
probes files. However, more complicated tests are probably best done with NSE scripts.

---

[Prev](https://nmap.org/book/vscan-find-service-fast.html)SOLUTION: Find All Servers Running an Insecure or Nonstandard Application Version

[Up](https://nmap.org/book/vscan.html)Chapter 7. Service and Application Version Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/osdetect.html)Chapter 8. Remote OS Detection