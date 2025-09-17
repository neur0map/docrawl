---
title: "Bypassing Firewall Rules | Nmap Network Scanning"
source_url: https://nmap.org/book/firewall-subversion.html
fetched_at: 2025-09-17T16:44:38.892325+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 10. Detecting and Subverting Firewalls and Intrusion Detection Systems](https://nmap.org/book/firewalls.html)
* Bypassing Firewall Rules

[Prev](https://nmap.org/book/determining-firewall-rules.html)

[Next](https://nmap.org/book/subvert-ids.html)

Bypassing Firewall Rules
----------

[]()

While mapping out firewall rules can be valuable, bypassing
rules is often the primary goal. Nmap implements many techniques for
doing this, though most are only effective against poorly configured
networks. Unfortunately, those are common. Individual techniques
each have a low probability of success, so try as many different methods
as possible. The attacker need only find one misconfiguration to succeed, while
the network defenders must close every hole.

### Exotic Scan Flags ###

[]()

The previous section discussed using an ACK scan to map out
which target network ports are filtered. However, it could not determine
which of the accessible ports were open or closed. Nmap offers
several scan methods that are good at sneaking past firewalls while
still providing the desired port state information.
FIN scan[]()is one
such technique. In [the section called “ACK Scan”](https://nmap.org/book/determining-firewall-rules.html#defeating-firewalls-ids-ackscan), SYN and ACK scans were run against a machine named Para. The SYN
scan showed only two open ports, perhaps due to firewall restrictions.
Meanwhile, the ACK scan is unable to recognize open ports from closed
ones. [Example 10.6](https://nmap.org/book/firewall-subversion.html#defeating-firewalls-fin-scan) shows another
scan attempt against Para, this time using a FIN scan.
Because a naked FIN packet is being set, this packet flies past the
rules blocking SYN packets. While a SYN scan only found one open port
below 100, the FIN scan finds both of them.

Example 10.6. FIN scan against stateless firewall

[]()

```
# nmap -sF -p1-100 -T4 para

Starting Nmap ( https://nmap.org )
Nmap scan report for para (192.168.10.191)
Not shown: 98 filtered ports
PORT   STATE         SERVICE
22/tcp open|filtered ssh
53/tcp open|filtered domain
MAC Address: 00:60:1D:38:32:90 (Lucent Technologies)

Nmap done: 1 IP address (1 host up) scanned in 1.61 seconds

```

Many other scan types are worth trying, since the target
firewall rules and target host type determine which techniques will
work. Some particularly valuable scan types are
FIN,
Maimon,[]()Window,[]()SYN/FIN, and
NULL scans.[]()These are all described in [Chapter 5, *Port Scanning Techniques and Algorithms*](https://nmap.org/book/scan-methods.html).

### Source Port Manipulation ###

[]()

One surprisingly common misconfiguration is to trust traffic
based only on the source port number. It is easy to understand how
this comes about. An administrator will set up a shiny new firewall,
only to be flooded with complains from ungrateful users whose
applications stopped working. In particular, DNS may be broken
because the UDP DNS replies from external servers can no longer enter
the network. FTP is another common example. In active FTP transfers,
the remote server tries to establish a connection back to the client
to transfer the requested file.

Secure solutions to these problems exist, often in the form of
application-level proxies or protocol-parsing firewall modules.
Unfortunately there are also easier, insecure solutions. Noting that
DNS replies come from port 53 and active FTP from port 20, many administrators
have fallen into the trap of simply allowing incoming traffic from
those ports. They often assume that no attacker would notice and
exploit such firewall holes. In other cases, administrators consider this a
short-term stop-gap measure until they can implement a more secure
solution. Then they forget the security upgrade.

Overworked network administrators are not the only ones to fall
into this trap. Numerous products have shipped with these insecure
rules. Even Microsoft has been guilty. The IPsec filters that
shipped with Windows 2000 and Windows XP contain an implicit rule that
allows all TCP or UDP traffic from port 88 (Kerberos). Apple fans
shouldn't get too smug about this because the firewall which shipped
with Mac OS X Tiger is just as bad. Jay Beale[]()discovered
that even if you enable the “Block UDP Traffic” box in the firewall
GUI, packets from port 67 (DHCP) and 5,353 (Zeroconf) pass right
through. Yet another pathetic example of this configuration is that Zone Alarm personal firewall (versions up to 2.1.25) allowed any incoming UDP packets
with the source port 53 (DNS) or 67 (DHCP).

[]()Nmap offers the `-g` and`--source-port`[]()options (they are equivalent) to exploit these
weaknesses. Simply provide a port number, and Nmap will send packets
from that port where possible. Nmap must use different port numbers
for certain OS detection tests to work properly. Most TCP scans, including SYN scan,
support the option completely, as does UDP scan. In May 2004,
JJ Gray[]()posted example Nmap scans to Bugtraq that demonstrate exploitation of
the Windows IPsec source port 88 bug against one of his clients. A
normal scan, followed by a `-g 88` scan are shown in[Example 10.7](https://nmap.org/book/firewall-subversion.html#defeating-firewalls-sourceport88). Some output has
been removed for brevity and clarity.

Example 10.7. Bypassing Windows IPsec filter using source port 88

[]()

```
# nmap -sS -v -v -Pn 172.25.0.14

Starting Nmap ( https://nmap.org )
Nmap scan report for 172.25.0.14
Not shown: 1658 filtered ports
PORT   STATE  SERVICE
88/tcp closed kerberos-sec   

Nmap done: 1 IP address (1 host up) scanned in 7.02 seconds

# nmap -sS -v -v -Pn -g 88 172.25.0.14

Starting Nmap ( https://nmap.org )
Nmap scan report for 172.25.0.14
Not shown: 1653 filtered ports
PORT     STATE SERVICE
135/tcp  open  msrpc
139/tcp  open  netbios-ssn
445/tcp  open  microsoft-ds
1025/tcp open  NFS-or-IIS
1027/tcp open  IIS
1433/tcp open  ms-sql-s

Nmap done: 1 IP address (1 host up) scanned in 0.37 seconds

```

Note that the closed port 88 was the hint that lead JJ to try
using it as a source port. For further information on this
vulnerability, see [Microsoft
Knowledge Base Article 811832](https://support.microsoft.com/kb/811832).

### IPv6 Attacks ###

[]()

While IPv6 has not exactly taken the world by storm, it is
reasonably popular in Japan and certain other regions. When
organizations adopt this protocol, they often forget to lock it down
as they have instinctively learned to do with IPv4. Or they may try
to, but find that their hardware does not support IPv6 filtering
rules. Filtering IPv6 can sometimes be more critical than IPv4
because the expanded address space often allows the allocation of
globally addressable IPv6 addresses to hosts that would normally have
to use the
private IPv4 addresses[]()specified by [RFC 1918](http://www.rfc-editor.org/rfc/rfc1918.txt).

Performing an IPv6 scan rather than the IPv4 default is often as
easy as adding `-6`[]()to the command line. Certain
features such as OS detection and UDP scanning are not yet supported
for this protocol, but the most popular features work. [Example 10.8](https://nmap.org/book/firewall-subversion.html#defeating-firewalls-ex-ipv6) demonstrates IPv4 and IPv6
scans, performed long ago, of a well-known IPv6 development and
advocacy organization.

Example 10.8. Comparing IPv4 and IPv6 scans

[]()

```
> nmap www.kame.net

Starting Nmap ( https://nmap.org )
Nmap scan report for kame220.kame.net (203.178.141.220)
Not shown: 984 closed ports
Port       State       Service
19/tcp     filtered    chargen
21/tcp     open        ftp
22/tcp     open        ssh
53/tcp     open        domain
80/tcp     open        http
111/tcp    filtered    sunrpc
137/tcp    filtered    netbios-ns
138/tcp    filtered    netbios-dgm
139/tcp    filtered    netbios-ssn
513/tcp    filtered    login
514/tcp    filtered    shell
2049/tcp   filtered    nfs
2401/tcp   open        cvspserver
5999/tcp   open        ncd-conf
7597/tcp   filtered    qaz
31337/tcp  filtered    Elite

Nmap done: 1 IP address (1 host up) scanned in 34.47 seconds

> nmap -6 www.kame.net

Starting Nmap ( https://nmap.org )
Nmap scan report for 3ffe:501:4819:2000:210:f3ff:fe03:4d0
Not shown: 994 closed ports
Port       State       Service
21/tcp     open        ftp
22/tcp     open        ssh
53/tcp     open        domain
80/tcp     open        http
111/tcp    open        sunrpc
2401/tcp   open        cvspserver

Nmap done: 1 IP address (1 host up) scanned in 19.01 seconds

```

The first scan shows numerous filtered ports, including
frequently exploitable services such as SunRPC, Windows NetBIOS, and
NFS. Yet scanning the same host with IPv6 shows no filtered ports!
Suddenly SunRPC (port 111)[]()is available, and waiting to be queried by an IPv6-enabled**rpcinfo**[]()or by Nmap version
detection, which supports IPv6. They fixed the issue shortly after I
notified them of it.

In order to perform an IPv6 scan, a system must be configured
for IPv6. It must have an IPv6 address and routing information.
Since my ISPs do not provide IPv6 addresses, I use the free IPv6 tunnel
broker[]()service at [`http://www.tunnelbroker.net`](http://www.tunnelbroker.net/). Other tunnel brokers are [listed at Wikipedia](http://en.wikipedia.org/wiki/List_of_IPv6_tunnel_brokers). 6to4 tunnels are another popular,
free approach. Of course, this technique also requires that the target
use IPv6.

[]()

### IP ID Idle Scanning ###

The IP ID idle scan has a reputation for being one of the most
stealthy scan types, since no packets are sent to the target from your
real address. Open ports are inferred from the IP ID sequences of a
chosen zombie machine. A less recognized feature of idle scan is that
the results obtained are actually those you would get if the zombie
was to scan the target host directly. In a similar way that the`-g` option allows exploitation of trusted source
ports, idle scan can sometimes exploit trusted source IP addresses.
This ingenious scan type, which was originally conceived by security
researcher Antirez, is described fully in [the section called “TCP Idle Scan (`-sI`)”](https://nmap.org/book/idlescan.html).

### Multiple Ping Probes ###

A common issue when trying to scan through firewalled networks
is that dropped ping probes can lead to missed hosts. To reduce this
problem, Nmap allows a very wide variety of probes to be sent in
parallel. Hopefully at least one will get through. [Chapter 3, *Host Discovery (“Ping Scanning”)*](https://nmap.org/book/host-discovery.html) discusses these techniques in depth, including empirical data on the
best firewall-busting techniques.

### Fragmentation[]() ###

Some packet filters have trouble dealing with IP packet
fragments. They could reassemble the packets themselves, but that
requires extra resources. There is also the possibility that
fragments will take different paths, preventing reassembly. Due to
this complexity, some filters ignore all fragments, while others
automatically pass all but the first fragment. Interesting things can
happen if the first fragment is not long enough to contain the whole
TCP header, or if the second packet partially overwrites it. The
number of filtering devices vulnerable to these problems is shrinking,
though it never hurts to try.

An Nmap scan will use tiny IP fragments
if the`-f`[]()is specified. By default Nmap will include up to eight bytes of data in
each fragment, so a typical 20 or 24 byte (depending on options) TCP
packet is sent in three tiny fragments. Every instance
of `-f` adds eight to the maximum fragment data size.
So `-f -f` allows up to 16 data bytes within each
fragment. Alternatively, you can specify the `--mtu`option and give the maximum data bytes as an argument.
The `--mtu` argument must be a multiple of eight, and
cannot be combined with the `-f` option.

Some source systems defragment outgoing packets in the kernel. Linux
with the iptables[]()connection tracking module is one such example. Do a scan while a
sniffer such asWireshark[]()is running to ensure that sent packets are fragmented. If your host
OS is causing problems, try
the `--send-eth`[]()option to bypass the IP layer and send raw ethernet frames.

Fragmentation is only supported for Nmap's raw packet features,
which includes TCP and UDP port scans (except connect scan and FTP
bounce scan) and OS detection. Features such as version detection and
the Nmap Scripting Engine generally don't support fragmentation because
they rely on your host's TCP stack to communicate with target services.

Out-of-order and partially overlapping IP fragments can be
useful for Network research and exploitation, but that calls for an
even lower-level networking tool than Nmap. Nmap sends fragments in
order without any overlaps.

If a fragmented port scan gets through, a tool such as [Fragroute](http://www.monkey.org/~dugsong/fragroute/)[]()can be used to fragment other tools and exploits used to
attack the host.

### Proxies ###

[]()

Application-level proxies, particularly for the Web, have become
popular due to perceived security and network efficiency (through
caching) benefits. Like firewalls and IDS, misconfigured proxies can
cause far more security problems than they solve. The most frequent
problem is a failure to set appropriate access controls. Hundreds of
thousands of wide-open proxies[]()[]()exist on the Internet, allowing
anyone to use them as anonymous hopping points to other Internet
sites. Dozens of organizations use automated scanners to find these
open proxies and distribute the IP addresses. Occasionally the
proxies are used for arguably positive things, such as escaping the
draconian censorship imposed by the Chinese government on its
residents. This “great firewall of China” has been known to block the
New York Times web site as well as other news, political, and spiritual
sites that the government disagrees with. Unfortunately, the open
proxies are more frequently abused by more sinister folks who want to
anonymously crack into sites, commit credit card fraud, or flood the
Internet with spam.

While hosting a wide-open proxy to Internet resources can cause
numerous problems, a more serious condition is when the open proxies
allow connections back into the protected network. Administrators who
decide that internal hosts must use a proxy to access Internet
resources often inadvertently allow traffic in the opposite direction
as well. The hacker
Adrian Lamo[]()is famous for breaking into Microsoft,
Excite, Yahoo, WorldCom, the New York Times, and other large networks,
usually by exploiting this reverse-proxy technique.

Nmap does not presently offer a proxy scan-through option,
though it is high on the priority list. [the section called “SOLUTION: Hack Version Detection to Suit Custom Needs, such as Open Proxy Detection”](https://nmap.org/book/vscan-hack-it.html) discusses a way to find open proxies using
Nmap version detection. In addition, plenty
of dedicated free proxy scanners are available on Internet sites such as[Packet Storm](http://packetstormsecurity.nl/).
Lists of thousands of open proxies are widespread as well.

### MAC Address Spoofing ###

[]()[]()

Ethernet devices (including Wi-Fi) are identified by a unique
six-byte media access control (MAC) address. The first three bytes make
up an organizationally unique identifier
(OUI).[]()[]() This prefix is assigned to a
vendor by the IEEE. The vendor is then responsible for assigning the
remaining three bytes uniquely in the adapters and devices it sells.
Nmap includes a database which maps OUIs to the vendor names they are
assigned to. This helps in identifying devices while scanning a
network, though this section describes why it can't be completely
trusted. The OUI database file,`nmap-mac-prefixes`,[]()is described in [the section called “MAC Address Vendor Prefixes: `nmap-mac-prefixes`”](https://nmap.org/book/nmap-mac-prefixes.html).

While MAC addresses are pre-assigned to ethernet devices, they
can be changed with a driver on most current hardware. But since few
people change their MAC address (or even know they have one), many
networks use them for identification and authorization purposes. For
example, most wireless access points provide a configuration option
for limiting access to a certain set of MAC addresses. Similarly,
some paid or private networks will force you to authenticate or pay
after you connect using a web form. Then they will allow you access
to the rest of the network based on your MAC address. Given that it
is generally easy to sniff MAC addresses (they must be sent in every
frame sent and received), and then to spoof that MAC to gain
unauthorized access to the network, this form of access control is
rather weak. It is also only effective at the edges of a network,
since an end-host's MAC address is replaced when traversing a
router.

In addition to access control, MAC addresses are sometimes used
for accountability. Network admins will record MAC addresses when
they obtain a DHCP lease or when a new machine communicates on the
network. If network abuse or piracy complaints are received later,
they figure out the MAC address based on the IP address and incident
time. Then they use the MAC to track down the responsible machine and
its owner. The ease of MAC address spoofing undermines this approach
to some degree. Even when users are guilty, they may raise the
specter of MAC address spoofing to deflect responsibility.

Nmap supports MAC address spoofing with the`--spoof-mac` option.[]()The argument given can take several forms. If it is simply the number `0`, Nmap chooses a completely random MAC address for the session. If the given string is an even number of hex digits (with the pairs optionally separated by a colon), Nmap will use those as the MAC. If fewer than 12 hex digits are provided, Nmap fills in the remainder of the six bytes with random values. If the argument isn't a zero or hex string, Nmap looks through `nmap-mac-prefixes` to find a vendor name containing the given string (it is case insensitive). If a match is found, Nmap uses the vendor's OUI and fills out the remaining three bytes randomly. Valid `--spoof-mac` argument examples are `Apple`, `0`, `01:02:03:04:05:06`, `deadbeefcafe`, `0020F2`, and `Cisco`. This option implies `--send-eth`[]() to ensure that Nmap actually sends ethernet-level packets. This option only affects raw packet scans such as SYN scan or OS detection, not connection-oriented features such as version detection or the Nmap Scripting Engine.

Even when MAC address spoofing isn't needed for network access,
it can be used for deception. If I'm at a conference and launch a
scan from my Thinkpad with `--spoof-mac Apple`,
suspicious eyes may turn to the MacBook users in the room.

### Source Routing ###

[]()[]()This old-school technique is still effective in some cases.
If[]()a particular router on the path is causing you trouble, try to find a
route around it. Effectiveness of this technique is limited because
packet filtering problems usually occur on or near the target network.
Those machines are likely to either drop all source routed packets or
to be the only way into the network. Nmap supports both loose and
strict source routing using the`--ip-options` option.[]()For example, specifying `--ip-options "L 192.168.0.7
192.168.30.9"` requests that the packet be loose source routed
through those two given IP way points. Specify `S`instead of `L` for strict source routing. If you
choose strict source routing, keep in mind that you will have to
specify every single hop along the path.

For a real-life example of source routing used to evade
filtering policies on a modern network, see[the section called “A Practical Real-life Example of Firewall Subversion”](https://nmap.org/book/firewall-subversion.html#fw-subversion-example). While IPv4 source routing is
very commonly blocked, the
IPv6[]()form of source routing is much more
pervasive. An interesting article on that problem is available at[`http://lwn.net/Articles/232781/`](http://lwn.net/Articles/232781/).

If a source routed path to a target machine is discovered with
Nmap, exploitability is not limited to port scanning.[]()[]()[Ncat](https://nmap.org/ncat)can enable TCP and UDP communication over source routed paths (use the`-g`[]()option).

### FTP Bounce Scan[]() ###

While only a small percentage of FTP servers are still
vulnerable, it is worth checking all of your clients' systems
for this problem. At a minimum, it allows outside attackers to
utilize vulnerable systems to scan other parties. Worse
configurations even allow attackers to bypass the organization's
firewalls. Details and examples of this technique are provided in[the section called “TCP FTP Bounce Scan (`-b`)”](https://nmap.org/book/scan-methods-ftp-bounce-scan.html). [Example 10.9](https://nmap.org/book/firewall-subversion.html#defeating-firewalls-ftpbounce-working) shows an HP printer being
used to relay a port scan. If this printer is behind the
organization's firewall, it can be used to scan normally inaccessible
(to the attacker) internal addresses as well.

Example 10.9. Exploiting
a printer with the FTP bounce scan

[]()

```
felix~> nmap -p 22,25,135 -Pn -v -b XXX.YY.111.2 scanme.nmap.org

Starting Nmap ( https://nmap.org )
Attempting connection to ftp://anonymous:-wwwuser@@XXX.YY.111.2:21
Connected:220 JD FTP Server Ready
Login credentials accepted by ftp server!
Initiating TCP ftp bounce scan against scanme.nmap.org (64.13.134.52)
Adding open port 22/tcp
Adding open port 25/tcp
Scanned 3 ports in 12 seconds via the Bounce scan.
Nmap scan report for scanme.nmap.org (64.13.134.52)
PORT    STATE    SERVICE
22/tcp  open     ssh
25/tcp  open     smtp
135/tcp filtered msrpc

Nmap done: 1 IP address (1 host up) scanned in 21.79 seconds

```

### Take an Alternative Path ###

I hate to overuse the “think outside the box”cliché, but continually banging on the front door of a well-secured
network is not always the best approach. Look for other ways in.
Wardial their phone lines, attack subsidiaries who may have special
network access, or show up at their offices with Wi-Fi sniffing
equipment, or even sneak in and plug into a convenient ethernet jack.
Nmap works well through all of these connections. Just make sure that
your
penetration-testing contract[]()covers these methods before your
client catches you in a ninja suit grappling onto their datacenter
rooftop.

### A Practical Real-life Example of Firewall Subversion ###

[]()[]()

Now that many individual techniques for
bypassing firewall rules have been covered, it is time to put them together in a real-life penetration testing scenario. It
all started with
a [post](https://seclists.org/pen-test/2008/Mar/10)to the SecurityFocus *pen-test* list from security pro
Michael Cain.[]()He and coworker
Demetris Papapetrou[]()were penetration testing the
internal network of a large corporation and had just bypassed firewall rules meant to
prevent one VLAN from accessing another. I was pleased to read that
they performed this feat using Nmap, and I wrote them for the whole
story. It is both instructional and inspirational in that it
demonstrates the value of perseverance and trying every technique you
know, even after the most common exploits fail. Don't let that
firewall beat you!

The story starts with Michael and Demetris performing an
Nmap scan which shows that they are
stuck on a heavily filtered network. They can reach some
corporate servers, but not any of the (potentially vulnerable) desktop
client machines which have to exist somewhere on the network. Perhaps they are on a
restricted conference room or lobby network, or maybe a wireless
access point set up for corporate guests. Some of the discovered hosts and networks
are shown in [Example 10.10](https://nmap.org/book/firewall-subversion.html#fw-megacorp-layout). A few details in this
story (such as IP addresses) have been changed for confidentiality
reasons. I will call the target corporation Megacorp.

Example 10.10. Some interesting hosts and networks at Megacorp

[]()

```
10.10.5.1  - A router/firewall which will give us grief later
10.10.5.42 - Our protagonists are scanning from this machine
10.10.6.30 - files2.megacorp.com; Nmap shows this is a Windows machine
             with port 445 open.
10.10.6.60 - mail.megacorp.com; Nmap OS detection shows that it is
             Solaris 8. Port 25 is open and accessible.
10.10.10.0/24 - Nothing shows up here, but many of the IPs have
                reverse-DNS names, so Demetris suspects that a
                firewall may be blocking his probes.  The goal is to
                reach any available hosts on this subnet.

```

Given the goal of determining if any hosts are hiding on the
10.10.10.0/24 network, Demetris starts with a simple ping scan using
ICMP echo request queries (`-PE`). The results are
shown in [Example 10.11](https://nmap.org/book/firewall-subversion.html#fw-megacorp-pingscan).

Example 10.11. Ping scan against the target network

[]()

```
# nmap -n -sn -PE -T4 10.10.10.0/24
Starting Nmap ( https://nmap.org )
Nmap done: 256 IP addresses (0 hosts up) scanned in 26.167 seconds

```

The ping scan fails to find any responsive hosts. Demetris is understandably disappointed, but at least it makes this
section more interesting and instructive. Perhaps the network truly
is empty, but it could also be packed with vulnerable machines which
Demetris is blocked from accessing. He needs to dig deeper. In[Example 10.12](https://nmap.org/book/firewall-subversion.html#fw-megacorp-pingscan-pt), Demetris chooses one IP on
that network and performs a ping scan. He specifies the packet
tracing (`--packet-trace`) and extra verbosity
(`-vv`) options to determine what is going on at the
packet level. The reason for choosing just one IP is to avoid a
confusing flood of hundreds of packets.

Example 10.12. Packet trace against a single IP

[]()

```
# nmap -vv -n -sn -PE -T4 --packet-trace 10.10.10.7
Starting Nmap ( https://nmap.org )
SENT (0.3130s) ICMP 10.10.5.42 > 10.10.10.7 echo request (type=8/code=0)
               ttl=41 id=7193 iplen=28
RCVD (0.3130s) ICMP 10.10.5.1 > 10.10.5.42 host 10.10.10.7 unreachable
               (type=3/code=1) ttl=255 id=25980 iplen=56
Nmap done: 1 IP address (0 hosts up) scanned in 0.313 seconds

```

It seems that Demetris is receiving ICMP host unreachable
messages when trying to scan these IPs (or at least this one). Routers
commonly do that when a host is unavailable and so they can't determine
a MAC address. It is also occasionally caused by filtering.
Demetris scans the other hosts on the network and verifies that they
behave the same way. It is possible that only ICMP packets are
filtered, so Demetris decides to try a TCP SYN scan. He runs the
command **nmap -vv -n -sS -T4 -Pn --reason
10.10.10.0/24**.[]()All ports are shown as filtered, and
the `--reason` results blame some host unreachable
messages and some nonresponsive ports. The nonresponsive ports may
be due to rate limiting of
host unreachable[]()messages sent by the
router. Many routers will only send one of these every few seconds.
Demetris can verify whether rate limiting is the cause by running the
scan again and seeing if the host unreachable messages come for
exactly the same set of ports. If the ports are the same, it may be a
specific port-based filter. If Nmap receives host-unreachable
messages for different ports each time, rate limiting is likely the
cause.

If a filter is causing the problem, it could be a simple
stateless firewall as is commonly available on routers and switches.
As discussed in previous sections, these sometimes allow TCP ACK
packets through unmolested. Demetris repeats the scan, but
specifies `-sA` for an ACK scan rather
than `-sS`. Any `unfiltered` ports
found by the scan would suggest that the ACK packets made it through
and elicited a TCP RST response from the target host. Unfortunately,
the results were all `filtered` in this case, just as
with the SYN scan.

Demetris decides to try something more advanced. He already
knows that port 445 is open on the Windows machine at 10.10.6.30
(files2.megacorp.com) from his initial Nmap scan. While Demetris
hasn't been able to reach the 10.10.10.0/24 network directly, perhaps files2 (being
an important company file server) is able to access that IP range.
Demetris decides to try bouncing his scans off files2 using the IPID Idle
scan. First he wants to ensure that files2 works as a zombie by
testing it against 10.10.6.60—a known-responsive machine with port 25 open. The results of this test are shown in [Example 10.13](https://nmap.org/book/firewall-subversion.html#fw-megacorp-idlescan).

Example 10.13. Testing an idle scan

[]()[]()

```
# nmap -vv -n -Pn -sI 10.10.6.30:445 -p 25 10.10.6.60

Starting Nmap ( https://nmap.org )

Initiating idle scan against 10.10.6.60 at 13:10
Idle scan using zombie 10.10.6.30 (10.10.6.30:445); Class: Incremental
Even though your Zombie (10.10.6.30) appears to be vulnerable to IP ID
sequence prediction (class: Incremental), our attempts have failed.  This
generally means that either the Zombie uses a separate IP ID base for each
host (like Solaris), or because you cannot spoof IP packets (perhaps your ISP
has enabled egress filtering to prevent IP spoofing), or maybe the target
network recognizes the packet source as bogus and drops them
QUITTING!

```

Using 10.10.6.30 as an Idle Zombie didn't work out well. If the
problem was due to heavy traffic, he could try again in the middle of
the night. The `--packet-trace` option combined with
thorough reading of [the section called “TCP Idle Scan (`-sI`)”](https://nmap.org/book/idlescan.html) could
help determine why 10.10.6.30 isn't working as a zombie. Demetris
tries the handful of other hosts he has found on the network, and none
work as zombies.

Demetris begins to worry about whether he will ever crack into
the 10.10.10.0/24 network. Fortunately, he is an old hand at
this and has another trick up his sleeve—IP
source routing.[]()In
the early days of the Internet (and even today with IPv6), source
routing was an important and widely deployed network diagnosis feature.
It allows you to specify the hops you want a packet to
take to its target rather than relying on normal routing rules. With
strict source routing, you must specify every hop. Loose
source routing allows you to fill in key IP way points, while normal
Internet routing fills in hop details between those way points.

Long ago the networking community reached consensus that source
routing is more trouble (particularly for security) than it is
worth. Many (if not most) routers are configured to drop source
routed IPv4 packets, so some folks have considered the problem fixed
since the early 90's. Yet source routing, like SYN flooding and
Telnet password sniffing, continues as a rare but potent
risk. Demetris tests this attack by ping-scanning files2
(10.10.6.30) using packets loose-source-routed through the 10.10.6.60 mail server.
Results are shown in [Example 10.14](https://nmap.org/book/firewall-subversion.html#fw-megacorp-source-route).

Example 10.14. Testing source routing

[]()[]()[]()

```
# nmap -n -sn -PE --ip-options "L 10.10.6.60" --reason 10.10.6.30

Starting Nmap ( https://nmap.org )
Host 10.10.6.30 appears to be up, received echo-reply.
Nmap done: 1 IP address (1 host up) scanned in .313 seconds

```

Demetris is both surprised and delighted that the test works. He immediately turns his attention to his true target network, repeating his initial ping scan with an additional option: `--ip-options "L 10.10.6.60"`. This time, Nmap reports that the machine at 10.10.10.7 is responsive. Demetris learns that it wasn't reachable before because the 10.10.10.0/24 and 10.10.5.0/24 subnets are on different router VLANs configured to prevent them from communicating to each other. Demetris' source routing technique opened a big loophole in that policy! Demetris follows up with a SYN scan of the 10.10.10.7 machine, as shown in [Example 10.15](https://nmap.org/book/firewall-subversion.html#fw-megacorp-source-route-success).

Example 10.15. Success at last

```
# nmap -vv -n -sS -Pn --ip-options "L 10.10.6.60" --reason 10.10.10.7

Starting Nmap ( https://nmap.org )
Nmap scan report for 10.10.10.7
Not shown: 988 closed ports
Reason: 988 resets
PORT     STATE    SERVICE              REASON
21/tcp   filtered ftp                  no-response
23/tcp   filtered telnet               no-response
25/tcp   open     smtp                 syn-ack
80/tcp   open     http                 syn-ack
135/tcp  open     msrpc                syn-ack
139/tcp  open     netbios-ssn          syn-ack
443/tcp  open     https                syn-ack
445/tcp  open     microsoft-ds         syn-ack
515/tcp  open     printer              syn-ack
1032/tcp open     iad3                 syn-ack
1050/tcp open     java-or-OTGfileshare syn-ack
3372/tcp open     msdtc                syn-ack
Nmap done: 1 IP address (1 host up) scanned in 21.203 seconds

```

Demetris omitted OS detection and version detection
from this initial scan, but this looks like a Windows machine from the open
port profile. Demetris can now connect to and access these ports as
long as he uses tools such as Ncat[]()which offer source routing
options. I don't know what happens next in the story, but I'm
guessing that it involves Demetris fully penetrating the network
and then helping the company redesign it more securely.

[]()

---

[Prev](https://nmap.org/book/determining-firewall-rules.html)Determining Firewall Rules

[Up](https://nmap.org/book/firewalls.html)Chapter 10. Detecting and Subverting Firewalls and Intrusion Detection Systems

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/subvert-ids.html)Subverting Intrusion Detection Systems