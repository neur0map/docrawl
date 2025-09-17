---
title: "Dealing with Misidentified and Unidentified Hosts | Nmap Network Scanning"
source_url: https://nmap.org/book/osdetect-unidentified.html
fetched_at: 2025-09-17T16:43:19.557151+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 8. Remote OS Detection](https://nmap.org/book/osdetect.html)
* Dealing with Misidentified and Unidentified Hosts

[Prev](https://nmap.org/book/osdetect-guess.html)

[Next](https://nmap.org/book/osdetect-find-rogue-ap.html)

Dealing with Misidentified and Unidentified Hosts
----------

While Nmap has a huge database, it cannot detect everything.
Nmap has no chance to detect most toasters, refrigerators, chairs, or
automobiles because they have no IP stack. Yet I wouldn't rule any of
these out, given the ever-expanding list of connected devices. The Nmap
fingerprint DB includes plenty of game consoles, phones, thermometers, cameras,
interactive toys, and media players.

Having an IP address is necessary but not sufficient to guarantee a proper fingerprint. Nmap may still guess wrong or fail to produce any guess at all. Here are some suggestions for improving your results:

Upgrade to the latest Nmap

Many Linux distributions and other operating systems ship
with ancient versions of Nmap. The Nmap OS database is improved with
almost every release, so check your version number by running**nmap -V** and then compare that to the latest
available from [`https://nmap.org/download.html`](https://nmap.org/download.html). Installing the newest version takes only a few minutes on most platforms.

Scan all ports

When Nmap detects OS detection problems against a certain
host, it will issue warnings. One of the most common is:“Warning: OS detection will be MUCH less reliable because we did
not find at least 1 open and 1 closed TCP port”. It is
possible that such ports really are unavailable on the machine, but
retrying your scan with `-p-` to scan all ports may
find some that are responsive for OS detection. Doing a UDP scan
(`-sU`) too can help even more, though it will slow the
scan substantially.

Try a more aggressive guess

If Nmap says there are no matches close enough to print,
something is probably wrong. Maybe a firewall or NAT box in the way
is modifying the probe or response packets. This can cause a hybrid
situation where one group of tests look like they are from one OS,
while another set look completely different. Adding the`--osscan-guess`[may give more clues as to what is running.]()

[Scan from a different location]()

[The more network hops your packet has to go through to reach
its target, the greater the chances that a network device will modify
(or drop) the probe or response. NAT gateways, firewalls, and
especially port forwarding can confuse OS detection. If you are
scanning the IP of a load balancing device which simply redirects
packets to a diverse network of servers, it isn't even clear what the“correct” OS detection result would be.]()

[Many ISPs filter]()[traffic to “bad” ports, and others use transparent
proxies to redirect certain ports to their own servers. The port 25
or 80 you think are open on your target may actually be spoofed from
your ISP to connect to ISP proxy servers. Another behavior which can
confuse OS detection is when firewalls
spoof]()[TCP reset packets as if they are coming from the destination host. This
is particularly common from port 113
(identd).]()[Both the reset spoofing and transparent proxies
can often be detected by noticing that every machine on a target
network seems to exhibit the behavior—even those which otherwise
seem to be down. If you detect any such nonsense, be sure to exclude
these ports from your scan so they don't taint your results. You may
also want to try from a completely different network location. The
closer you are to the target, the more accurate the results will be.
In a perfect case, you would always scan the target from the same
network segment it resides on.]()

### [When Nmap Guesses Wrong]() ###

[]()

Occasionally Nmap will report an OS guess which you know is
wrong. The errors are usually minor (such as reporting a machine
running Linux 2.4.16 as “Linux kernel 2.4.8 - 2.4.15”),
but there have been reports of Nmap being completely off (such as
reporting your web server as an AppleWriter printer). When you
encounter such problems (minor or major), please report them so
everyone can benefit. The only reason the Nmap DB is so comprehensive
is that thousands of users have spent a few minutes each to submit new
information. Please follow these instructions:

Have a recent version of Nmap

Run **nmap -V** to determine which
version of Nmap you have. You don't need to be running the absolute
latest version of Nmap (though that would be ideal), but make sure
your version is 4.20 or higher because we only need second
generation OS fingerprints, not the old style produced by previous
versions. You can determine the latest available version of Nmap by
visiting [`https://nmap.org/download.html`](https://nmap.org/download.html). If
you upgrade, you might find that the mis-identification has already been
fixed.

Be absolutely certain you know what is running

Invalid “corrections” can corrupt the OS DB. If you aren't certain exactly what is running on the remote machine, please find out before submitting.

Generate a fingerprint

Run the command **nmap -O -sV -T4 -d*`<target>`***, where*`<target>`* is the misidentified system in
question. Look at the OS detection results to ensure that the
misidentification is still present. If you are scanning the target system over IPv6, add the `-6` option as well.

If the Nmap output for the host OS results says`(JUST GUESSING)`,[it is expected that results may be a little off. Don't submit a correction in this case.]()

[Otherwise, the map command should have produced results
including the line `OS fingerprint:`. Below that is
the fingerprint (a series of lines which each start with`OS:`).]()

[Check that OS detection works against other hosts]()

[Try scanning a couple other hosts on the target
network which you know have a different OS. If they aren't detected
properly, maybe there is some network obstruction between the systems
which is corrupting the packets.]()

[If you have gotten this far and are still able to submit, good for you! Please submit the information at ]()[`https://insecure.org/cgi-bin/submit.cgi?corr-os`](https://insecure.org/cgi-bin/submit.cgi?corr-os)

### When Nmap Fails to Find a Match and Prints a Fingerprint ###

[]()[]()

When Nmap detects that OS detection conditions seem ideal and yet it finds no exact matches, it will print out a message like this:

```
No OS matches for host (If you know what OS is running on it, see
https://nmap.org/submit/ ).
TCP/IP fingerprint:
OS:SCAN(V=5.05BETA1%D=8/23%OT=22%CT=1%CU=42341%PV=N%DS=0%DC=L%G=Y%TM=4A91CB
OS:90%P=i686-pc-linux-gnu)SEQ(SP=C9%GCD=1%ISR=CF%TI=Z%CI=Z%II=I%TS=A)OPS(O1
OS:=M400CST11NW5%O2=M400CST11NW5%O3=M400CNNT11NW5%O4=M400CST11NW5%O5=M400CS
OS:T11NW5%O6=M400CST11)WIN(W1=8000%W2=8000%W3=8000%W4=8000%W5=8000%W6=8000)
OS:ECN(R=Y%DF=Y%T=40%W=8018%O=M400CNNSNW5%CC=N%Q=)T1(R=Y%DF=Y%T=40%S=O%A=S+
OS:%F=AS%RD=0%Q=)T2(R=N)T3(R=Y%DF=Y%T=40%W=8000%S=O%A=S+%F=AS%O=M400CST11NW
OS:5%RD=0%Q=)T4(R=Y%DF=Y%T=40%W=0%S=A%A=Z%F=R%O=%RD=0%Q=)T5(R=Y%DF=Y%T=40%W
OS:=0%S=Z%A=S+%F=AR%O=%RD=0%Q=)T6(R=Y%DF=Y%T=40%W=0%S=A%A=Z%F=R%O=%RD=0%Q=)
OS:T7(R=Y%DF=Y%T=40%W=0%S=Z%A=S+%F=AR%O=%RD=0%Q=)U1(R=Y%DF=N%T=40%IPL=164%U
OS:N=0%RIPL=G%RID=G%RIPCK=G%RUCK=G%RUD=G)IE(R=Y%DFI=N%T=40%CD=S)

```

Please consider submitting the fingerprint so that all Nmap
users can benefit. It only takes a minute or two and it may mean you
don't need to see that ugly message again when you scan the host with
the next Nmap version! Simply visit the URL Nmap provides for
instructions.

If Nmap finds no matches and yet prints no fingerprint,
conditions were not ideal. Even if you obtain the fingerprint through
debug mode or XML output, please don't submit it unless Nmap asks you
to (as in the previous example).

### Modifying the `nmap-os-db` Database Yourself ###

[]()

People often ask about integrating a fingerprint themselves
rather than (or in addition to) submitting it to Nmap.Org. While
we don't offer detailed instructions or scripts for this, it is
certainly possible after you become intimately familiar with [the section called “Understanding an Nmap Fingerprint”](https://nmap.org/book/osdetect-fingerprint-format.html). I hope this is useful for
your purposes, but there is no need to send your own reference
fingerprint creations to us. We can only integrate raw subject
fingerprint submissions from the web form.

---

[Prev](https://nmap.org/book/osdetect-guess.html)OS Matching Algorithms

[Up](https://nmap.org/book/osdetect.html)Chapter 8. Remote OS Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/osdetect-find-rogue-ap.html)SOLUTION: Detect Rogue Wireless Access Points on an Enterprise Network