---
title: "Well Known Port List: nmap-services | Nmap Network Scanning"
source_url: https://nmap.org/book/nmap-services.html
fetched_at: 2025-09-17T16:48:00.306342+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 14. Understanding and Customizing Nmap Data Files](https://nmap.org/book/data-files.html)
* Well Known Port List: nmap-services

[Prev](https://nmap.org/book/data-files.html)

[Next](https://nmap.org/book/nmap-service-probes.html)

Well Known Port List: `nmap-services`
----------

[]()[]()

The `nmap-services` file is a registry of port
names to their corresponding number and protocol. Each entry has a
number representing how likely that port is to be found open.
Most lines have a
comment as well. Nmap ignores the comments, but users sometimes grep
for them in the file when Nmap reports an open service of a type
that the user does not recognize. [Example 14.1](https://nmap.org/book/nmap-services.html#data-files-nmap-services-file) shows a typical excerpt
from the file. Some padding whitespace has been added for readability.

Example 14.1. Excerpt from `nmap-services`

[]()

```
qotd         17/tcp    0.002346  # Quote of the Day
qotd         17/udp    0.009209  # Quote of the Day
msp          18/udp    0.000610  # Message Send Protocol
chargen      19/tcp    0.002559  # ttytst source Character Generator
chargen      19/udp    0.015865  # ttytst source Character Generator
ftp-data     20/tcp    0.001079  # File Transfer [Default Data]
ftp-data     20/udp    0.001878  # File Transfer [Default Data]
ftp          21/tcp    0.197667  # File Transfer [Control]
ftp          21/udp    0.004844  # File Transfer [Control]
ssh          22/tcp    0.182286  # Secure Shell Login
ssh          22/udp    0.003905  # Secure Shell Login
telnet       23/tcp    0.221265
telnet       23/udp    0.006211
priv-mail    24/tcp    0.001154  # any private mail system
priv-mail    24/udp    0.000329  # any private mail system
smtp         25/tcp    0.131314  # Simple Mail Transfer
smtp         25/udp    0.001285  # Simple Mail Transfer

```

This file was originally based off the IANA
assigned ports list[]()at [`http://www.iana.org/assignments/port-numbers`](http://www.iana.org/assignments/port-numbers),
though many other ports have been added over the years. The IANA does
not track trojans, worms and the like, yet discovering them is
important for many Nmap users.

The grammar of this file is pretty simple. There are three
whitespace-separated columns. The first is
the service name or abbreviation, as seen in the`SERVICE` column[]()of Nmap output. The second column
gives the port number and protocol, separated by a slash. That syntax
is seen in the`PORT` column[]()of Nmap output. The third column is the“port frequency”,[]()a measure of how often the port was found open during research scans of
the Internet. If omitted, the frequency is zero.
Nmap disregards anything beyond the third column, but most
lines continue with whitespace then and a pound
(‘`#`’) character, followed by a
comment.[]()Lines may be blank or contain just a pound character followed by comments.

Astute readers notice the similarity in structure between`nmap-services` and`/etc/services` (usually found at`C:\windows\system32\drivers\etc\services` on Windows).
This is no coincidence. The format was kept to allow systems
administrators to copy in any custom entries from their own`/etc/services`, or even to substitute their own
version of that file entirely. The `/etc/services`format allows a third column providing alias names for a service.
This would conflict with the third column being used for the port
frequency, so the contents of that column are ignored if they are not
numeric.

[Example 14.1](https://nmap.org/book/nmap-services.html#data-files-nmap-services-file)shows that UDP ports are often registered for
TCP-only services such as SSH and FTP. This was
inherited from the IANA, who tend to always register services for
both protocols. Having the extra entries doesn't hurt, because by
default Nmap scans ports with the highest frequencies and low-frequency
ports are simply skipped. And, though it may be unexpected, the excerpt
shows that sometimes the UDP counterparts of popular TCP ports are found
open.

Administrators sometimes change this file to reflect custom services
running on their network. For example, an online services company I
once consulted for had dozens of different custom daemons running on
high-numbered ports. Doing this allows Nmap to display results for
these ports using their proper names rather than`unknown`. Remember that if you add entries without a
port frequency figure, the frequency is taken to be zero, so the port
will not be scanned by default. Use an option like`-p [1-65535]` to ensure that all named ports are
scanned.

Similarly, a certain registered port may be frequently wrong for
a certain organization. `nmap-services` can only
handle one service name per port number and protocol combination, yet
sometimes several different types of applications end up using the
same default port number. In that case, I try to choose the most
popular one for `nmap-services`. Organizations
which commonly use another service on such a port number may change
the file accordingly.

Services specific to a single
organization should generally stay in their own`nmap-services`, but other port registrations can
benefit everyone. If you find that the default port for a major worm,
trojan, file sharing application, or other service is missing from the
latest `nmap-services`, please send it to me
(`<[fyodor@nmap.org](mailto:fyodor@nmap.org)>`) for inclusion in the next release. This helps
all users while preventing you from having to maintain and update your
own custom version of `nmap-services`.

Another common customization is to strip`nmap-services` down to only the most common,
essential services for an organization. Without a port specification,
Nmap will not scan any ports not listed in the services file, so this is
a way to limit the number of ports scanned without using a long argument
to the `-p` option. The stripped-down file should
normally be placed in a custom location accessible with the`--datadir`[]()or `--servicedb`[]()option rather than where Nmap will use it by default. Advice for
customizing these files, including ways to prevent Nmap upgrades from
wiping out your modified versions can be found in[the section called “Using Customized Data Files”](https://nmap.org/book/data-files-replacing-data-files.html).

[]()

---

[Prev](https://nmap.org/book/data-files.html)Chapter 14. Understanding and Customizing Nmap Data Files

[Up](https://nmap.org/book/data-files.html)Chapter 14. Understanding and Customizing Nmap Data Files

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nmap-service-probes.html)Version Scanning DB: nmap-service-probes