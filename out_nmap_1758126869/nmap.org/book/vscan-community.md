---
title: "Community Contributions | Nmap Network Scanning"
source_url: https://nmap.org/book/vscan-community.html
fetched_at: 2025-09-17T16:42:17.856382+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 7. Service and Application Version Detection](https://nmap.org/book/vscan.html)
* Community Contributions

[Prev](https://nmap.org/book/vscan-fileformat.html)

[Next](https://nmap.org/book/vscan-find-service-fast.html)

Community Contributions
----------

No matter how technically advanced a service detection framework
is, it would be nearly useless without a comprehensive
database of services against which to match. This is where the
open source[]()nature of Nmap really shines.
The Insecure.Org lab is pretty substantial by geek
standards, but it can never hope to run more than a tiny percentage of
machine types and services that are out there. Fortunately experience with OS detection
fingerprints has shown that Nmap users together run all of the common
stuff, plus a staggering array of bizarre equipment as well. The
Nmap OS fingerprint database
contains more than a thousand entries, including all sorts of switches, WAPs, VoIP phones, game consoles, Unix
boxes, Windows hosts, printers, routers, PDAs, firewalls, etc.
Version detection also supports user
submissions.[]()[]()Nmap users have contributed thousands of
services. There are three primary ways that the
Nmap community helps to make this an exceptional database: submitting service fingerprints, database corrections, and new probes.

### Submit Service Fingerprints ###

If a service responds to
one or more of Nmap's probes and yet Nmap is unable to identify that
service, Nmap prints a *service fingerprint*[]() like this one:

```
SF-Port21-TCP:V=3.40PVT16%D=9/6%Time=3F5A961C%r(NULL,3F,"220\x20stage\x20F
SF:TP\x20server\x20\(Version\x202\.1WU\(1\)\+SCO-2\.6\.1\+-sec\)\x20ready\
SF:.\r\n")%r(GenericLines,81,"220\x20stage\x20FTP\x20server\x20\(Version\x
SF:202\.1WU\(1\)\+SCO-2\.6\.1\+-sec\)\x20ready\.\r\n500\x20'':\x20command\
SF:x20not\x20understood\.\r\n500\x20'':\x20command\x20not\x20understood\.\
SF:r\n");

```

If you receive such a fingerprint, and are sure you know what
daemon version is running on the target host, please submit the
fingerprint at the URL Nmap gives you. The whole submission process
is anonymous (unless you choose to provide identifying info) and
should not take more than a couple minutes. If you are feeling
particularly helpful, scan the system again using `-d` (Nmap sometimes
gives longer fingerprints that way) and paste both fingerprints into
the fingerprint box on the submission form. Sometimes people read the
file format section and submit their own
working match lines. This is OK, but please submit the service
fingerprint(s) as well because existing scripts make integrating and
testing them relatively easy.

For those who care, the information in the fingerprint above is
port number (21), protocol (TCP), Nmap version (3.40PVT16), date
(September 6), Unix time in hex, and a sequence of probe responses in
the form r({*`<probename>`*}, {*`<responselength>`*},
"{*`<responsestring>`*}").

### Submit Database Corrections ###

[]()

This is another easy way to help improve the database. When
integrating a service fingerprint submitted for “chargen on
Windows XP” or “FooBar FTP server 3.9.213”, it is
difficult to determine how general the match is. Will it also match
chargen on Solaris or FooBar FTP 2.7? Since there is no good way to
tell, a very specific name is used in the hope that people will report
when the match needs to be generalized. The only reason the Nmap DB is
so comprehensive is that thousands of users have spent a few minutes
each to submit new information. If you scan a host and the service
fingerprint gives an incorrect OS, version number, application name,
or even service type, please let us know as described below:

Upgrade to the latest Nmap (Optional)

Many Linux distributions and other operating systems
ship with ancient versions of Nmap. The Nmap version detection
database is improved with almost every release, so check your version
number by running **nmap -V** and then compare that to
the latest available from [`https://nmap.org/download.html`](https://nmap.org/download.html). The problem you are
seeing may have already been corrected. Installing the newest version
takes only a few minutes on most platforms, and is valuable regardless
of whether the version detection flaw you are reporting still exists.
But even if you don't have time to upgrade right now, submissions from
older releases are still valuable.

Be absolutely certain you know what is running

Invalid “corrections” can corrupt the
version detection DB. If you aren't certain exactly what is running
on the remote machine, please find out before submitting.

Generate a fingerprint

Run the command **nmap -O -Pn -sSV -T4 -d
--version-trace -p*`<port>`**`<target>`***, where*`<port>`* is the port running the misidentified
service on the *`<target>`* host. If the service
is UDP rather than TCP, substitute `-sUV` for`-sSV`.

Send us your correction

Now simply submit your correction to us at [`https://insecure.org/cgi-bin/submit.cgi?corr-service`](https://insecure.org/cgi-bin/submit.cgi?corr-service). Thanks for contributing to the Nmap community and helping to
make version detection even better!

### Submit New Probes ###

Suppose Nmap fails to detect a service. If it received a
response to any probes at all, it should provide a fingerprint that
can be submitted as described above. But what if there is no
response and thus a fingerprint is not available? Create and submit
your own probe! These are very welcome. The following steps describe
the process.

Steps for creating a new version detection probe[]()

1. Download the latest version of Nmap from [Home page](https://nmap.org/) and try again. You would feel a bit silly spending time developing a new probe just to find out that it has already been added. Make sure no fingerprint is available, as it is better to recognize services using existing probes if possible than to create too many new ones. If the service does not respond to any of the existing probes, there is no other choice.

2. Decide on a good probe string for recognizing the service. An ideal probe should elicit a response from as many instances of the service as possible, and ideally the responses should be unique enough to differentiate between them. This step is easiest if you understand the protocol very well, so consider reading the relevant RFCs and product documentation. One simple approach is to simply start a client for the given service and watch what initial handshaking is done by sniffing the network with Wireshark[]() or tcpdump,[]() or connecting to a listening Ncat.[]()

3. Once you have decided on the proper string, add the appropriate new Probe line to Nmap (see [the section called “Technique Described”](https://nmap.org/book/vscan-technique.html) and [the section called “`nmap-service-probes` File Format”](https://nmap.org/book/vscan-fileformat.html)). Do not put in any match lines at first, although a `ports` directive to make this new test go first against the registered ports is OK. Then scan the service with Nmap a few times. You should get a fingerprint back showing the service's response to your new probe. Send the new probe line and the fingerprints (against different machines if possible, but even a few against the same daemon helps to note differences) to the Nmap development list at `<[dev@nmap.org](mailto:dev@nmap.org)>`. It will likely then be integrated into future versions of Nmap. Any details you can provide on the nature of your probe string is helpful as well. For custom services that only appear on your network, it is better to simply add them to your own `nmap-service-probes` rather than the global Nmap.

---

[Prev](https://nmap.org/book/vscan-fileformat.html)nmap-service-probes File Format

[Up](https://nmap.org/book/vscan.html)Chapter 7. Service and Application Version Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/vscan-find-service-fast.html)SOLUTION: Find All Servers Running an Insecure or Nonstandard Application Version