---
title: "SOLUTION: Scan a Large Network for a Certain Open TCP Port | Nmap Network Scanning"
source_url: https://nmap.org/book/solution-find-open-port.html
fetched_at: 2025-09-17T16:39:47.810471+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 4. Port Scanning Overview](https://nmap.org/book/port-scanning.html)
* SOLUTION: Scan a Large Network for a Certain Open TCP Port

[Prev](https://nmap.org/book/port-scanning-ipv6.html)

[Next](https://nmap.org/book/scan-methods.html)

SOLUTION: Scan a Large Network for a Certain Open TCP Port
----------

### Problem ###

You wish to quickly find all machines on a network that have a
certain TCP port open. For example, after a new Microsoft IIS
vulnerability is found, you might want to scan for all machines with
TCP port 80 open and ensure that they aren't running a vulnerable
version of that software. Or if you investigate a compromised box and
find that the attacker left a backdoor running on port 31337, scanning
your whole network for that port might quickly identify other
compromised systems. A full (all ports) scan would be done later.

### Solution ###

The straightforward way is to run:

**nmap -Pn -p*`<portnumber>`* -oG*`<logfilename.gnmap>`* *`<target networks>`***

Here is a concrete example of searching 4096 IPs for web servers
(port 80 open):

**nmap -Pn -p80 -oG logs/pb-port80scan-%D.gnmap 216.163.128.0/20**

The “%D” in the filename is replaced with the numeric date on which the scan was run (e.g. “090107” on September 1, 2007). While this scan command works, a little effort choosing appropriate timing
values for the network being scanned reduces scan time
substantially. The scan above took 1,236 seconds, while the optimized
version below provided the same results in 869 seconds:

[]()[]()[]()**nmap -T4 -Pn -p80 --max-rtt-timeout 200ms --initial-rtt-timeout 150ms
--min-hostgroup 512 -oG logs/pb-port80scan2-%D.gnmap
216.163.128.0/20**

And much of that time is spent doing
reverse-DNS[]()resolution.
Excluding that by adding `-n` to the command-line above
reduces the 4096-host scan time to 193 seconds. Being patient for three
minutes is far easier than for the 21 minutes taken before.

The commands above store grepable-format results in the specified file. A simple egrep command will then find the machines with port 80 open:

**egrep '[^0-9]80/open' logs/pb-port80scan2-\*.gnmap**

The egrep pattern is preceded with [^0-9] to avoid bogus
matching ports such as 3180. Of course that can't happen since we are
only scanning port 80, but it is a good practice to remember for
many-port scans. If you only want the IP addresses and nothing else,
pipe the egrep output to **awk
'{print $2}'**.

### Discussion ###

Sometimes a story is the best way to understand decisions, such as how I decided upon the command lines in the solution section.
I was bored at home, and started exploring the network of a popular magazine named*Playboy*[](). Their main site includes a huge trove of images, but most are locked away behind a paid subscription authentication system. I was curious as to whether I could find any other systems on their network which offer up images for free. I figured that they might have staging or development servers which rely on obscurity rather than password authentication. While such servers could theoretically listen on any port number, the most likely is TCP port 80. So I decide to scan their whole network for that open port as quickly as possible.

The first step is determining which IP addresses to scan. I[]()perform a whois search of the
American Registry for Internet Numbers (ARIN)[]()for organizations named Playboy. The results are shown in[Example 4.5](https://nmap.org/book/solution-find-open-port.html#port-scanning-ex-whois-playboy).

Example 4.5. Discovering Playboy's IP space

```
core~> whois -h whois.arin.net n playboy
[Querying whois.arin.net]
[whois.arin.net]

OrgName:    Playboy
OrgID:      PLAYBO
Address:    680 N. Lake Shore Drive
City:       Chicago
StateProv:  IL
PostalCode: 60611
Country:    US

NetRange:   216.163.128.0 - 216.163.143.255
CIDR:       216.163.128.0/20
NetName:    PLAYBOY-BLK-1
NetHandle:  NET-216-163-128-0-1
Parent:     NET-216-0-0-0-0
NetType:    Direct Assignment
NameServer: NS1-CHI.PLAYBOY.COM
NameServer: NS2-CHI.PLAYBOY.COM
[...]

```

This shows 4096 IPs (the net range 216.163.128.0/20) registered
to Playboy. Using techniques discussed in [the section called “Finding an Organization's IP Addresses”](https://nmap.org/book/host-discovery-find-ips.html) I could have found many more
netblocks they control, but 4096 IPs are
sufficient for this example.

Next I want to estimate
latency[]()to these machines, so that Nmap
will know what to expect. This isn't required, but feeding Nmap
appropriate timing values can speed it up. This is particularly true
for single-port `-Pn` scans, such as this one. Nmap
does not receive enough responses from each host to accurately
estimate latency and packet drop rate, so I will help it out on the
command line. My first thought is to ping their main web server, as
shown in [Example 4.6](https://nmap.org/book/solution-find-open-port.html#port-scanning-ex-www-playboy-ping).

Example 4.6. Pinging Playboy's web server for a latency estimate

[]()

```
# ping -c5 www.playboy.com
PING www.phat.playboy.com (209.247.228.201) from 205.217.153.56
64 bytes from free-chi.playboy.com (209.247.228.201): icmp_seq=1 time=57.5 ms
64 bytes from free-chi.playboy.com (209.247.228.201): icmp_seq=2 time=56.7 ms
64 bytes from free-chi.playboy.com (209.247.228.201): icmp_seq=3 time=56.9 ms
64 bytes from free-chi.playboy.com (209.247.228.201): icmp_seq=4 time=57.0 ms
64 bytes from free-chi.playboy.com (209.247.228.201): icmp_seq=5 time=56.6 ms

--- www.phat.playboy.com ping statistics ---
5 packets transmitted, 5 received, 0% loss, time 4047ms
rtt min/avg/max/mdev = 56.652/57.004/57.522/0.333 ms

```

The maximum round trip time is 58 milliseconds. Unfortunately,
this IP address (209.247.228.201) is not within the 216.163.128.0/20
netblock I wish to scan. I would normally add this new netblock to
the target list, but have already decided to limit my scan to the
original 4096 IPs. These times are probably perfectly fine to use,
but finding actual values from IPs on the target network would be even
better. I usedig[]()to obtain Playboy's
public DNS records from a nameserver shown in the previous whois
query. The output is shown in [Example 4.7](https://nmap.org/book/solution-find-open-port.html#port-scanning-ex-www-playboy-dig).

Example 4.7. Digging through Playboy's DNS records[]()

```
core~> dig @ns1-chi.playboy.com playboy.com. any
; <<>> DiG 8.3 <<>> @ns1-chi.playboy.com playboy.com. any
[...]
;; ANSWER SECTION:
playboy.com.            1D IN A         209.247.228.201
playboy.com.            1D IN MX        10 mx.la.playboy.com.
playboy.com.            1D IN MX        5 mx.chi.playboy.com.
playboy.com.            1D IN NS        ns15.customer.level3.net.
playboy.com.            1D IN NS        ns21.customer.level3.net.
playboy.com.            1D IN NS        ns29.customer.level3.net.
playboy.com.            1D IN NS        ns1-chi.playboy.com.
playboy.com.            1D IN NS        ns2-chi.playboy.com.
playboy.com.            1D IN SOA       ns1-chi.playboy.com. dns.playboy.com. (
                                        2004092010      ; serial
                                        12H             ; refresh
                                        2h30m           ; retry
                                        2w1d            ; expiry
                                        1D )            ; minimum

;; ADDITIONAL SECTION:
mx.chi.playboy.com.     1D IN A         216.163.143.4
mx.la.playboy.com.      1D IN A         216.163.128.15
ns1-chi.playboy.com.    1D IN A         209.247.228.135
ns2-chi.playboy.com.    1D IN A         64.202.105.36

;; Total query time: 107 msec

```

The DNS query reveals two MX (mail) servers within the target 216.163.128.0/20 netblock. Since the names `mx.chi` and `mx.la` imply that they are in different regions (Chicago and Los Angeles), I decide to test them both for latency. The **ping** results are shown in [Example 4.8](https://nmap.org/book/solution-find-open-port.html#port-scanning-ex-www-playboy-mxping).

Example 4.8. Pinging the MX servers

```
core~> ping -c5 mx.chi.playboy.com
PING mx.chi.playboy.com (216.163.143.4) 56(84) bytes of data.

--- mx.chi.playboy.com ping statistics ---
5 packets transmitted, 0 received, 100% packet loss, time 4000ms

core~> ping -c5 mx.la.playboy.com
PING mx.la.playboy.com (216.163.128.15) 56(84) bytes of data.

--- mx.la.playboy.com ping statistics ---
5 packets transmitted, 0 received, 100% packet loss, time 4011ms

```

Well, that attempt was a miserable failure! The hosts seem to
be blocking ICMP ping packets. Since they are mail servers, they must
have TCP port 25 open, so I try again using [hping2](http://www.hping.org/)[]()to perform a TCP
ping against port 25, as demonstrated in [Example 4.9](https://nmap.org/book/solution-find-open-port.html#port-scanning-ex-playboy-mxping-tcp).

Example 4.9. TCP pinging the MX servers

[]()

```
core# hping2 --syn -p 25 -c 5 mx.chi.playboy.com
eth0 default routing interface selected (according to /proc)
HPING mx.chi.playboy.com (eth0 216.163.143.4): S set, 40 headers + 0 data bytes
46 bytes from 216.163.143.4: flags=SA seq=0 ttl=51 id=14221 rtt=56.8 ms
46 bytes from 216.163.143.4: flags=SA seq=1 ttl=51 id=14244 rtt=56.9 ms
46 bytes from 216.163.143.4: flags=SA seq=2 ttl=51 id=14274 rtt=56.9 ms
46 bytes from 216.163.143.4: flags=SA seq=3 ttl=51 id=14383 rtt=61.8 ms
46 bytes from 216.163.143.4: flags=SA seq=4 ttl=51 id=14387 rtt=57.5 ms

--- mx.chi.playboy.com hping statistic ---
5 packets transmitted, 5 packets received, 0% packet loss
round-trip min/avg/max = 56.8/58.0/61.8 ms

core# hping2 --syn -p 25 -c 5 mx.la.playboy.com
eth0 default routing interface selected (according to /proc)
HPING mx.la.playboy.com (eth0 216.163.128.15): S set, 40 headers + 0 data bytes
46 bytes from 216.163.128.15: flags=SA seq=0 ttl=52 id=58728 rtt=16.0 ms
46 bytes from 216.163.128.15: flags=SA seq=1 ttl=52 id=58753 rtt=15.4 ms
46 bytes from 216.163.128.15: flags=SA seq=2 ttl=52 id=58790 rtt=15.5 ms
46 bytes from 216.163.128.15: flags=SA seq=3 ttl=52 id=58870 rtt=16.4 ms
46 bytes from 216.163.128.15: flags=SA seq=4 ttl=52 id=58907 rtt=15.5 ms

--- mx.la.playboy.com hping statistic ---
5 packets transmitted, 5 packets received, 0% packet loss
round-trip min/avg/max = 15.4/15.8/16.4 ms

```

These are the results I was looking for. The LA host never
takes more than 16 milliseconds to respond, while the Chicago one
takes up to 62 milliseconds. This is not surprising, given that I am
probing from a machine in California. It pays to be cautious, and
latency can increase during heavy scanning, so I decide to let Nmap
wait up to 200 milliseconds for responses. I'll have it start with a
timeout of 150 ms. So I pass it the options `--max-rtt-timeout
200ms --initial-rtt-timeout 150ms`. To set a generally aggressive
timing mode, I specify `-T4` at the beginning of the
line.

Since I value minimizing completion time of the whole scan over
minimizing the amount of time before the first batch of host results
is returned, I specify a large scan group size. The option`--min-hostgroup 512` is specified so that at least 512
IPs will be scanned in parallel (when possible). Using an exact
factor of the target network size (4096) prevents the small and less
efficient 96-host block which would occur at the end if I specified`--min-hostgroup 500`. All of these timing issues are
explained in much more depth in [Chapter 6, *Optimizing Nmap Performance*](https://nmap.org/book/performance.html).

There is no need to waste time with a prior ping stage, since a
ping would take as long as the single-port scan itself. So`-Pn` is specified to disable that stage. Substantial
time is saved by skipping reverse-DNS resolution with the`-n` argument. Otherwise, with ping scanning disabled,
Nmap would try to look up all 4096 IPs. I am
searching for web servers, so I request port 80 with`-p80`. Of course I will miss any HTTP servers running
on non-standard ports such as 81 or 8080. SSL servers on port 443
won't be found either. One could add them to the `-p`option, but even one more port would double the scan time, which is
roughly proportional to the number of ports scanned.

The final option is `-oG` followed by the
filename in which I want grepable results stored. I append the target
network to the command, then press enter to execute Nmap. The output
is shown in [Example 4.10](https://nmap.org/book/solution-find-open-port.html#port-scanning-ex-playboy-port80-scan).

Example 4.10. Launching the scan

[]()[]()[]()[]()[]()

```
# nmap -T4 -p80 -Pn --max-rtt-timeout 200ms --initial-rtt-timeout 150ms \
  --min-hostgroup 512 -n -oG pb-port80scan-%D.gnmap 216.163.128.0/20
Warning: You specified a highly aggressive --min-hostgroup.
Starting Nmap ( https://nmap.org )
Nmap scan report for 216.163.128.0
PORT   STATE    SERVICE
80/tcp filtered http

Nmap scan report for 216.163.128.1
PORT   STATE    SERVICE
80/tcp filtered http

Nmap scan report for 216.163.128.2
PORT   STATE    SERVICE
80/tcp filtered http

Nmap scan report for 216.163.128.3
PORT   STATE    SERVICE
80/tcp filtered http
[ ... ]
Nmap scan report for 216.163.143.255
PORT   STATE    SERVICE
80/tcp filtered http

Nmap done: 4096 IP addresses (4096 hosts up) scanned in 192.97 seconds

```

Nmap scans all 4096 IPs in about three minutes. The normal
output shows a bunch of ports in the `filtered`state. Most of those IPs are probably not active hosts—the port
simply appears filtered because Nmap receives no response to its SYN
probes. I obtain the list of web servers with a simple **egrep** on the
output file, as shown in [Example 4.11](https://nmap.org/book/solution-find-open-port.html#port-scanning-ex-playboy-port80-grep).

Example 4.11. Egrep for open ports

```
# egrep '[^0-9]80/open' pb-port80scan-*.gnmap
Host: 216.163.140.20 () Ports: 80/open/tcp//http///
Host: 216.163.142.135 ()     Ports: 80/open/tcp//http///

```

After all that effort, only two accessible web servers are
found out of 4096 IPs! Sometimes that happens. The first one,
216.163.140.20 (no reverse DNS name) brings me to a Microsoft Outlook
Web Access (webmail) server. That might excite me if I was trying to
compromise their network, but it isn't gratifying now. The next server
(reverse name mirrors.playboy.com) is much better. It offers those
gigabytes of free images I was hoping for! In particular it offers Linux ISO images as well as substantial FreeBSD, CPAN, and
Apache archives! I download the latest Fedora Core ISOs at a
respectable 6 Mbps. The abundance of bandwidth at Playboy is
not surprising. Later I scan other Playboy netblocks, finding
dozens more web servers, though some of their content is inappropriate
for this book.

While this is an unusual reason for port scanning, single port
sweeps are common for many other purposes expressed previously. The
techniques described here can be easily applied to any single-port TCP
sweep.

### See Also ###

Version detection can be used to find specific applications listening on a network. For example, you could seek a certain vulnerable version of
OpenSSH rather than find all hosts with port 22 open. This is also
useful for single-port UDP scans, as the techniques in this solution
only work well for TCP. Instructions
are provided in [the section called “SOLUTION: Find All Servers Running an Insecure or Nonstandard Application Version”](https://nmap.org/book/vscan-find-service-fast.html).

[Chapter 6, *Optimizing Nmap Performance*](https://nmap.org/book/performance.html) looks at scan speed
optimization in much more depth.

---

[Prev](https://nmap.org/book/port-scanning-ipv6.html)IPv6 Scanning (-6)

[Up](https://nmap.org/book/port-scanning.html)Chapter 4. Port Scanning Overview

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/scan-methods.html)Chapter 5. Port Scanning Techniques and Algorithms