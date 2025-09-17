---
title: "Scan Time Reduction Techniques | Nmap Network Scanning"
source_url: https://nmap.org/book/reduce-scantime.html
fetched_at: 2025-09-17T16:41:15.071477+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 6. Optimizing Nmap Performance](https://nmap.org/book/performance.html)
* Scan Time Reduction Techniques

[Prev](https://nmap.org/book/performance.html)

[Next](https://nmap.org/book/scantime-coping.html)

Scan Time Reduction Techniques
----------

The ideal solution to long scan times is to reduce them. This
section offers many high-level tips for doing so. Unlike many
circumstances in life, tuning your Nmap command line can make a huge
difference. Hot-rodding your Honda Accord with a coffee-can exhaust
tip, a three-foot-high spoiler, and a big red “type R” sticker won't
reduce your 0–60 time much. Yet [the section called “Scanning 676,352 IP Addresses in 46 Hours”](https://nmap.org/book/mayo-scan.html) describes how
Jack Mogren shaved days off his Nmap runtime by simply adding a few stickers
(I mean options) to his Nmap command line.

### Omit Non-critical Tests ###

The electronic equivalent to buying a Hummer when you never
leave the pavement or carry more than groceries is to launch an
intense and comprehensive Nmap scan to obtain a relatively trivial
amount of information. Wasting a few seconds per host rarely matters
on a home network, but can make daily WAN scans infeasible for large
enterprises. The following list details ways to avoid common over-scanning
mistakes, starting with the most egregious problems and followed by more subtle optimizations that even advanced users often forget.

[ Skip the port scan (`-sn`) when you only need to determine what hosts are online.]()

[Some people determine whether a host is online using the command **nmap *`<hostname>`***. While this works, it is overkill. Nmap will send four packets to determine that the host is up, then at least 1,000 to port scan the host. The problem is amplified when a whole network is scanned this way to find all online hosts, or one particular host.]()

[Rather than waste time port scanning, specify `-sn` to do a ping scan when all you wish to know is what hosts are up or what their MAC addresses are.]()

[]()[ Limit the number of ports scanned. ]()

[By default, Nmap scans the most common 1,000 ports. On a fast network of responsive machines, this may take a fraction of a second per host. But Nmap must slow down dramatically when it encounters rate limiting or firewalls that drop probe packets without responding. UDP scans can be agonizingly slow for these reasons. Yet the vast majority of open ports fall into just a few hundred port numbers. A port scan will be about 10 times as fast if you only scan 100 ports instead of the default 1,000. You can scan just the most popular 100 ports with the `-F` (fast scan) option, specify an arbitrary number of the most commonly open ports with `--top-ports`, or provide a custom list of ports to `-p`.]()

[Skip advanced scan types (`-sC`, `-sV`, `-O`, `--traceroute`, and `-A`). ]()[

Some people regularly specify the `-A` Nmap option, which gives them the works. It causes Nmap to do OS detection, version detection, script scanning (NSE), and traceroute as well as the default port scan. Version detection can be extraordinarily useful, but can also bog down a large scan. So can NSE. When pressed for time, you can always skip `-sC` and `-sV` on the large scale scan and then perform them on individual ports as necessary later.

]()

[OS detection is not nearly as slow as version detection, but it can still easily take up 5–10 seconds per online host. Even without this, you can often guess the OS based on the name, open ports, and MAC address on a LAN. And in many cases you may not care about the OS. So `-O` is another candidate for only-as-necessary use. As a compromise, you can specify `--osscan-limit --max-os-tries 1`]()[]()[ which tells Nmap not to retry OS detection attempts which fail to match, and also to skip OS detection against any online hosts that don't have at least one open TCP port and one closed TCP port. OS detection isn't as accurate against such hosts anyway. ]()

[Remember to turn off DNS resolution when it isn't necessary. ]()[ ]()

[By default, Nmap performs reverse-DNS resolution against every host that is found to be online. It is done against all hosts if you skip the ping step with `-Pn`]()[ or specify `-R`.]()[ This was a major bottleneck when host DNS libraries were used to look up one IP at a time.]()

[While Nmap now has a fast parallel reverse-DNS system to speed queries, they still can take a substantial amount of time. Disable them with the `-n` option when you don't need the data. For simple scans (such as ping scans) against a large number of hosts, omitting DNS can sometimes reduce scan time by 20% or more. DNS time is not a major factor in more involved scans which probe thousands of ports or utilize intensive features such as version detection. If you want the Nmap host machine to handle name resolution (using the `gethostbyaddr` function), specify the `--system-dns`]()[ option. Doing so can slow scans down dramatically.]()

### [Optimize Timing Parameters]() ###

[]()[]()

Nmap offers dozens of options for providing hints and rules to
control scan activity. These range from high level timing
aggressiveness levels provided by the`-T`[]()option
(described in [the section called “Timing Templates (`-T`)”](https://nmap.org/book/performance-timing-templates.html)) to the
finer-grained controls described in[the section called “Low-Level Timing Controls”](https://nmap.org/book/performance-low-level.html). You can even combine the
two. These options are particularly useful when scanning highly
filtered networks where Nmap receives few responses to determine its
own timing estimates. Scan time can often be safely cut in half.
Most of these options will have little effect against a local LAN
filled with responsive hosts, as Nmap can determine optimal values
itself in that case.

### Separate and Optimize UDP Scans ###

[]()

Scanning UDP ports is important because many vulnerable services
use that protocol, but the timing characteristics and performance
requirements of UDP scans are much different than TCP scans. Of
particular concern is ICMP error rate-limiting, which is extremely
common and affects UDP scans far more often than TCP.

For these reasons, I don't recommend combining TCP and UDP scans
when performance is critical, even though Nmap supports doing so with
options such as `-sSU`. You often want
different timing flags for each protocol, requiring separate command lines.[the section called “Speeding Up UDP Scans”](https://nmap.org/book/scan-methods-udp-scan.html#scan-methods-udp-optimizing) provides valuable tricks
and real-life examples for improving UDP scan performance.

### Upgrade Nmap ###

There have been many cases where I have investigated reports of
poor Nmap performance only to find that the reporter used an ancient
version that was many years out of date. The newest versions of Nmap
have important algorithmic improvements, bug fixes,
performance-enhancing features such as local network ARP scanning, and
more. The first response to performance problems should be to compare
your version of Nmap (run **nmap -V**) with the latest
version available from [Home page](https://nmap.org/).
Upgrade if necessary. If it is still not fast enough, try the other
techniques in this chapter.

### Execute Concurrent Nmap Instances ###

[]()

Some people try to speed up Nmap by executing many copies in
parallel against one target each. For example, the Nessus scanner[]()used to do this by default. This is usually much less efficient and slower
than letting Nmap run against the whole network. Nmap has its own
parallelization[]()system that is customized to its needs, and Nmap is
able to speed up as it learns about network reliability when it scans
a large group. Further, there is substantial overhead in asking the
OS to fork 65,536 separate Nmap instances just to scan a class B.
Having dozens of copies of Nmap running in parallel is also a memory
drain since each instance loads its own copy of the data files such as`nmap-services` and`nmap-os-db`.

While launching single-host Nmap scans in parallel is a bad
idea, overall speed can usually be improved by dividing the scan into
several large groups and executing those concurrently. Don't go
overboard though. Five or ten Nmap processes are fine, but launching 100 Nmap processes at once is not
recommended. Launching too many concurrent Nmap processes leads to
resource contention. Another sort of concurrency is to run Nmap from
different hosts at once. You can have **cron** (or **At** on Windows)
schedule local hosts on each of your networks to start scanning machines
local to them at the same time, then mail the results to a central data
server. Scanning your Australian network from the U.S. will be slower
than scanning it from a local machine on that network. The difference will be even
greater if the U.S. machine must traverse extra firewalls to reach the
distant network.

### Scan From a Favorable Network Location ###

Restrictive firewalls can turn a five-second scan into a multi-hour
chore. The
latency[]()and packet loss[]()associated with some Internet
routes doesn't help either. If you can run Nmap from host(s) local to
the target network, do so. Of course if the goal is to view the
network as an external attacker would, or to test the firewall,
external scanning is required. On the other hand, scanning and
securing the internal network provides defense in depth which is
critical against internal threats and those wily attackers who circumvent
the firewall (see [Chapter 10, *Detecting and Subverting Firewalls and Intrusion Detection Systems*](https://nmap.org/book/firewalls.html)).

When doing reverse DNS resolution, especially if you have a heavily burdened
local nameserver, it can help to use a less busy nameserver or directly query
the authoritative nameservers[](). This gain is usually slight and only worth doing for repeated or enormous scans. Of course, there are sometimes non-performance reasons for choosing nameservers.

### Increase Available Bandwidth and CPU Time ###

You can occasionally improve Nmap scan times by increasing your
available bandwidth or CPU power. This may be done either by installing
a new data line or CPU, or by halting concurrently running applications
which compete for these resources. For example, Nmap will run slower if you concurrently saturate your DSL line by downloading a pirate torrent of *The Matrix
Reloaded*.

It is far more common that Nmap is constrained by its own
congestion control algorithms[]()than being CPU-bound or limited by the
available local bandwidth. These controls help prevent network
flooding and increase accuracy. Increasing CPU power and local
bandwidth won't help this sort of self-limiting by Nmap—timing
options must be adjusted instead. You can test whether Nmap is CPU
constrained by monitoring your CPU load with an application such astop on Unix or the Task
Manager on Windows. If your CPU spends most of its time
idle, then upgrading won't help much. To test Nmap's bandwidth usage,
run it in verbose mode (`-v`). Nmap will then report
the number of bytes sent and received and its execution time, as shown
in [Example 6.1](https://nmap.org/book/reduce-scantime.html#performance-bandwidth-usage).

Example 6.1. Bandwidth usage over local 100 Mbps ethernet network

```
# nmap -v -n -p- sec.titan.net

Starting Nmap ( https://nmap.org )
[10 lines cut]
Nmap scan report for 192.168.0.8
Not shown: 65534 closed ports
PORT   STATE SERVICE
22/tcp open  ssh
MAC Address: 00:1A:6B:C1:33:37 (USI)

Nmap done: 1 IP address (1 host up) scanned in 2.20 seconds
           Raw packets sent: 65536 (2.884MB) | Rcvd: 65536 (2.621MB)

```

Multiply the byte values by eight and divide by the execution time
to get the average bandwidth usage in bits per second. In[Example 6.1](https://nmap.org/book/reduce-scantime.html#performance-bandwidth-usage),
Nmap received 2,621,000 bytes
(Nmap considers 1,000,000 bytes to be a MB) in 2.20 seconds. So
receive traffic was about 9.5 Mbps (send rate was 10.5 Mbps). Therefore the 100 Mbps ethernet link
isn't likely constraining Nmap, and upgrading to gigabit ethernet won't help
much.

Some consumer broadband devices and other equipment struggles to
handle the rate of packets sent by Nmap, even though the
small packet size (usually Nmap sends empty headers) keeps bandwidth
low. In [Example 6.1, “Bandwidth usage over local 100 Mbps ethernet network”](https://nmap.org/book/reduce-scantime.html#performance-bandwidth-usage), Nmap sent
about 30,000 packets per second and received a similar number. Such
high packet rates can cause problem with low-quality devices. In this
case, we see that both send and receive packet counts were 65,536,
which is the number of scanned ports (65,535) plus one for the initial
ARP ping probe. Therefore Nmap did not encounter any packet drops
requiring retransmission. This suggests again that the networking
equipment was not a limiting factor—Nmap was probably CPU
bound.

---

[Prev](https://nmap.org/book/performance.html)Chapter 6. Optimizing Nmap Performance

[Up](https://nmap.org/book/performance.html)Chapter 6. Optimizing Nmap Performance

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/scantime-coping.html)Coping Strategies for Long Scans