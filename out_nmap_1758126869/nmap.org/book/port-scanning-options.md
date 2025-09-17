---
title: "Command-line Flags | Nmap Network Scanning"
source_url: https://nmap.org/book/port-scanning-options.html
fetched_at: 2025-09-17T16:40:03.060765+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 4. Port Scanning Overview](https://nmap.org/book/port-scanning.html)
* Command-line Flags

[Prev](https://nmap.org/book/port-scanning-tutorial.html)

[Next](https://nmap.org/book/port-scanning-ipv6.html)

Command-line Flags
----------

While the tutorial showed how simple executing an Nmap port scan
can be, dozens of command-line flags are available to make the system
more powerful and flexible. This section covers only options that
relate to port scans, and often describes only the
port-scanning-related functionality of those options. See [Chapter 15, *Nmap Reference Guide*](https://nmap.org/book/man.html) for a comprehensive list of option flags and
everything they do.

### Selecting Scan Techniques ###

One of the first considerations when contemplating a port scan
is deciding what techniques to use. Nmap offers about a dozen such
methods and this section provides a brief summary of them. Full
coverage comes in the next chapter. Only one scan method may be used
at a time, except that UDP scan (`-sU`) may be combined
with any one of the TCP scan types. As a memory aid, port scan type
options are of the form`-s*`<C>`*`, where*`<C>`* is a prominent character in the scan
name, usually the first. The one exception to this is the deprecated
FTP bounce scan (`-b`).[]()[]()By default, Nmap performs a
SYN Scan, though it substitutes a connect scan if the user does not
have proper privileges to send
raw packets[]()(requires root access[]()on Unix) or if
IPv6[]()targets were specified.

Port scanning methods supported by Nmap

[]()[]()[the section called “TCP SYN (Stealth) Scan (`-sS`)”](https://nmap.org/book/synscan.html) (`-sS`)

This is far and away the most popular scan type because it the fastest way to scan ports of the most popular protocol (TCP). It is stealthier than connect scan, and it works against all functional TCP stacks (unlike some special-purpose scans such as FIN scan).

[]()[]()[the section called “TCP Connect Scan (`-sT`)”](https://nmap.org/book/scan-methods-connect-scan.html) (`-sT`)

Connect scan uses the system call of the same name to scan machines, rather than relying on raw packets as most of the other methods do. It is usually used by unprivileged Unix users and against IPv6 targets because SYN scan doesn't work in those cases.

[]()[]()[the section called “UDP Scan (`-sU`)”](https://nmap.org/book/scan-methods-udp-scan.html) (`-sU`)

Don't forget UDP ports—they offer plenty of
security holes too.

[]()[]()[]()[]()[]()[]()[the section called “TCP FIN, NULL, and Xmas Scans (`-sF`, `-sN`, `-sX`)”](https://nmap.org/book/scan-methods-null-fin-xmas-scan.html) (`-sF`, `-sX`, `-sN`)

These special purpose scan types are adept at sneaking past firewalls to explore the systems behind them. Unfortunately they rely on target behavior that some systems (particularly Windows variants) don't exhibit.

[]()[]()[the section called “TCP ACK Scan (`-sA`)”](https://nmap.org/book/scan-methods-ack-scan.html) (`-sA`)

 ACK scan is commonly used to map out firewall rulesets. In particular, it helps understand whether firewall rules are stateful or not. The downside is that it cannot distinguish open from closed ports.

[]()[]()[the section called “TCP Window Scan (`-sW`)”](https://nmap.org/book/scan-methods-window-scan.html) (`-sW`)

 Window scan is like ACK scan, except that it is able to detect open versus closed ports against certain machines.

[]()[]()[the section called “TCP Maimon Scan (`-sM`)”](https://nmap.org/book/scan-methods-maimon-scan.html) (`-sM`)

This obscure firewall-evading scan type is
similar to a FIN scan, but includes the ACK flag as well. This allows
it to get by more packet filtering firewalls, with the downside that
it works against even fewer systems than FIN scan does.

[]()[]()[the section called “TCP Idle Scan (`-sI`)”](https://nmap.org/book/idlescan.html) (`-sI *`<zombie host>`*`)

Idle scan is the stealthiest scan type of all, and can sometimes exploit trusted IP address relationships. Unfortunately, it is also slow and complex.

[]()[]()[]()[the section called “IP Protocol Scan (`-sO`)”](https://nmap.org/book/scan-methods-ip-protocol-scan.html) (`-sO`)

Protocol scan determines which IP protocols (TCP, ICMP, IGMP, etc.)
are supported by the target machine. This isn't technically a port
scan, since it cycles through IP protocol numbers rather than TCP or
UDP port numbers. Yet it still uses the `-p` option to
select scanned protocol numbers, reports its results with the normal
port table format, and even uses the same underlying scan engine as
the true port scanning methods. So it is close enough to a port scan
that it belongs here.

[]()[]()[the section called “TCP FTP Bounce Scan (`-b`)”](https://nmap.org/book/scan-methods-ftp-bounce-scan.html) (`-b *`<FTP bounce proxy>`*`)

This deprecated scan type tricks FTP servers into performing port scans by proxy. Most FTP servers are now patched to prevent this, but it is a good way to sneak through restrictive firewalls when it works.

### Selecting Ports to Scan ###

[]()

Nmap's port registration file
(`nmap-services`)[]()contains empirical data about how
frequently each TCP or UDP port is found to be open. This data was
collected by scanning tens of millions of Internet addresses, then
combining those results with internal scan data contributed by large
enterprises. By default, Nmap scans the 1,000 most popular ports of
each protocol it is asked to scan. Alternatively, you can specify
the[]() `-F` (fast)
option to scan only the 100 most common ports in each protocol
or `--top-ports` to specify an arbitrary number of ports to
scan.

When none of these canned port sets suit your needs, an arbitrary list of port numbers can be specified on the command-line with the`-p` option. The syntax of the `-p`option can be complex, and is best described with examples.

[]()

Port selection examples with the `-p` option

[]()[`-p 22`

Scan a single port (in this case port 22) by
specifying just that number as the `-p`argument.

`-p ssh`

Port names may be specified rather than numbers. Note that a name may match multiple ports.

`-p 22,25,80`

Multiple ports may be separated with commas. Note
that no protocol is specified, so these same port numbers will be used for whatever scan methods are specified on the command-line. If a TCP scan such as
SYN scan (`-sS`) is specified, TCP ports 22, 25, and 80
are scanned. Those correspond to the services SSH, SMTP, and
HTTP, respectively. If a UDP scan is selected (`-sU`),
those three UDP ports are scanned. If both are specified, those
three ports are scanned for each protocol, for a total of six
scanned ports. With IP protocol scan (`-sO`), those
three IP protocols (corresponding to XNS IDP, Leaf-1, and ISO-IP) are
scanned.

`-p80-85,443,8000-8005,8080-8085`

Port ranges may be specified by separating the
beginning and end port with a hyphen. Multiple ranges or individual
ports can be specified with commas. This option scans ports 80, 81,
82, 83, 84, 85, 443, 8000, etc. Based on the port numbers, this user is
probably scanning TCP and looking for web
servers.

`-p-100,60000-`

You can omit the beginning of a range to imply port one, or the end to imply the last port possible (65535 for TCP and UDP, 255 for protocol scan). This example scans ports one through 100, and all ports greater or equal to 60,000.

`-p-`

Omit beginning and end numbers to scan the whole range (excluding zero).

`-pT:21,23,110,U:53,111,137,161`

Separate lists of TCP and UDP ports can be given by
preceding the lists with T: (for TCP) or U:. This example scans
three TCP ports (FTP, Telnet, and POP3), and four UDP services (DNS,
rpcbind, NetBIOS, and SNMP). Specifying both TCP and UDP ports only
matters if you also tell Nmap to do a UDP scan (`-sU`)
and one of the TCP scan methods, such as `-sS`,`-sA`, or`-sF`.

`-p http*`]()

### [Timing-related Options]() ###

[Port scanning is often the most time consuming part of an Nmap
scan (which might also include OS detection, version detection, and
NSE scripts). While Nmap tries to be quick and efficient by default,
manual optimization often helps. Nmap offers dozens of options for
tailoring scan intensity and speed to match your exact needs. This
section lists the most important options for optimizing port scan
times. Options which take an amount of time are in seconds by default, or you may append `ms` (milliseconds), `s` (seconds), `m` (minutes), or `h` (hours) to the value. For further details on any of these options, see]()[the section called “Timing and Performance”](https://nmap.org/book/man-performance.html). A much more thorough treatment,
with examples and best-practices for improving Nmap performance is
available in [Chapter 6, *Optimizing Nmap Performance*](https://nmap.org/book/performance.html).

Top port scan performance options

[`-T0` through `-T5`]()

[These timing templates affect many variables, offering a simple way to adjust overall Nmap speed from very slow (`-T0`) to extremely aggressive ( `-T5`). A timing template may be combined with the more granular options describe below, and the most granular option takes precedence. ]()

[]()[ ]()[ ]()[ `--min-rtt-timeout`, `--max-rtt-timeout`, `--initial-rtt-timeout`]()

[The minimum, maximum, and initial amount of time that Nmap will wait for a port scan probe response.]()

[]()[ `--host-timeout`]()

[Asks Nmap to give up on hosts that take more than the given amount of time to scan.]()

[]()[ ]()[ `--min-rate`, `--max-rate`]()

[Sets the floor and ceiling, respectively, to the number of probe packets Nmap sends per second.]()

[]()[ `--max-retries`]()

[Specifies the maximum number of port scan probe retransmissions to a single port.]()

[]()[ ]()[ `--min-hostgroup`, `--max-hostgroup`]()

[Sets the minimum and maximum number of hosts that Nmap will port scan in parallel.]()

[]()[ ]()[ `--min-parallelism`, `--max-parallelism`]()

[Limits the minimum or maximum number of port scan probes (across all hosts scanned concurrently) that Nmap may have outstanding.]()

[]()[ ]()[ `--scan-delay`, `--max-scan-delay`]()

[Asks Nmap to wait at least the given amount of time between sending probes to any individual host. The scan delay can grow as Nmap detects packet loss, so a maximum may be specified with `--max-scan-delay`.]()

### [Output Format and Verbosity Options]() ###

[Nmap offers the ability to write its reports in its standard
format, a simple line-oriented “grepable” format, or XML.
These reports are enabled with the `-oN` (normal),`-oG` (grepable), and `-oX` (XML)
options. Each option takes a filename, and they may be combined to
output in several formats at once. Several options are also available to
increase output verbosity. This section lists the most important output-related options and how they apply to port scanning. For further details on any of these options, see]()[the section called “Output”](https://nmap.org/book/man-output.html). A much more thorough treatment of output options and formats, with many examples, is available in [Chapter 13, *Nmap Output Formats*](https://nmap.org/book/output.html).

Top Nmap output options applicable to port scans

[`-v`]()

[Increases the verbosity level, causing Nmap to print more information about the scan in progress. Open ports are shown as they are found and completion time estimates are provided when Nmap thinks a scan will take more than a few minutes. Use it twice or more for even greater verbosity.]()

[]()[ `-d`]()

[Increases the debugging level, causing Nmap to print out details about its operation that can be useful for tracking down bugs or simply understanding how it works. Higher levels result in massive amounts of data. Using the option once sets the debugging level to one, and it is incremented for each additional `-d`. Or you may follow the `-d` with the desired level, as in `-d5`. If you don't see enough information, try a higher level. The maximum effective level is nine. If your screen is flooded with too much debugging data, reduce the level. Reducing scan intensity, such as the number of ports or targets scanned and the features used, can also help to isolate only the debug messages you want.]()

[]()[ `--packet-trace`]()

[Causes Nmap to print a summary of every packet sent or received. This is often used for debugging, but is also a valuable way for new users to understand exactly what Nmap is doing under the covers. To avoid printing thousands of lines, you may want to specify a limited number of ports to scan, such as `-p20-30`.]()

[]()[ `-oN *`<filename>`*` (normal output)]()

[Write output in Nmap's normal format to *`<filename>`*. This format is roughly the same as the standard interactive output printed by Nmap at runtime.]()

[]()[ `-oX *`<filename>`*` (XML output)]()

[Write output in Nmap's XML format to *`<filename>`*. Normal (human readable) output will still be printed to stdout]()[ unless you ask for XML to be directed there by specifying `-` as *`<filename>`*. This is the preferred format for use by scripts and programs that process Nmap results.]()

[]()[ `-oG *`<filename>`*` (grepable format output)]()

[Write output in Nmap's so-called grepable format to *`<filename>`*. This tabular format fits the output of each host on a single line, making it easy to grep for open ports, certain operating systems, application names, or other data. Normal output will still be printed to stdout unless you ask for the grepable output to be directed there by specifying `-` as *`<filename>`*. While this format works well for parsing with simple grep and awk command-lines, significant scripts and programs should use the XML output instead. The XML format contains substantial information that grepable format has no place for, and extensibility makes XML easier to update with new information without breaking tools that rely on it.]()

[]()[ `-oA *`<basename>`*` (output to all formats)]()

[ As a convenience, you may specify `-oA *`<basename>`*` to store scan results in normal, XML, and grepable formats at once. They are stored in *`<basename>`*.nmap, *`<basename>`*.xml, and *`<basename>`*.gnmap, respectively. As with most programs, you can prefix the filenames with a directory path, such as `~/nmaplogs/foocorp/` on Unix or `c:\hacking\sco` on Windows.]()

[]()[ `--resume *`<filename>`*`]()

[Resume an aborted scan by specifying the normal (`-oN`) or grepable (`-oG`) output file which was created during the ill-fated scan. Don't use any options other than `--resume`, as Nmap will use the ones specified in the output file. It then parses the file and resumes scanning (and logging to the file) at the host which the previous Nmap execution was working on when it ceased.]()

[]()[ `--append-output`]()

[Tells Nmap to append scan results to any output files specified (with arguments such as `-oN` or `-oX`) rather than overwriting them.]()

[]()[ `--open`]()

[Only show hosts that have open ports, and only show the open ports for those. Here, “open ports” are any ports that have the possibility of being open, which includes `open`, `open|filtered`, and `unfiltered`.]()

### [Firewall and IDS Evasion Options]() ###

[Nmap offers many options for sneaking past IDSs undetected or
evading firewall rules. For an overview, see]()[the section called “Firewall/IDS Evasion and Spoofing”](https://nmap.org/book/man-bypass-firewalls-ids.html). For a comprehensive look
at firewall and IDS evasion techniques, along with practical examples,
see [Chapter 10, *Detecting and Subverting Firewalls and Intrusion Detection Systems*](https://nmap.org/book/firewalls.html).

### Specifying Targets ###

To scan a single host (or a few of them), simply add their names
or IP addresses to the end of your Nmap command line. Nmap also has a
structured syntax to make scanning large networks easy. You can give
Nmap a file listing targets, or even ask Nmap to generate them
randomly. This is all described in [the section called “Specifying Target Hosts and Networks”](https://nmap.org/book/host-discovery-specify-targets.html).

### Miscellaneous Options ###

Here are some options that can be quite handy even though they
don't fit into specific categories. The descriptions focus on how
each option relates to port scanning. See the [Chapter 15, *Nmap Reference Guide*](https://nmap.org/book/man.html)for more comprehensive coverage of each option.

`-6`

Asks Nmap to scan the target using the IPv6 protocol. This process is described in [the section called “IPv6 Scanning (`-6`)”](https://nmap.org/book/port-scanning-ipv6.html).

[`-r`]()

[Nmap randomizes the port scan order by default to make detection slightly harder. The `-r` option causes them to be scanned in numerical order instead.]()

[]()[ `-Pn`]()

[Tells Nmap to skip the ping test and simply scan every target host provided. Other options for controlling host discovery are described in ]()[Chapter 3, *Host Discovery (“Ping Scanning”)*](https://nmap.org/book/host-discovery.html).

[`--reason`]()

[Adds a column to the interesting ports table which describes why Nmap classified a port as it did.]()

---

[Prev](https://nmap.org/book/port-scanning-tutorial.html)A Quick Port Scanning Tutorial

[Up](https://nmap.org/book/port-scanning.html)Chapter 4. Port Scanning Overview

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/port-scanning-ipv6.html)IPv6 Scanning (-6)