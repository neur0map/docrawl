---
title: "The Phases of an Nmap Scan | Nmap Network Scanning"
source_url: https://nmap.org/book/nmap-phases.html
fetched_at: 2025-09-17T16:38:27.807785+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 1. Getting Started with Nmap](https://nmap.org/book/intro.html)
* The Phases of an Nmap Scan

[Prev](https://nmap.org/book/nmap-overview-and-demos.html)

[Next](https://nmap.org/book/legal-issues.html)

The Phases of an Nmap Scan
----------

Now that we've seen some applications of Nmap, let's look at
what happens when an Nmap scan runs. Scans proceed in phases, with
each phase finishing before the next one begins. As you can see from
the phase descriptions below, there is far more to Nmap than just port
scanning.

**Script
pre-scanning.**[]() The Nmap Scripting Engine (NSE) uses a
collection of special-purpose scripts to gain more information about
remote systems. NSE is not executed unless you request it with
options such as `--script` or `-sC`, and
the pre-scanning phase only happens when scripts which need it are
selected. This phase is for scripts which only have to be run once
per Nmap execution rather than running separately against individual
targets. Examples include [`dhcp-discover`](https://nmap.org/nsedoc/scripts/dhcp-discover.html)and [`broadcast-dns-service-discovery`](https://nmap.org/nsedoc/scripts/broadcast-dns-service-discovery.html),
which use broadcast queries to obtain information from common network
services. NSE is fully documented in [Chapter 9](https://nmap.org/book/nse.html), and the different phases are covered in[the section called “Script Types and Phases”](https://nmap.org/book/nse-usage.html#nse-script-types).

**Target
enumeration.**[]() In this phase, Nmap researches the host
specifiers provided by the user, which may be a combination of host
DNS names, IP addresses, CIDR network notations, and more. You can
even use (`-iR`) to ask Nmap to choose your targets for
you! Nmap resolves these specifiers into a list of IPv4 or IPv6
addresses for scanning. This phase cannot be skipped since it is
essential for further scanning, but you can simplify the processing by
passing just IP addresses so Nmap doesn't have to do forward
resolution. If you pass the `-sL -n` options (list
scan with no reverse-DNS resolution), Nmap will print out the targets
and perform no further scanning. This phase is discussed in[the section called “Specifying Target Hosts and Networks”](https://nmap.org/book/host-discovery-specify-targets.html) and[the section called “List Scan (`-sL`)”](https://nmap.org/book/host-discovery-controls.html#host-discovery-list-scan).

**Host discovery (ping
scanning).**[]()Network scans usually begin by discovering which targets on the network are online
and thus worth deeper investigation. This process is called*host discovery* or *ping
scanning*. Nmap offers many host discovery techniques,
ranging from quick ARP requests to elaborate combinations of TCP,
ICMP, and other types of probes. This phase is run by default, though
you can skip it (simply assume all target IPs are online) using
the `-Pn` (no ping) option. To quit after host
discovery, specify `-sn -n`. Host discovery is the
subject of [Chapter 3](https://nmap.org/book/host-discovery.html).

**Reverse-DNS
resolution.**[]() Once Nmap has determined which hosts to
scan, it looks up the reverse-DNS names of all hosts found online by
the ping scan. Sometimes a host's name provides clues to its
function, and names make reports more readable than providing only IP
numbers. This step may be skipped with the `-n` (no
resolution) option, or expanded to cover all target IPs (even down
ones) with `-R` (resolve all). Name resolution is
covered in [the section called “DNS Resolution”](https://nmap.org/book/host-discovery-dns.html).

**Port
scanning.**[]()This is Nmap's core operation. Probes are sent, and the responses (or
non-responses) to those probes are used to classify remote ports into
states such as `open`, `closed`,
or `filtered`. That brief description doesn't begin
to encompass Nmap's many scan types, configurability of scans, and
algorithms for improving speed and accuracy. An overview of port
scanning is in [Chapter 4](https://nmap.org/book/port-scanning.html). Detailed information on algorithms and command-line options
are in [Chapter 5](https://nmap.org/book/scan-methods.html). Port
scanning is performed by default, though you can skip it with the`-sn` option and still
perform some of the later traceroute and partial Nmap Scripting Engine
phases by specifying their particular command-line options (such
as `--traceroute` and `--script`).

**Version
detection.**[]() If any ports are found to be open,
Nmap may be able to determine what server software is running on the
remote system. It does this by sending a variety of probes to the open ports and
matching any responses against a database of thousands of more than 6,500 known
service signatures. Version detection is enabled with the`-sV` option and is fully described in[Chapter 7](https://nmap.org/book/vscan.html).

**OS
detection.**[]()If requested with the `-O` option, Nmap proceeds
to OS detection. Different operating systems implement network standards
in subtly different ways. By measuring these differences it is often
possible to determine the operating system running on a remote host. Nmap
matches responses to a standard set of probes against a database of
more than a thousand known operating system responses. OS detection is covered in[Chapter 8](https://nmap.org/book/osdetect.html).

**Traceroute.**[]()Nmap contains an optimized traceroute implementation, enabled by the`--traceroute` option. It can find the network routes to
many hosts in parallel, using the best available probe packets as determined by Nmap's previous discovery phases.
Traceroute usually involves another round of reverse-DNS resolution for
the intermediate hosts. More information is found in[the section called “Host Discovery”](https://nmap.org/book/man-host-discovery.html).

**Script
scanning.**[]()Most Nmap Scripting Engine (NSE) scripts run during this main script scanning phase, rather than the prescan and postscan phases. NSE is
powered by the Lua programming language and a standard library designed
for network information gathering. Scripts running during this phase generally run once for each target host and port number that they interact with. They commonly perform tasks such as detecting service vulnerabilities, malware discovery, collecting more information from databases and other network services, and advanced version detection. NSE is not executed unless you request it with options such as `--script` or `-sC`.

**Output.** Finally, Nmap collects
all the information it has gathered and writes it to the screen or to
a file. Nmap can write output in several formats. Its default,
human-readable format (interactive format) is usually presented in
this book. Nmap also offers an XML-based output format, among others.
The ins and outs of output are the subject of [Chapter 13](https://nmap.org/book/output.html).

As already discussed, Nmap offers many options for controlling
which of these phases are run. For scans of large networks, each
phase is repeated many times since Nmap deals with the hosts in
smaller groups. It scans each group completely and outputs those
results, then moves on to the next batch of hosts.

**Script
post-scanning.**[]() After Nmap has completed its scanning and
normal output, scripts in this phase can process results and deliver
final reports and statistics. See [the section called “Script Types and Phases”](https://nmap.org/book/nse-usage.html#nse-script-types).
Nmap does not yet include any scripts in this phase, so it only
runs if the user includes and executes their own post-scanning scripts.

---

[Prev](https://nmap.org/book/nmap-overview-and-demos.html)Nmap Overview and Demonstration

[Up](https://nmap.org/book/intro.html)Chapter 1. Getting Started with Nmap

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/legal-issues.html)Legal Issues