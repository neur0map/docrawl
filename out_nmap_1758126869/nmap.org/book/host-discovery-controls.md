---
title: "Host Discovery Controls | Nmap Network Scanning"
source_url: https://nmap.org/book/host-discovery-controls.html
fetched_at: 2025-09-17T16:39:23.810709+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 3. Host Discovery (“Ping Scanning”)](https://nmap.org/book/host-discovery.html)
* Host Discovery Controls

[Prev](https://nmap.org/book/host-discovery-dns.html)

[Next](https://nmap.org/book/host-discovery-techniques.html)

Host Discovery Controls
----------

By default, Nmap will include a ping scanning
stage prior to more intrusive probes such as port scans, OS detection,
Nmap Scripting Engine, or version detection. Nmap usually only performs intrusive scans on
machines that are shown to be available during the ping scan stage. This
saves substantial time and bandwidth compared to performing full scans against every single
IP address. Yet this approach is not ideal for all
circumstances. There are times when you *do* want
to scan every IP (`-Pn`), and other times when you want
to perform host discovery without a port scan (`-sn`). There
are even times when you want to print out the target hosts and exit
prior to even sending ping probes (`-sL`). Nmap offers
several high-level options to control this behavior.

### List Scan (`-sL`) ###

[]()[]()

List scan is a degenerate form of host discovery that
simply lists each host on the network(s) specified, without sending
any packets to the target hosts.
By default, Nmap still performs reverse-DNS resolution on the hosts to[]()learn their names. Nmap also reports the total number of IP addresses at the
end. List scan is a good sanity check to ensure that you have
proper IP addresses for your targets. If the hosts display domain names
you do not recognize, it is worth investigating further to prevent
scanning the wrong company's network.

[]()

There are many reasons target IP ranges can be incorrect. Even
network administrators can mistype their own netblocks, and
pen-testers have even more to worry about. In some cases, security
consultants are given the wrong addresses. In others, they try to
find proper IP ranges through resources such as whois databases and
routing tables. The databases can be out of date, or the company
could be loaning IP space to other organizations. Whether to scan
corporate parents, siblings, service providers, and subsidiaries is an
important issue that should be worked out with the customer in advance. A
preliminary list scan helps confirm exactly what targets are being
scanned.

[]()

Another reason for an advance list scan is stealth. In some
cases, you do not want to begin with a full-scale assault on the
target network that is likely to trigger IDS alerts and bring unwanted
attention. A list scan is unobtrusive and provides information that
may be useful in choosing which individual machines to target. It is
possible, though highly unlikely, that the target will notice all of
the reverse-DNS requests. When that is a concern, you can bounce through anonymous recursive DNS servers using the`--dns-servers`[]()option as described in [the section called “DNS proxying”](https://nmap.org/book/subvert-ids.html#defeating-firewalls-dns-proxy).

A list scan is specified with the `-sL`command-line option. Since the idea is to simply print a list of
target hosts, options for higher level functionality such as port
scanning, OS detection, or ping scanning cannot be combined with `-sL`.
If you wish to disable ping scanning while still performing such
higher level functionality, read up on the `-Pn` option. [Example 3.6](https://nmap.org/book/host-discovery-controls.html#host-discovery-ex-listscan) shows list scan being used
to enumerate the CIDR /28 network range (16 IP
addresses) surrounding the main Stanford University web server.

Example 3.6. Enumerating hosts surrounding www.stanford.edu with list scan

[]()

```
felix~> nmap -sL www.stanford.edu/28

Starting Nmap ( https://nmap.org )
Host www9.Stanford.EDU (171.67.16.80) not scanned
Host www10.Stanford.EDU (171.67.16.81) not scanned
Host scriptorium.Stanford.EDU (171.67.16.82) not scanned
Host coursework-a.Stanford.EDU (171.67.16.83) not scanned
Host coursework-e.Stanford.EDU (171.67.16.84) not scanned
Host www3.Stanford.EDU (171.67.16.85) not scanned
Host leland-dev.Stanford.EDU (171.67.16.86) not scanned
Host coursework-preprod.Stanford.EDU (171.67.16.87) not scanned
Host stanfordwho-dev.Stanford.EDU (171.67.16.88) not scanned
Host workgroup-dev.Stanford.EDU (171.67.16.89) not scanned
Host courseworkbeta.Stanford.EDU (171.67.16.90) not scanned
Host www4.Stanford.EDU (171.67.16.91) not scanned
Host coursework-i.Stanford.EDU (171.67.16.92) not scanned
Host leland2.Stanford.EDU (171.67.16.93) not scanned
Host coursework-j.Stanford.EDU (171.67.16.94) not scanned
Host 171.67.16.95 not scanned
Nmap done: 16 IP addresses (0 hosts up) scanned in 0.38 seconds

```

### Disable Port Scan (`-sn`) ###

[]()[]()[]()

This option tells Nmap not to run a port scan after host
discovery. When used by itself, it makes Nmap do host discovery,
then print out the available hosts that responded to the scan.
This is often called a “ping scan”. Even though no port
scanning is done, you can still request Nmap Scripting Engine
(`--script`) host scripts and traceroute probing
(`--traceroute`). A ping-only scan
is one step more intrusive than a list scan, and can often be used
for the same purposes. It performs light reconnaissance of a target
network quickly and without attracting much attention. Knowing how
many hosts are up is more valuable to attackers than the list of every
single IP and host name provided by list scan.

Systems administrators often find this option valuable as well.
It can easily be used to count available machines on a network or
monitor server availability. This is often called a ping sweep, and
is more reliable than pinging the broadcast address because many hosts
do not reply to broadcast queries.

[Example 3.7](https://nmap.org/book/host-discovery-controls.html#host-discovery-ex-pingscan)shows a quick ping scan against the CIDR /24 (256
IPs) surrounding one of my favorite web sites, Linux Weekly News.

Example 3.7. Discovering hosts surrounding `www.lwn.net` with a ping scan

[]()

```
# nmap -sn -T4 www.lwn.net/24

Starting Nmap ( https://nmap.org )
Host 66.216.68.0 seems to be a subnet broadcast address (returned 1 extra ping)
Host 66.216.68.1 appears to be up.
Host 66.216.68.2 appears to be up.
Host 66.216.68.3 appears to be up.
Host server1.camnetsec.com (66.216.68.10) appears to be up.
Host akqa.com (66.216.68.15) appears to be up.
Host asria.org (66.216.68.18) appears to be up.
Host webcubic.net (66.216.68.19) appears to be up.
Host dizzy.yellowdog.com (66.216.68.22) appears to be up.
Host www.outdoorwire.com (66.216.68.23) appears to be up.
Host www.inspectorhosting.com (66.216.68.24) appears to be up.
Host jwebmedia.com (66.216.68.25) appears to be up.
[...]
Host rs.lwn.net (66.216.68.48) appears to be up.
Host 66.216.68.52 appears to be up.
Host cuttlefish.laughingsquid.net (66.216.68.53) appears to be up.
[...]
Nmap done: 256 IP addresses (105 hosts up) scanned in 12.69 seconds

```

This example only took 13 seconds, but provides valuable
information. In that class C sized address range, 105 hosts are online. From
the unrelated domain names all packed into such a small
IP space, it is clear that LWN uses a colocation or dedicated server
provider. If the LWN machines turn out to be highly secure, an
attacker might go after one of those neighbor machines and then
perform a local ethernet attack with tools such asEttercap orDsniff. An ethical use of this data would be a
network administrator considering moving machines to this provider.
He might e-mail a few of the listed organizations and ask their
opinion of the service before signing a long-term contract or making
the expensive and disruptive datacenter move.

[]()

The `-sn` option sends an ICMP echo request, a
TCP SYN packet to port 443, a TCP ACK packet to port 80, and an ICMP
timestamp request by default. Since unprivileged Unix users
(or Windows users without Npcap installed) cannot send these raw
packets, only SYN packets are sent instead in those cases. The SYN packet
is sent using a TCP `connect`system call to ports
80 and 443 of the target host. When a privileged user tries to scan targets
on a local
ethernet network,[]()ARP requests (`-PR`)[]()are used unless the`--send-ip`[]()option is specified.

The `-sn` option can be combined with any of the
techniques discussed in [the section called “Host Discovery Techniques”](https://nmap.org/book/host-discovery-techniques.html)for greater flexibility. If any of those probe type and port number
options are used, the default probes are
overridden. When strict firewalls are in place between the source
host running Nmap and the target network, using those advanced
techniques is recommended. Otherwise hosts could be missed when the
firewall drops probes or their responses.

### Disable Ping (`-Pn`) ###

[]()[]()

Another option is to skip the Nmap discovery stage altogether.
Normally, Nmap uses this stage to determine active machines for
heavier scanning. By default, Nmap only performs heavy probing such
as port scans, version detection, or OS detection against hosts that
are found to be up. Disabling host discovery with
the `-Pn` option causes Nmap to attempt the requested scanning
functions against*every* target IP address specified. So if a class
B sized target address space (/16) is specified on the command line,
all 65,536 IP addresses are scanned. Proper host discovery is skipped
as with a list scan, but instead of stopping and printing the target
list, Nmap continues to perform requested functions as if each target
IP is active.

There are many reasons for disabling the Nmap ping tests. One
of the most common is intrusive vulnerability assessments. One can
specify dozens of different ping probes in an attempt to elicit a
response from all available hosts, but it is still possible that an
active yet heavily firewalled machine might not reply to any of those probes.
So to avoid missing anything, auditors frequently perform intense
scans, such as for all 65,536 TCP ports, against every IP on the
target network. It may seem wasteful to send hundreds of thousands of
packets to IP addresses that probably have no host listening, and it can slow
scan times by an order of magnitude or more. Nmap must send
retransmissions to every port in case the original probe was dropped
in transit, and Nmap must spend substantial time waiting for responses
because it has no round-trip-time (RTT) estimate for these non-responsive IP
addresses. But serious penetration testers are willing to pay this
price to avoid even a slight risk of missing active machines. They
can always do a quick scan as well, leaving the massive`-Pn` scan to run in the background while they work.[Chapter 6, *Optimizing Nmap Performance*](https://nmap.org/book/performance.html) provides more performance tuning advice.

Another frequent reason given for using `-Pn` is
that the tester has a list of machines that are already known to be
up. So the user sees no point in wasting time with the host discovery
stage. The user creates their own list of active hosts and then
passes it to Nmap using the `-iL` (take input from
list) option. This strategy is rarely beneficial from a time-saving
perspective. Due to the retransmission and RTT estimate issues
discussed in the previous paragraph, even one unresponsive IP address
in a large list will often take more time to scan than a whole ping
scanning stage would have. In addition, the ping stage allows Nmap to
gather RTT samples that can speed up the following port scan,
particularly if the target host has strict firewall rules. While
specifying `-Pn` is rarely helpful as a time saver, it
is important if some of the machines on your list block all of the
discovery techniques that would otherwise be specified. Users must
strike a balance between scan speed and the possibility of missing
heavily cloaked machines.

---

[Prev](https://nmap.org/book/host-discovery-dns.html)DNS Resolution

[Up](https://nmap.org/book/host-discovery.html)Chapter 3. Host Discovery (“Ping Scanning”)

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/host-discovery-techniques.html)Host Discovery Techniques