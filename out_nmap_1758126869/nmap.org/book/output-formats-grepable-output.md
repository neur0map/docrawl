---
title: "Grepable Output (-oG) | Nmap Network Scanning"
source_url: https://nmap.org/book/output-formats-grepable-output.html
fetched_at: 2025-09-17T16:47:55.883388+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 13. Nmap Output Formats](https://nmap.org/book/output.html)
* Grepable Output (-oG)

[Prev](https://nmap.org/book/output-formats-output-to-html.html)

[Next](https://nmap.org/book/data-files.html)

Grepable Output (`-oG`)
----------

[]()[]()[]()

This output format is covered last because it is deprecated.[]()The XML output format is far more powerful, and is nearly as
convenient for experienced users. XML is a standard for which dozens
of excellent parsers are available, while grepable output is my own
simple hack. XML is extensible to support new Nmap features as they
are released, while I often must omit those features from grepable
output for lack of a place to put them.

Nevertheless, grepable output is still quite popular. It is a
simple format that lists each host on one line and can be trivially
searched and parsed with standard Unix tools such as grep, awk, cut,
sed, diff, and Perl. Even I usually use it for one-off tests done at the
command line. Finding all the hosts with the SSH port open or that
are running Solaris takes only a simple grep to identify the hosts,
piped to an awk or cut command to print the desired fields. One grepable output aficionado is
Lee “MadHat” Heath,[]()who contributed to this section.

[Example 13.14](https://nmap.org/book/output-formats-grepable-output.html#output-formats-ex-grepable-scanme) shows a
typical example of grepable output. Normally each host takes only one
line, but I split this entry into seven lines to fit on the page.
There are also three lines starting with a hash prompt (not counting
the Nmap command line). Those are
comments[]()describing when Nmap
started, the command line options used, and completion time and statistics.
One of the comment lines enumerates the port numbers that were
scanned. I shortened it to avoid wasting dozens of lines. That
particular comment is only printed in verbose (`-v`)
mode. Increasing the verbosity level beyond one `-v`will not further change the grepable output. The times and dates have
been replaced with `[time]` to reduce line length.

Example 13.14. A typical example of grepable output

[]()[]()

```
# nmap -T4 -A -v -oG - scanme.nmap.org
# Nmap 5.35DC18 scan initiated [time] as: nmap -T4 -A -v -oG - scanme.nmap.org
# Ports scanned: TCP(1000;1,3-4,6-7,...,,65389) UDP(0;) SCTP(0;) PROTOCOLS(0;)
Host: 64.13.134.52 (scanme.nmap.org)    Status: Up
Host: 64.13.134.52 (scanme.nmap.org)    Ports: 22/open/tcp//ssh//OpenSSH 4.3 (protocol 2.0)/, 25/closed/tcp//smtp///, 53/open/tcp//domain///, 70/closed/tcp//gopher///, 80/open/tcp//http//Apache httpd 2.2.3 ((CentOS))/, 113/closed/tcp//auth///, 31337/closed/tcp//Elite///  Ignored State: filtered (993)   OS: Linux 2.6.13 - 2.6.31       Seq Index: 204  IP ID Seq: All zeros
# Nmap done at [time] -- 1 IP address (1 host up) scanned in 21.90 seconds

```

The command-line here requested that grepable output be sent to
standard output with the `-` argument to`-oG`. Aggressive timing (`-T4`) as
well as OS and version detection (`-A`) were requested.
The comment lines are self-explanatory, leaving the meat of grepable output
in the `Host` line. Had I scanned more hosts, each
of the available ones would have its own `Host` line.

### Grepable Output Fields ###

[]()

The host line is split into fields, each of which consist of a
field name followed by a colon and space, then the field content. The
fields are separated by tab characters (ASCII number nine, '\\t'). [Example 13.14, “A typical example of grepable output”](https://nmap.org/book/output-formats-grepable-output.html#output-formats-ex-grepable-scanme) shows six fields:`Host`,`Status`,`Ports`,`Ignored State`,`OS`,`Seq Index`, and`IP ID Seq`.
A `Protocols` section is included in IP protocol
(`-sO`)[]()scans. The exact fields given
depend on Nmap options used. For example, OS detection triggers the`OS`, `Seq Index`, and `IP ID Seq` fields. Because they are tab-delimited, you
might split up the fields with a Perl line such as:

```
@fields = split("\t", $host_line);
```

In the case of [Example 13.14, “A typical example of grepable output”](https://nmap.org/book/output-formats-grepable-output.html#output-formats-ex-grepable-scanme), the array`@fields` would contain six
members. `$fields[0]` would contain “`Host:
64.13.134.52 (scanme.nmap.org)`”, and`$fields[1]` would contain the long `Ports` field. Scripts
that parse grepable output should ignore fields they don't recognize,
as new fields may be added to support Nmap enhancements.

The eight possible fields are described in the
following sections.

#### `Host` field ####

**Example:** `Host: 64.13.134.52 (scanme.nmap.org)`

The `Host` field always comes first and is included no matter what
Nmap options are chosen. The contents are the IP address (an IPv6
address if `-6` was specified), a space, and then the
reverse DNS name in parentheses. If no reverse name is available,
the parentheses will be empty.

#### `Status` field ####

**Example:** `Status: Up`

The `Status` field indicates the whether the
target host is `Up`, `Down`, or`Unknown`.
List scan (`-sL`) always categorizes targets as`Unknown` because it
does not perform any tests. Ping scan lists a host as `Up` if it
responds to at least one ping probe, and `Down` if no responses are
received. It used to also report `Smurf` if ping probes sent to the target resulted in one
or more responses from other hosts, but that is no longer done.[Example 13.15](https://nmap.org/book/output-formats-grepable-output.html#output-formats-ex-grepable-pingscan) demonstrates a ping
scan of five random hosts, while[Example 13.16](https://nmap.org/book/output-formats-grepable-output.html#output-formats-ex-grepable-listscan) demonstrates a list
scan of five hosts.

Example 13.15. Ping scan grepable output

[]()[]()

```
# nmap -sn -oG - -iR 5
# Nmap 5.35DC18 scan initiated [time] as: nmap -sn -oG - -iR 5
Host: 93.182.218.153 () Status: Up
Host: 154.223.142.85 () Status: Down
Host: 120.128.8.97 ()   Status: Down
Host: 47.159.134.149 () Status: Down
Host: 24.172.4.19 ()    Status: Down
# Nmap done at [time] -- 5 IP addresses (1 host up) scanned in 4.25 seconds

```

Example 13.16. List scan grepable output

[]()

```
# nmap -sL -oG - -iR 5
# Nmap 5.35DC18 scan initiated [time] as: ./nmap -sL -oG - -iR 5
Host: 91.244.202.129 () Status: Unknown
Host: 216.36.141.91 (cm216036141091.wcgwave.ca) Status: Unknown
Host: 17.130.29.192 ()  Status: Unknown
Host: 45.89.164.99 ()   Status: Unknown
Host: 215.22.1.81 ()    Status: Unknown
# Nmap done at [time] -- 5 IP addresses (0 hosts up) scanned in 13.00 seconds

```

#### `Ports` field ####

**Example:** `Ports: 111/open/tcp//rpcbind (rpcbind V2)/(rpcbind:100000*2-2)/2 (rpc #100000)/, 113/closed/tcp//auth///`

The `Ports` field is by far the most complex, as can be seen in[Example 13.14, “A typical example of grepable output”](https://nmap.org/book/output-formats-grepable-output.html#output-formats-ex-grepable-scanme). It includes
entries for every interesting port (the ones which would be included
in the port table in normal Nmap output). The port entries are
separated with a comma and a space character. Each port entry
consists of seven subfields, separated by a forward slash (`/`). The subfields
are: port number, state, protocol, owner, service, SunRPC info, and
version info. Some subfields may be empty, particularly for basic port
scans without OS or version detection. The consecutive slashes in[Example 13.14, “A typical example of grepable output”](https://nmap.org/book/output-formats-grepable-output.html#output-formats-ex-grepable-scanme) reveal empty
subfields. In Perl, you might split them up as so:

```
($port, $state, $protocol, $owner, $service, $rpc_info, $version) =
        split('/', $ports);

```

Alternatively, you could grab the information from the command
line using commands such as these:

```
cut -d/ -f<fieldnumbers>
awk -F/ '{print $<fieldnumber>}'

```

Certain subfields can contain a slash in other output modes.
For example, an SSL-enabled web server would show up as`ssl/http` and the version info might contain strings
such as `mod_ssl/2.8.12`. Since a slash is the
subfield delimiter, this would screw up parsing. To avoid this
problem, slashes are changed into the pipe character (`|`) when they
would appear anywhere in the `Port` field.

Parsers should be written to allow more than seven
slash-delimited subfields and to simply ignore the extras because
future Nmap enhancements may call for new ones. The following
list describes each of the seven currently defined Port
subfields.

Port number

This is simply the numeric TCP or UDP port number.

State

The same port state which would appear in the normal output port table is shown here.

Protocol

This is `tcp`, `udp`, or `sctp`.

Owner

This used to specify the username that the remote server is running under based on results from querying an identd (auth) server of the target host. The ident scan (`-I`) is no longer available with Nmap, so this field is always empty. Ident data can still be obtained using the `auth-owners`[ NSE script, though results are not placed in this field.]()

[Service]()

[The service name, as obtained from an `nmap-services` lookup, or (more reliably) through version detection (`-sV`) if it was requested and succeeded. With version detection enabled, compound entries such as `ssl|http` and entries with a trailing question mark may be seen. The meaning is the same as for normal output, as discussed in ]()[Chapter 7, *Service and Application Version Detection*](https://nmap.org/book/vscan.html).

SunRPC info

If version detection (`-sV`) was requested and the port was found to use the SunRPC protocol, the RPC program number and accepted version numbers are included here. A typical example is `(rpcbind:100000*2-2)`. The data is always returned inside parentheses. It starts with the program name, then a colon and the program number, then an asterisk followed by the low and high supported version numbers separated by a hyphen. So in this example, rpcbind (program number 100000) is listening on the port for rpcbind version 2 requests.

Version info

If version detection is requested and succeeds, the results are provided here in the same format used in interactive output. For SunRPC ports, the RPC data is printed here too. The format for RPC results in this column is *`<low version number>`*-*`<high version number>`* (rpc #*`<rpc program number>`*). When only one version number is supported, it is printed by itself rather than as a range. A port which shows `(rpcbind:100000*2-2)` in the SunRPC info subfield would show `2 (rpc #100000)` in the version info subfield.

#### `Protocols` field ####

**Example:** `Protocols: 1/open/icmp/, 2/open|filtered/igmp/`

The IP protocol scan
(`-sO`)[]()has a `Protocols`field rather than `Ports`. Its contents are quite similar to the `Ports`field, but it has only three subfields rather than seven. They are
delimited with slashes, just as with the `Ports` field. Any slashes
that would appear in a subfield are changed into pipes (`|`), also as
done in the `Ports` field. The subfields are protocol number, state,
and protocol name. These correspond to the three fields shown in
interactive output for a protocol scan. An example of IP protocol
scan grepable output is shown in [Example 13.17](https://nmap.org/book/output-formats-grepable-output.html#output-formats-ex-grepable-protocol-scanme). The `Host` line,
which would normally be all one line, is here wrapped for
readability.

Example 13.17. Grepable output for IP protocol scan

[]()

```
# nmap -v -sO -oG - localhost
# Nmap 5.35DC18 scan initiated [time] as: nmap -v -sO -oG - localhost
# Ports scanned: TCP(0;) UDP(0;) SCTP(0;) PROTOCOLS(256;0-255)
Host: 127.0.0.1 (localhost)     Status: Up
Host: 127.0.0.1 (localhost)     Protocols: 1/open/icmp/, 2/open/igmp/, 4/open|filtered/ip/, 6/open/tcp/, 17/open/udp/, 41/open|filtered/ipv6/, 47/open|filtered/gre/, 50/open|filtered/esp/, 51/open|filtered/ah/, 108/open|filtered/ipcomp/, 132/open/sctp/, 255/open|filtered//       Ignored State: closed (244)
# Nmap done at [time] -- 1 IP address (1 host up) scanned in 2.36 seconds

```

#### `Ignored State` field ####

[]()

**Example:** `Ignored State: filtered (1658)`

To save space, Nmap may omit ports in one non-open state from
the list in the `Ports` field. Nmap does this in interactive output
too. Regular Nmap users are familiar with the lines such as`Not shown: 993 closed ports`. For grepable mode, that state is given in the`Ignored State` field. Following the state name is a space, then in
parentheses is the number of ports found in that state.

#### `OS` field ####

**Example:** `OS: Linux 2.4.0 - 2.5.20`

Any perfect OS matches are listed here. If there are multiple
matches, they are separated by a pipe character as shown in [Example 13.14, “A typical example of grepable output”](https://nmap.org/book/output-formats-grepable-output.html#output-formats-ex-grepable-scanme). Only the free-text
descriptions are provided. Grepable mode does not provide the vendor,
OS family, and device type classification shown in other output
modes.

#### `Seq Index` field ####

**Example:** `Seq Index: 3004446`

This number is an estimate of the difficulty of performing TCP
initial sequence number[]()prediction attacks against the remote host. These are also known as
blind spoofing attacks,[]()and they
allow an attacker to forge a full TCP connection to a remote host as
if it was coming from some other IP address. This can always help an
attacker hide his or her tracks, and it can lead to privilege
escalation against services such as rlogin that commonly grant extra
privileges to trusted IP addresses. The `Seq Index` value is only available when
OS detection (`-O`)[]()is requested and succeeds in
probing for this. It is reported in interactive output when verbosity
(`-v`) is requested. More details on the computation
and meaning of this value are provided in [Chapter 8, *Remote OS Detection*](https://nmap.org/book/osdetect.html).

#### `IP ID Seq` field ####

**Example:** `IP ID Seq: All zeros`

This simply describes the remote host's IP ID generation
algorithm. It is only available when OS detection
(`-O`) is requested and succeeds in probing for it.
Interactive mode reports this as well, and it is discussed in[Chapter 8, *Remote OS Detection*](https://nmap.org/book/osdetect.html).

### Parsing Grepable Output on the Command Line ###

[]()

Grepable output really shines when you want to gather
information quickly without the overhead of writing a script to parse
XML output. [Example 13.18](https://nmap.org/book/output-formats-grepable-output.html#output-formats-ex-grepable-commandline)shows a typical example of this. The goal is to find all hosts on a
class C sized network with port 80 open. Nmap is told to scan just
that port of each host (skipping the ping stage) and to output a
grepable report to stdout. The results are piped to a trivial awk
command which finds lines containing `/open/` and
outputs fields two and three for each matching line. Those fields are
the IP address and hostname (or empty parentheses if the hostname is
unavailable).

Example 13.18. Parsing grepable output on the command line

```
> nmap -p80 -Pn -oG - 10.1.1.0/24 | awk '/open/{print $2 " " $3}'
10.1.1.72 (userA.corp.foocompany.biz)
10.1.1.73 (userB.corp.foocompany.biz)
10.1.1.75 (userC.corp.foocompany.biz)
10.1.1.149 (admin.corp.foocompany.biz)
10.1.1.152 (printer.corp.foocompany.biz)
10.1.1.160 (10-1-1-160.foocompany.biz)
10.1.1.161 (10-1-1-161.foocompany.biz)
10.1.1.201 (10-1-1-201.foocompany.biz)
10.1.1.254 (10-1-1-254.foocompany.biz)

```

[]()

---

[Prev](https://nmap.org/book/output-formats-output-to-html.html)Creating HTML Reports

[Up](https://nmap.org/book/output.html)Chapter 13. Nmap Output Formats

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/data-files.html)Chapter 14. Understanding and Customizing Nmap Data Files