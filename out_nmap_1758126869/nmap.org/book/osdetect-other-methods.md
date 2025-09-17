---
title: "Fingerprinting Methods Avoided by Nmap | Nmap Network Scanning"
source_url: https://nmap.org/book/osdetect-other-methods.html
fetched_at: 2025-09-17T16:42:51.809229+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 8. Remote OS Detection](https://nmap.org/book/osdetect.html)
* Fingerprinting Methods Avoided by Nmap

[Prev](https://nmap.org/book/osdetect-ipv6-methods.html)

[Next](https://nmap.org/book/osdetect-fingerprint-format.html)

Fingerprinting Methods Avoided by Nmap
----------

Nmap supports many more OS detection techniques than any other
program, and we are always interested in hearing about new ideas.
Please send them to the Nmap development list
(*nmap-dev*) for discussion.[]()However there are some methods that just aren't a good fit. This
section details some of the most interesting ones. While they aren't
supported by Nmap, some are useful in combination with Nmap to verify
findings or learn further details.

### Passive Fingerprinting ###

Passive fingerprinting[]()uses most of the same techniques as the
active fingerprinting performed by Nmap. The difference is that a
passive system simply sniffs the network, opportunistically
classifying hosts as it observes their traffic. This is more
difficult than active fingerprinting, since you have to accept
whatever communication happens rather than designing your own custom
probes. It is a valuable technique, but doesn't belong in a
fundamentally active tool such as Nmap. Fortunately, Michal Zalewski[]()[]()has written the excellent [p0f](http://lcamtuf.coredump.cx/p0f.shtml) passive OS
fingerprinting tool. He also devised a couple of the current Nmap OS
fingerprinting tests. Another option is[SinFP](http://www.gomor.org/bin/view/Sinfp) by GomoR,[]()[]()which supports both active and passive fingerprinting.

### Exploit Chronology ###

TCP/IP fingerprinting works well for distinguishing different
operating systems, but detecting different versions of the same
operating system can be troublesome. The company must change their
stack in some way we can differentiate. Fortunately, many OS vendors
regularly update their systems to comply with the latest standards.
But what about those who don't? Most of them at least get around to
fixing exploitable stack bugs eventually. And those fixes are easy to
detect remotely. First send the exploit payload, be it a land attack,
teardrop, ping of death, SYN flood, or WinNuke. Send one attack at a
time, then immediately try to contact the system again. If it is
suddenly non-responsive, you have narrowed down the OS to versions
which didn't ship with the fix.

|                                     ![[Warning]](https://nmap.org/book/images/warning.png)                                     |Warning|
|--------------------------------------------------------------------------------------------------------------------------------|-------|
|If you use<br/>denial of service (DoS)[exploits as part of your OS<br/>detection suite, remember to perform those tests last.]()|       |

### Retransmission Times ###

TCP implementations have significant leeway in exactly how long
they wait before retransmitting packets. The proof-of-concept tools
Ring and Cron-OS are available to exploit this. They send a SYN
packet to an open port, then ignore the SYN/ACK they receive rather
than acknowledging it with an ACK (to complete the connection) or a
RST (to kill it). The target host will resend the SYN/ACK several
more times, and these tools track every subsecond of the wait. While
some information can indeed be gleaned from this technique, there are
several reasons that I haven't incorporated the patch into
Nmap:

* It usually requires modifying the source host firewall rules to prevent your system from replying with a RST packet to the SYN/ACK it receives. That is hard to do in a portable way. And even if it was easy, many users don't appreciate applications mucking with their firewall rules.

* It can be very slow. The retransmissions can go on for
  several minutes. That is a long time to wait for a test that doesn't
  give all that much information in the first place.

* It can be inaccurate because packet drops and latency
  (which you have to expect in real-world environments) can lead to
  bogus results.

I have enumerated these reasons here because they also apply to
some other proposed OS detection methods. I would love to add new
tests, but they must be quick and require few packets. Messing with
host firewall is unacceptable. I try to avoid making full TCP
connections for stack fingerprinting, though that is done for OS
detection as part of the version scanning system.

### IP Fragmentation ###

IP fragmentation is a complex system and implementations are[]()riddled with bugs and inconsistencies. Possible tests could examine
how overlapping fragments are assembled or time the defragmentation
timeouts. These tests are avoided for Nmap because many firewalls and
other inline devices defragment traffic at gateways. Thus Nmap may
end up fingerprinting the firewall rather than the true destination
host. In addition, fragments are difficult to send on some operating
systems. Linux 2.6 kernels have a tendency to queue the fragments you
are trying to send and assemble them itself before
transmission.

### Open Port Patterns ###

The target host OS can often be guessed simply by looking at the
ports which are open. Microsoft Windows machines often have TCP ports
135 and 139 open. Windows 2000 and newer also listen on port 445.
Meanwhile, a machine running services on port 22 (ssh) and 631
(Internet Printing Protocol) is likely running Unix.

While this heuristic is often useful, it just isn't reliable
enough for Nmap. Combinations of ports can be obscured by firewall
rules, and most mainstream protocols are available on multiple
platforms. OpenSSH servers can
be [run on
Windows](http://sshwindows.sourceforge.net/), and the “Windows SMB” ports can be
serviced by [Samba](http://www.samba.org/) running
on a Unix machine. Port forwarding clouds the issue even further. A
machine which appears to be running Microsoft IIS might be a Unix
firewall simply forwarding port 80 to a Windows machine.

For these reasons, Nmap does not consider open port numbers
during TCP/IP stack fingerprinting. However, Nmap can use version
detection information (see [Chapter 7, *Service and Application Version Detection*](https://nmap.org/book/vscan.html)) to separately
discover operating system and device type information. By keeping the
OS detection results discovered by OS detection and version detection
separate, Nmap can gracefully handle a Checkpoint firewall which uses
TCP port forwarding to a Windows web server. The stack fingerprinting
results should be “Checkpoint Firewall-1” while version
detection should suggest that the OS is Windows. Keep in mind that
only a small fraction of version detection signatures include OS and
device type information—we can only populate these fields when
the application divulges the information or when it only runs on one
OS or device type.

### Retired Tests ###

[]()

There are some tests that were once performed by Nmap, but which have
been retired because they were found not to help in distinguishing
operating systems and only took up space in the database. Two tests in
the`IE`[]()line were removed:`DLI`[]()checked the length of the data payload in returned packets and`SI`[]()checked the ICMP sequence numbers. They were never found to vary from
the values sent. In the`U1`[]()line, the`RUL`[]()test checked the length of the UDP packet returned. It was different
from what was sent in only one case out of more than 1,700. These tests
were removed in March 2009.

Other tests were removed because they were *too*discriminating; they caused false differences to be measured that harmed
detection accuracy. Both of these had to do with the
TOS[]()(type of service) field in responses.`TOS`[]()did this for the `U1` probe and`TOSI`[]()did it for `IE`. Although the test values did
legitimately differ between operating systems, often false differences
were recorded because the TOS was modified by an intermediate host.
Better results were had overall when these tests were removed in
October 2008.

---

[Prev](https://nmap.org/book/osdetect-ipv6-methods.html)IPv6 fingerprinting

[Up](https://nmap.org/book/osdetect.html)Chapter 8. Remote OS Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/osdetect-fingerprint-format.html)Understanding an Nmap Fingerprint