---
title: "Clever Trickery | Nmap Network Scanning"
source_url: https://nmap.org/book/nmap-defenses-trickery.html
fetched_at: 2025-09-17T16:45:21.355493+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 11. Defenses Against Nmap](https://nmap.org/book/defenses.html)
* Clever Trickery

[Prev](https://nmap.org/book/nmap-defenses-detection.html)

[Next](https://nmap.org/book/zenmap.html)

Clever Trickery
----------

Nmap, like other active probing tools, obtains its information
by sending out packets to target systems and then trying to interpret
and organize any responses into useful reports. Nmap must rely on information from
systems and networks that may be downright hostile environments. Some
administrators take offense at being scanned, and a small percentage
try to confuse or slow Nmap with active measures beyond the firewall and IDS techniques discussed
previously.

Many of these active response methods are quite clever. I would
argue that many are too clever, causing more problems than they solve.
One such problem is exploitability. Much of this custom active
response software is just a quick hack, written without careful
security consideration. For example, an administrator friend of mine
named Paul was quite proud of installing FakeBO on his machine. He
laughed at the prospect of fooling
script kiddies[]()into thinking they
found a Back Orifice infected machine to commandeer, when Paul was
really just logging their attempts. The joke was on Paul when a FakeBO
buffer overflow was discovered and an attacker used it to compromise
his box and install a real backdoor.

The other major risk common to these technologies is
displacement of time that is better spent elsewhere. Confusing
attackers can be fun and gratifying, and in some cases even hampers
attacks. In the end, however, these techniques are mostly security by
obscurity.[]()While they can still be beneficial, they aren't as important as
more resilient technologies such as firewalls and vulnerability
patching. Advanced attackers will likely see through the obfuscation
anyway, and the script kiddies and worms rarely bother with
reconnaissance. The daily attempted IIS exploits against my Apache
web server are testament to that. These techniques should be
considered only when you are already highly confident of your security
posture. Too many people use them as a substitute for truly
securing their networks.

### Hiding Services on Obscure Ports ###

[]()[]()

Occasionally administrators advocate running services on unusual
ports to make it harder for attackers to find them. In particular,
they note the frequency of single-port sweeps across their address
space from attackers seeking out a vulnerable version of some
software. Autonomous worms frequently do the same thing.

It is true that this sort of obfuscation may prevent some worms
and script kiddies from finding services, but they are rarely more
than a marginal threat to companies that quickly patch
vulnerabilities. And companies who do not patch quickly will not be
saved by this simple port obfuscation. Proponents often argue that
even more skillful attackers will fall for this. Some have even
posted to security lists that scanning all 65,536 TCP ports is
inconceivable. They are wrong. Attackers can and do scan all TCP
ports. In addition, techniques such as Nmap version detection make it
easy to determine what service is listening on an unusual port. [Example 11.1](https://nmap.org/book/nmap-defenses-trickery.html#ex-defending-against-nmap-obscureports) shows such a scan.
Notable is that it only takes eight minutes, and this is from a
slow residential aDSL line in another state. From a faster machine, the same scan takes only three
minutes. If the default state had been filtered, the scan would have
been slower but not unreasonably so. Even if a scan takes 10 or 20
minutes, an attacker does not have to sit around watching. A
targeted attack against a company can easily be left overnight, and
mass attackers may leave a scanner running for weeks, periodically
downloading the latest data files.

Example 11.1. An all-TCP-port version scan

[]()

```
# nmap -sSV -T4 -O -p0-65535 apollo.sco.com

Starting Nmap ( https://nmap.org )
Nmap scan report for apollo.sco.com (216.250.128.35)
Not shown: 65524 closed ports
PORT      STATE    SERVICE VERSION
0/tcp     filtered unknown
21/tcp    open     ftp     WU-FTPD 2.1WU(1)+SCO-2.6.1+-sec
22/tcp    open     ssh     SSH 1.2.22 (protocol 1.5)
199/tcp   open     smux?
457/tcp   open     http    NCSA httpd 1.3
615/tcp   open     http    NCSA httpd 1.5  
1035/tcp  filtered unknown
1521/tcp  open     oracle  Oracle DB Listener 2.3.4.0.0 (for SCO System V/386)
13722/tcp open     inetd   inetd exec err /usr/openv/netbackup/bin/bpjava-msvc
13782/tcp open     inetd   inetd exec err /usr/openv/netbackup/bin/bpcd
13783/tcp open     inetd   inetd exec err /usr/openv/bin/vopied
64206/tcp open     unknown
Device type: general purpose
Running: SCO UnixWare
OS details: SCO UnixWare 7.0.0 or OpenServer 5.0.4-5.0.6

Nmap done: 1 IP address (1 host up) scanned in 501.90 seconds

```

The biggest downside to this approach is a major inconvenience
to legitimate users. Some services, such as SMTP and DNS, almost
always have to run on their well-known ports for practical reasons.
Even for services such as HTTP and SSH that can be more easily
changed, doing so means that all users must remember an unusual port number
such as 52,147 whenever they connect to the service. When there are
several“hidden” services,
it is particularly difficult to remember
which is which. Using different ports on each machine becomes even
more confusing, but standardizing on unusual port mappings across the
organization reduces the purported benefit of this scheme. Attackers
may notice that SSH is always at 52,147. The end result is that
all-port Nmap scans against your servers may increase, as frustrated
legitimate users try to find where essential services are hidden.
Less savvy users may flood you with phone calls instead.

### Port Knocking[]() ###

A technique called port knocking has recently become popular as a way to hide services from potential attackers. The method is well described on the front page of [`http://www.portknocking.org/`](http://www.portknocking.org/):

>
>
> Port knocking is a method of establishing a connection to a
> networked computer that has no open ports. Before a connection is
> established, ports are opened using a port knock sequence, which is a
> series of connection attempts to closed ports. A remote host generates
> and sends an authentic knock sequence in order to manipulate the
> server's firewall rules to open one or more specific ports. These
> manipulations are mediated by a port knock daemon, running on the
> server, which monitors the firewall log file for connection attempts
> that can be translated into authentic knock sequences. Once the
> desired ports are opened, the remote host can establish a connection
> and begin a session. Another knock sequence may be used to trigger the
> closing of the port.
>
>

This method is not brand new, but it exploded in popularity in
2003 when
Martin Krzywinski[]()coined the phrase port knocking, wrote an
implementation, created the extensive web site, and wrote articles
about it for Sys Admin and Linux Journal magazines. Port knocking adds a second layer of protection to services, though
authentication is usually weaker than that provided by primary
services such as SSH. Implementations are usually subject to sniffing
and replay attacks, and often suffer from brute force and denial of service
threats as well.

The upside is a service concealment which is much stronger than
the simple and ineffective obscure ports technique described
previously. A port competently hidden through port knocking is nearly
impossible to discover using active probes such as those sent by Nmap.
On the other hand, sniffer-based systems such as intrusion detection
systems and passive network mappers trivially detect this
scheme.

Deciding whether to implement port
knocking requires an analysis of the benefits and costs applicable to the proposed implementation. Service concealment is only beneficial for a small set
of applications. The motivation is to prevent attackers from connecting to (and
exploiting) vulnerable services, while still allowing connections from
authorized users all over the world. If only certain IP addresses
need to connect, firewall restrictions limiting connections to those
specific IPs are usually a better approach. In an ideal
world, applications would securely handle authentication themselves
and there would be no need to hide them to prevent exploitation.
Unfortunately, even security-conscious programs such as SSH have
suffered numerous remotely exploitable pre-authentication flaws.
While these bugs should be fixed as soon as possible in any case, port
knocking may provide an extra window of time before a new bug is
exploited. After all, some SSH exploits spread underground long
before official patches were available. Then when a bug is announced,
even the most conscientious administrator may require several hours or
days to learn about the bug, test the fix, and locate and patch all
vulnerable instances. The response time of a home computer owner
may be even longer. After all, the vast majority of computer users do
not subscribe to Bugtraq.

The good guys are not the only ones who benefit from service
concealment. It is at least as popular (if not more so) for gray hat
and downright criminal uses. Many ISPs restrict users from running
any server daemons such as web or SSH services. Customers could hide
a personal SSH daemon or web server (only for very limited use, as the
public could not easily connect) using port knocking technology.
Similarly, my friend Tom's employer only permitted connections
from home using a Windows-only VPN client. Tom
responded by setting up a port knocking system (before it was called
that) which, upon receiving the appropriate probes, set up a reverse
SSH tunnel from his work server back to his home Linux box. This
allowed him to work from home with full access to the work network and
without having to suffer the indignities of using Windows. It is
worth re-iterating that the service provider in both the ISP and
employer examples could have detected the subterfuge using a sniffer or netflow.
Segueing into even darker uses, computer criminals frequently use
techniques like these to hide backdoors in systems that they have
compromised. Script kiddies may just leave a blatant SSH daemon or
even raw root shell listening on some high port, vulnerable to
detection by the next Nmap scan. More cautious attackers use
concealment techniques including port knocking in their backdoors and
rootkits.[]()

While the service concealment provided by this system can be
valuable, it comes with many
limitations.[]()Services intended for
public use are inappropriate, since no one is going to install a
special knock client just to visit your web site. In addition,
publicizing the access instructions would defeat the system's primary
purpose. Non-public service should usually be blocked by a firewall
rather than shielded with port knocking. When a group of people need
access, VPNs are often a better solution as they offer encryption and
user-level access control. VPNs are also built to handle real-world
networks, where packets can be dropped, duplicated, and re-ordered. A
relatively simple probe using the Portknocking.Org implementation can
require more than 30 port probes, all of which must arrive at the
destination in order. For this many probes, you will need a special
client. Using **telnet** or a web browser is too
tedious. Additionally, all firewalls in the path must allow you to
connect to these unusual ports. Given these restrictions and
hassles, using a VPN may be just as convenient.

An additional risk is that port knocking implementations are
still immature. The best-known one, written by
Martin Krzywinski,[]()warns on the download page that “this is a prototype and
includes the bare minimum to get started. Do not use this for
production environments.” Also remember that proactive scanning
to inventory your own network will be more difficult with programs
such as this installed.

Do not let this long list of limitations dissuade you from even
considering port knocking. It may be
appropriate for specific circumstances, particularly those related to
hidden backdoors or remote administration of a personal machine.

### Honeypots and Honeynets ###

[]()

An increasingly popular method for confusing attackers is to
place bait systems on a network and monitor them for attacks. These
are known as honeypots. I am a member of the [Honeynet
Project](http://www.honeynet.org/),[]()which installs networks of these for research purposes. Many
corporations have deployed these systems for corporate security
purposes, though doing so is risky. The extensive monitoring required
makes them high-maintenance and there is always a risk that attackers
will break in and use the machines to commit serious crimes. Lower
maintenance solutions, such as
Honeyd[]()described in the next section,
or even an IDS, may be more appropriate. In any case, honeypots are
designed to catch more invasive attacks than simple Nmap scans, so
they are not discussed further.

### OS Spoofing[]() ###

Several programs have been developed specifically to trick Nmap
OS detection. They manipulate the host operating system to support
custom responses to Nmap probes. In this way, a Linux PC can be made
to resemble an Apple LaserWriter printer or even a webcam. [IP Personality](http://ippersonality.sourceforge.net/),[]()released in 2000, is one of the most popular systems. It extends the
Linux Netfilter[]()[]()framework to support these shenanigans.
Unfortunately, it has not been updated since April 2002 and may not
work on kernel versions beyond 2.4.18.

Tool availability alone does not make OS spoofing a good idea.
One has to justify the effort somehow. The IP Personality FAQ avoids
the question “Why would you need this?” by responding
that “If you ask this, then you don't”. Nevertheless,
some people find it valuable enough to write and use these tools. One
reason is that specific OS information makes it easier for attackers
to infer vulnerabilities on your network, and also helps decide what
sort of exploit to run. Of course the vulnerability itself is the
real problem there, and should be fixed. Other people run this sort
of tool because they are embarrassed about the OS they run, or they
are extremely privacy conscious. If your operating system is in a
legal gray area because some company is claiming IP infringement and
filing suits against users, OS spoofing might protect against such
a nuisance suit.

One serious problem with masking a host OS this way is that it
can cause security and functionality problems. Nmap tests for several
important security properties, such as TCP initial sequence number and
IP identification number predictability. Emulating a different
system, such as a printer, may require weakening these number
sequences so that they are predictable and vulnerable to all the
attacks that implies. The obscurity gained by spoofing your operating
system fingerprint is not worth sacrificing valuable security
mechanisms. This sort of spoofing can also cripple functionality.
Many Nmap OS detection tests involve asking the system what TCP
options are supported. Pretending not to support certain options such
as timestamps and window scaling will remove the efficiency benefits
of those options. Pretending to support unavailable options can be disastrous.

 In [Example 11.2](https://nmap.org/book/nmap-defenses-trickery.html#ex-defending-against-nmap-ippersonality),
Nmap is fooled by IP Personality into believing a Linux box is really
a Sega Dreamcast game console. It is from a paper entitled [*A practical
approach for defeating Nmap OS-Fingerprinting*](https://nmap.org/misc/defeat-nmap-osdetect.html) by
David Barroso Berrueta.[]()That excellent paper includes far more examples, as well as
detailed configuration instructions. It also describes many similar
systems, with handy warnings such as “the code is not very
stable. I loaded the module and in a few moments my Linux box got
frozen.”

Example 11.2. Deceiving Nmap with IP Personality

[]()[]()

```
# nmap -sS -O -oN nmap2.log 192.168.0.19

Nmap scan report for 192.168.0.19
(The 1597 ports scanned but not shown below are in state: closed)
Port       State       Service
22/tcp     open        ssh
25/tcp     open        smtp
80/tcp     open        http
143/tcp    open        imap
Remote operating system guess: Sega Dreamcast
Nmap finished: 1 IP address (1 host up) scanned in 5.886 seconds

```

A newer and more popular program for operating system spoofing
(among other features) is [Honeyd](http://www.honeyd.org/).[]()It is actively maintained by author
Niels Provos[]()and offers several major benefits over IP
Personality. One is that it is much easier to configure. Almost 100
configuration lines were required for the Dreamcast spoofing using IP Personality, above.
Honeyd, on the other hand, simply reads the Nmap OS detection database
and emulates any OS the user chooses. (Be aware that Honeyd uses a
database from Nmap's
1st generation OS detection,[]()which was discontinued
in 2007.) Honeyd also solves the security and
functionality problems of OS spoofing by creating synthetic hosts for
the emulation. You can ask Honeyd to take over hundreds of unused IP
addresses in an organization. It responds to probes sent to those IPs
based on its configuration. This eliminates the security and
functionality risks of trying to mask a host's own TCP stack. You are
creating a bunch of synthetic hosts instead, so this does not help
obscure the OS of existing hosts. The synthetic hosts basically
constitute a low-maintenance honeynet that can be watched for attacks.
It is mostly intended for research purposes, such as using the
worldwide network of Honeyd installations to identify new worms and
track spammer activity.

As with other techniques in this section, I recommend
experimenting with OS spoofing only when completely satisfied by your
security posture. Spoofing a single OS, or even adding hundreds of
decoy Honeyd instances, is no substitute for patching vulnerable
systems. Many attackers (and especially worms) do not even bother with
OS detection before sending exploit code.

It is also worth noting that these systems are easy to detect by
skilled attackers. It is extraordinarily hard to present a convincing
facade, given all of application and TCP stack differences between
operating systems. Nobody will believe that the system in [Example 11.2, “Deceiving Nmap with IP Personality”](https://nmap.org/book/nmap-defenses-trickery.html#ex-defending-against-nmap-ippersonality) offering IMAP,
SMTP, and SSH is really a Dreamcast running its native OS. In
addition, a [bug](http://www.honeyd.org/adv.2004-01.asc) in all versions up to 0.8 allowed for simple Honeyd
identification with a single probe packet. There are also many TCP
characteristics that Honeyd cannot yet handle. Those can be used to
detect Honeyd, though Nmap does not automate this work. If Honeyd
becomes widespread, detection functionality will likely be added to
Nmap.

Deception programs such as Honeyd
are just one reason that Nmap users should interpret Nmap results
carefully and watch for inconsistencies, particularly when scanning
networks that you do not control.

### Tar Pits[]() ###

Rather than trick attackers, some people aim for just slowing
them down. Tar pits have long been popular methods for slowing
Internet worms and spammers. Some administrators use TCP techniques such as
zero-sized receive windows or slowly trickling data back byte by byte.[LaBrea](http://labrea.sourceforge.net/) is a popular[]()implementation of this. Others use application-level techniques such
as long delays before responding to SMTP commands. While these
are mostly used by anti-spammers, similar techniques can be used to
slow Nmap scans. For example,
limiting the rate[]()of RST packets sent
by closed ports can dramatically slow scanners down.

### Reactive Port Scan Detection ###

We previously discussed scan detection using tools such as
Scanlogd. Other tools go much further than that, and actually respond
to the scans. Some people propose attacking back by launching
exploits or denial of service attacks against the scan source. This
is a terrible idea for many reasons. For one, scans are often
forged.[]()If the source address is accurate, it may be a previous victim that
the attacker is using as a scapegoat. Or the scan may be part of an
Internet research survey or come from a legitimate employee or
customer. Even if the source address is a computer belonging to an
actual attacker, striking back may disrupt innocent systems and
routers along the path. It may also be illegal.

While the idea of attacking back is widely shunned in the
security community, there is much more interest in responding to
detected attacks by adjusting firewall rules to block the offending IP
address. The idea is to prevent them from following up on the scan
with an actual attack. There are several risks in this approach. One
is that you show your hand. It will be obvious to attackers that they
have been blocked, and most have plenty of other IP addresses they can
use to continue probing. They will then know about your reactive
system, and could escalate their own attacks. A more important
problem is that scans are so easily forged. [the section called “Misleading Intrusion Detection Systems”](https://nmap.org/book/subvert-ids.html#misleading-ids) describes
several methods for doing so. When an attacker notices the block, he
may spoof scans from important systems, such as major web sites and
DNS servers. A target network which then blocks those IPs will be
committing a
denial of service[]()attack on itself. Restricting firewall blocks to scans that initiate a full TCP connection reduces the spoofing problem, but that fails to stop even the default Nmap SYN scan.

### Escalating Arms Race ###

While the primary focus of this book is on open-source tools, a
number of commercial vendors have introduced products that attempt to
deceive Nmap. One example is the Cisco Security
Agent[]().
The evaluation guide claims the following protections against
Nmap.

>
>
> Network Mapper (Nmap) identifies which devices are present on a
> network and what operating system and services they are running by
> sending out a series of network probes. The presence of a device on
> the network and the ports it is running are both announced by its
> response to Nmap probes. The pattern of error messages returned
> identifies the operating system. Nmap is surprisingly accurate. It
> is frequently used at the initial stage of an attack or investigation
> to determine which systems might respond to an attacker's
> exploits.
>
>
>
> Expected outcome of Nmap scan against Cisco Security Agent
> protected systems: Nmap is unable to identify the target operating
> system of systems running the default server or default desktop
> policies. Nmap scans appear to hang while its security tests timeout.
> Nmap scans against systems not protected by Cisco Security Agent
> report results very quickly
>
>

I am investigating how CSA works, and whether Nmap can
automatically detect and adjust for it. Scanning technology is an
arms race. Open source and commercial companies will continue to
create products designed to slow down, block, or deceive Nmap and
other tools. Meanwhile, Nmap continually improves, developing
resiliency in the face of these challenges.

---

[Prev](https://nmap.org/book/nmap-defenses-detection.html)Detect Nmap Scans

[Up](https://nmap.org/book/defenses.html)Chapter 11. Defenses Against Nmap

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/zenmap.html)Chapter 12. Zenmap GUI Users' Guide