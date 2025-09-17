---
title: "Detecting Packet Forgery by Firewall and Intrusion Detection Systems | Nmap Network Scanning"
source_url: https://nmap.org/book/firewall-ids-packet-forgery.html
fetched_at: 2025-09-17T16:44:44.853118+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 10. Detecting and Subverting Firewalls and Intrusion Detection Systems](https://nmap.org/book/firewalls.html)
* Detecting Packet Forgery by Firewall and Intrusion Detection Systems

[Prev](https://nmap.org/book/subvert-ids.html)

[Next](https://nmap.org/book/defenses.html)

Detecting Packet Forgery by Firewall and Intrusion Detection Systems
----------

[]()

Previous sections mentioned that some firewall and intrusion
detection systems can be configured to forge packets as if they came
from one of the protected systems behind the device. TCP RST packets
are a frequent example. Load balancers, SSL accelerators, network
address translation devices, and certain honeynets can also lead to confusing
or inconsistent results. Understanding how Nmap interprets responses
helps a great deal in piecing together complex remote network
topologies. When Nmap reports unusual or unexpected results, you can
add the`--packet-trace`[]()option to see the raw packets
upon which Nmap based its conclusions. In perplexing situations, you
may have to go even further and launch custom probes and analyze
packets with other tools such as
Nping[]()and Wireshark.[]()The goal is
often to find inconsistencies that help you understand the actual
network setup. The following sections describe several useful
techniques for doing so. While most of these tests do not involve
Nmap directly, they can be useful for interpreting unexpected Nmap
results.

### Look for TTL Consistency ###

[]()

Firewalls, load balancers, NAT gateways, and similar devices are
usually located one or more hops in front of the machines they are
protecting. In this case, packets can be created with a TTL such that
they reach the network device but not the end host. If a RST is
received from such a probe, it must have been sent by the
device.

During one informal assessment, I scanned the network of a large
magazine publisher over the Internet (you may remember them from [the section called “SOLUTION: Scan a Large Network for a Certain Open TCP Port”](https://nmap.org/book/solution-find-open-port.html)). Almost every IP address showed
port 113 closed. Suspecting RST forgery by a firewall, I dug a bit
deeper. Because it contained open, closed, and filtered ports, I
decided to focus on this host in particular:

[]()

```
# nmap -sS -Pn -T4 mx.chi.playboy.com
Starting Nmap ( https://nmap.org )
Nmap scan report for mx.chi.playboy.com (216.163.143.4)
Not shown: 998 filtered ports
PORT    STATE  SERVICE
25/tcp  open   smtp
113/tcp closed auth

Nmap done: 1 IP address (1 host up) scanned in 53.20 seconds

```

Is port 113 really closed, or is the firewall spoofing RST packets? I counted the distance (in network hops) to ports 25 and 113 using the custom traceroute mode of the free
hping2[]()utility, as shown in [Example 10.22](https://nmap.org/book/firewall-ids-packet-forgery.html#defeating-firewalls-ids-customtraceroute). I could have used the faster Nmap `--traceroute` option to do this, but that option did not exist at the time.

Example 10.22. Detection of closed and filtered TCP ports

```
# hping2 -t 5 --traceroute -p 25 -S mx.chi.playboy.com
[combined with results from hping2 -i 1 --ttl \* -p 25 -S mx.chi.playboy.com]
5->TTL 0 during transit from 64.159.2.97 (ae0-54.mp2.SanJose1.Level3.net)
6->TTL 0 during transit from 64.159.1.34 (so-3-0-0.mp2.Chicago1.Level3.net)
7->TTL 0 during transit from 200.247.10.170 (pos9-0.core1.Chicago1.level3.net)
8->TTL 0 during transit from 200.244.8.42 (gige6-0.ipcolo1.Chicago1.Level3.net)
9->TTL 0 during transit from 166.90.73.205 (ge1-0.br1.ord.playboy.net)
10->TTL 0 during transit from 216.163.228.247 (f0-0.b1.chi.playboy.com)
11->No response
12->TTL 0 during transit from 216.163.143.130 (fw.chi.playboy.com)
13->46 bytes from 216.163.143.4: flags=SA seq=0 ttl=52 id=48957 rtt=75.8 ms

# hping2 -t 5 --traceroute -p 113 -S mx.chi.playboy.com
[ results augmented again ]
5->TTL 0 during transit from 64.159.2.97 (ae0-54.mp2.SanJose1.Level3.net)
6->TTL 0 during transit from 64.159.1.34 (so-3-0-0.mp2.Chicago1.Level3.net)
7->TTL 0 during transit from 200.247.10.170 (pos9-0.core1.Chicago1.level3.net)
8->TTL 0 during transit from 200.244.8.42 (gige6-0.ipcolo1.Chicago1.Level3.net)
9->TTL 0 during transit from 166.90.73.205 (ge1-0.br1.ord.playboy.net)
10->TTL 0 during transit from 216.163.228.247 (f0-0.b1.chi.playboy.com)
11->Nothing
12->46 bytes from 216.163.143.4: flags=RA seq=0 ttl=48 id=53414 rtt=75.0 ms

```

This custom traceroute shows that reaching open port 25 requires
13 hops. 12 hops away is a firewall in Chicago, helpfully named
fw.chi.playboy.com. One would expect different ports on the same
machine to be the same hop-distance away. Yet port 113 responds with
a RST after only 12 hops.[]()[]()That RST is being forged by
fw.chi.playboy.com. Since the firewall is known to forge port 113
responses, those packets should not be taken as an indicator that a
host is available at a given IP address. I found available hosts by
ping scanning the network again, using common probe types such as
ICMP echo requests (`-PE`) and SYN packets to ports 22
and 80 (`-PS22,80`), but
omitting any ping probes involving TCP port 113.

### Look for IP ID and Sequence Number Consistency ###

[]()[]()

Every IP packet contains a 16-bit identification field that is used for defragmentation. It can also be exploited to gain a surprising amount of information on remote hosts. This includes port scanning using the Nmap idle scan technique, traffic estimation, host alias detection, and much more. It can also help to detect many network devices, such as load balancers. I once noticed strange OS detection results when scanning beta.search.microsoft.com. So I launched hping2 SYN probes against TCP port 80 to learn what was going on. [Example 10.23](https://nmap.org/book/firewall-ids-packet-forgery.html#defeating-firewalls-ids-ipid-ms) shows the results.

Example 10.23. Testing IP ID sequence number consistency

```
# hping2 -c 10 -i 1 -p 80 -S beta.search.microsoft.com
HPING beta.search.microsoft.com. (eth0 207.46.197.115): S set, 40 headers
46 bytes from 207.46.197.115: flags=SA seq=0 ttl=56 id=57645 win=16616
46 bytes from 207.46.197.115: flags=SA seq=1 ttl=56 id=57650 win=16616
46 bytes from 207.46.197.115: flags=RA seq=2 ttl=56 id=18574 win=0
46 bytes from 207.46.197.115: flags=RA seq=3 ttl=56 id=18587 win=0
46 bytes from 207.46.197.115: flags=RA seq=4 ttl=56 id=18588 win=0
46 bytes from 207.46.197.115: flags=SA seq=5 ttl=56 id=57741 win=16616
46 bytes from 207.46.197.115: flags=RA seq=6 ttl=56 id=18589 win=0
46 bytes from 207.46.197.115: flags=SA seq=7 ttl=56 id=57742 win=16616
46 bytes from 207.46.197.115: flags=SA seq=8 ttl=56 id=57743 win=16616
46 bytes from 207.46.197.115: flags=SA seq=9 ttl=56 id=57744 win=16616

```

Looking at the sequence of IP ID numbers (in bold), it is clear
that there are really two machines sharing this IP address through some
sort of load balancer. One has IP ID sequences in the range of 57K,
while the other is using 18K. Given this information, it is no wonder
that Nmap had trouble settling on a single operating system guess.
They may be running on very different systems.

Similar tests can be performed on other numeric fields, such as
the TCP timestamp option or the initial sequence number returned by
open ports. In this particular case, you can see that the TCP window
size and TCP flags also give the hosts away.

### The Bogus TCP Checksum Trick ###

[]()[]()

Another handy trick for determining whether an IDS or firewall
is spoofing response packets is to send probes with a bogus TCP checksum.
Essentially all end hosts check the checksum before further processing
and will not respond to these corrupt packets. Firewalls, on the
other hand, often omit this check for performance reasons. We can detect this behavior
with the`--badsum`[]() option, as shown in [Example 10.24](https://nmap.org/book/firewall-ids-packet-forgery.html#defeating-firewalls-ids-badsum-example).

Example 10.24. Finding a firewall with bad TCP checksums

[]()

```
# nmap -sS -p 113 -Pn --badsum google.com

Starting Nmap ( https://nmap.org )
Warning: Hostname google.com resolves to 3 IPs. Using 64.233.187.99.
Nmap scan report for jc-in-f99.google.com (64.233.187.99)
PORT    STATE  SERVICE
113/tcp closed auth

Nmap done: 1 IP address (1 host up) scanned in 0.44 seconds

```

From [Example 10.24](https://nmap.org/book/firewall-ids-packet-forgery.html#defeating-firewalls-ids-badsum-example) we can infer that
there is some sort of network device, perhaps a firewall, that is handling packets
destined to google.com on port 113 without verifying TCP checksums. Normally, an
end host will silently drop packets with bad TCP checksums and we will see a filtered
port instead of a closed one. `--badsum` will also use bad checksums for
other protocols on top of IP, including UDP, ICMP, and IGMP.

This technique, along with other reasons for deliberately sending packets with
malformed checksums, is further described in [*Phrack* 60, article
12](https://nmap.org/p60-12.html) by Ed3f. While this is sometimes a useful technique, there are
several caveats to consider:

1. Many modern firewalls now verify TCP checksums (at least when determining whether to respond to a packet) to avoid leaking this information. So this technique is more useful for proving that a `--badsum` probe response was sent by a firewall (or other device with an incomplete TCP stack) than for proving that a filtered `--badsum` probe was dropped by an end host.

2. []()Using `--badsum` does not guarantee that packets will be sent with
   bad checksums on all platforms. On a few systems, the kernel or the network card
   performs the checksum calculation and insert the correct value, overwriting
   the desired bad value. One way to make sure this isn't happening to you is to
   use a remote machine to sniff the packets you are sending. For example, when sniffing
   with tcpdump, packets with bad TCP checksums will be
   indicated like `[bad tcp cksum aa79 (->ab79)!]`. Another approach is to do a normal SYN scan against one of your hosts (with at least one open port). Then do the same scan with `--badsum`. If the same ports are still shown as open, then `--badsum` probably isn't working for you. Please report the problem as described in [the section called “Bugs”](https://nmap.org/book/man-bugs.html).

### Round Trip Times ###

[]()[]()

When a firewall forges a probe response, that response
usually returns slightly sooner than a response from the true
destination host would. After all, the firewall is usually at least
one hop closer. It is also optimized for quickly parsing and
processing packets, and does little else. The destination host, on
the other hand, may be so busy running applications that it takes
several milliseconds longer to respond to a probe. Thus, a close comparison
of round trip times can often give away firewall shenanigans.

A challenge with this technique is that the time discrepancy
between a firewall response and a true target response may be a
fraction of a millisecond. Normal round trip time variances may be
greater than that, so sending just two probes (one that solicits a
response known to be from the target host, and one suspect response
that may be from the firewall) is rarely enough. Sending a thousand
of each probe type cancels out most of the RTT variance so that
fundamental differences can be discerned. This doesn't need to take
all that
long—**nping**[]()with the options `-c 1000 --rate 20`sends a thousand probes in less than a minute. From those results,
calculate the median rather than using the average it gives you. This
prevents enormous times (such as from a lost response that is
retransmitted two seconds later) from skewing the data. Do the
thousand probes once or twice more to determine how consistent the
results are. Then try the same with the suspect probe and compare the
two. If the times are exactly the same to the last significant digit,
the same host is likely sending both responses. If you consistently
see that one probe type responds more quickly than the other, packet forgery may be responsible.

This method isn't perfect. A time discrepancy could be caused
by any number of other factors than a firewall. It is still a
valuable technique, as detecting network anomalies such as packet
forgery is like proving a court case. Every little bit of evidence
helps toward reaching a conclusion. The discrepancy may even lead to
more interesting discoveries than firewall forgery. Maybe certain
ports on the target are being redirected to a
honeynet[]()to better study attacks.

### Close Analysis of Packet Headers and Contents ###

It is surprising how many elements can differ in even a small
TCP header. Refer to [Chapter 8, *Remote OS Detection*](https://nmap.org/book/osdetect.html) for dozens of subtle details that can
be indicative of a different OS. For example, different systems
respond with different TCP options, RST packet text, type of service values,
etc. If there are several systems behind a load balancer, or the
packets are being sent by firewall or intrusion detection systems, the
packets will rarely match exactly.

An excellent tool for dissecting packet headers isWireshark[]()because it can break the header
out into individual fields and provide textual descriptions of the
binary contents of the packet. The trick to comparing packets is to
collect one packet you think may be from a firewall and another packet
of the same type from the target host or target operating system.
Two packet types you are
likely to be able to collect are TCP reset packets and ICMP error
packets. By using
Nping[]()or the`--scanflags`[]()[]()Nmap option it should be possible to elicit
responses with different IP, TCP, or ICMP headers.

### Unusual Network Uniformity ###

When response packets are sent by a firewall, they are often
more uniform than would be expected from clusters of individual
machines. While scanning the magazine company discussed in the
previous TTL-checking section, I found that hundreds of sequential-IP
machines responded with a RST to port 113. In a real cluster of
machines, you would expect at least a couple to be offline at a given
time. Additionally, I was unable to elicit any other type of response
from most of these addresses. This suspicious result led me to do the
TTL tests which showed that the `fw.chi` host was
actually spoofing the RST packets.

A firewall doesn't even have to spoof packets to give itself
away. Another common firewall configuration is to drop packets to
specific ports. Many ISPs filter Windows ports 135, 139, and 445 to
reduce the spread of worms. If a large number of adjacent live hosts
show up with the same set of filtered ports, a network firewall is the
likely culprit. After determining which ports are being
filtered by a firewall, you can often map out how many hosts are
protected by those firewall rules by scanning many netblocks for those
filtered ports. This can lead to the discovery of any
accidental holes or the organization's DMZ (demilitarized zone) which
typically hosts public services and has far looser firewall rules.

---

[Prev](https://nmap.org/book/subvert-ids.html)Subverting Intrusion Detection Systems

[Up](https://nmap.org/book/firewalls.html)Chapter 10. Detecting and Subverting Firewalls and Intrusion Detection Systems

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/defenses.html)Chapter 11. Defenses Against Nmap