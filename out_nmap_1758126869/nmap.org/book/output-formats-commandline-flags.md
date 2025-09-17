---
title: "Command-line Flags | Nmap Network Scanning"
source_url: https://nmap.org/book/output-formats-commandline-flags.html
fetched_at: 2025-09-17T16:47:15.811286+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 13. Nmap Output Formats](https://nmap.org/book/output.html)
* Command-line Flags

[Prev](https://nmap.org/book/output.html)

[Next](https://nmap.org/book/output-formats-interactive.html)

Command-line Flags
----------

As with almost all other Nmap capabilities, output behavior is
controlled by command-line flags. These flags are grouped by category
and described in the following sections.

### Controlling Output Type ###

The most fundamental output control is designating the format(s)
of output you would like. Nmap offers five types, as summarized in
the following list and fully described in later sections.

[]()

Output formats supported by Nmap

Interactive output

This is the output that Nmap sends to the standard
output stream[(stdout)]()[by default. So it has no special
command-line option. Interactive mode caters to human users reading the results
directly and it is characterized by a table of interesting ports that
is shown in dozens of examples throughout this book.]()

[Normal output (`-oN`)]()

[This is very similar to interactive output, and is
sent to the file you choose. It does differ from interactive output in
several ways, which derive from the expectation that this output will
be analyzed after the scan completes rather than interactively. So
interactive output includes messages (depending on
verbosity level specified with `-v`) such as scan
completion time estimates and open port alerts. Normal output omits
those as unnecessary once the scan completes and the final interesting
ports table is printed. This output type prints the nmap command-line
used and execution time and date on its first line.]()

[XML output (`-oX`)]()

[XML offers a stable format that is easily parsed by
software. Free XML parsers are available for all major computer
languages, including C/C++, Perl, Python, and Java. In almost all
cases that a non-trivial application interfaces with Nmap, XML is the preferred format. This chapter also discusses
how XML results can be transformed into other formats, such as
HTML reports and database tables.]()

[Grepable output (`-oG`)]()

[This simple format is easy to manipulate on the
command line with simple Unix tools such as grep, awk,
cut, and diff. Each host is listed on one line, with the tab, slash,
and comma characters used to delimit output fields. While this can be
handy for quickly grokking results, the XML format is preferred for
more significant tasks as it is more stable and
contains more information.]()

[sCRiPt KiDDi3 0utPU+ (`-oS`)]()

[This format is provided for the
l33t haXXorZ!]()[]()

[While interactive output is the default and has no associated
command-line options, the other four format options use the same
syntax. They take one argument, which is the filename that results
should be stored in. Multiple formats may be specified, but each
format may only be specified once. For example, you may wish to save
normal output for your own review while saving XML of the same scan
for programmatic analysis. You might do this with the options`-oX myscan.xml -oN myscan.nmap`. While this chapter
uses the simple names like `myscan.xml` for brevity,
more descriptive names are generally recommended. The names chosen
are a matter of personal preference, though I use long ones that
incorporate the scan date and a word or two describing the scan, placed
in a directory named after the company I'm scanning. As a
convenience, you may specify`-oA *`<basename>`*`]()[]()to store scan results in normal, XML, and grepable formats at once.
They are stored in*`<basename>`*.nmap,[]()*`<basename>`*.xml,[]()and *`<basename>`*.gnmap,[]()respectively. As with most
programs, you can prefix the filenames with a directory path, such as`~/nmaplogs/foocorp/` on Unix or`c:\hacking\sco` on Windows.

While these options save results to files, Nmap still prints
interactive output to stdout as usual. For example, the command**nmap -oX myscan.xml target** prints XML to`myscan.xml` and fills
standard output[]()with the same interactive results it would have printed if `-oX`wasn't specified at all. You can change this by passing a hyphen
character as the argument to one of the format types. This causes
Nmap to deactivate interactive output, and instead print
results in the format you specified to the standard output stream. So the
command`nmap -oX - target`[]()will send only XML output to
stdout. Serious errors may still be printed to the normal error
stream, stderr.[]()

[]()

When you specify a filename to an output format flag such as`-oN`, that file is overwritten by default. If you
prefer to keep the existing content of the file and append the new
results, specify the`--append-output`[]()option. All
output filenames specified in that Nmap execution will then be
appended to rather than clobbered. This doesn't work well for XML
(`-oX`) scan data as the resultant file generally won't
parse properly until you fix it up by hand.

Unlike some Nmap arguments, the space between the logfile option
flag (such as `-oX`) and the filename or hyphen is
mandatory. If you omit the flags and give arguments such as`-oG-` or `-oXscan.xml`, a backwards
compatibility feature of Nmap will cause the creation of*normal format* output files named`G-` and `Xscan.xml`respectively.

All of these arguments support`strftime`-like[]()conversions in the filename. `%H`, `%M`,`%S`, `%m`, `%d`,`%y`, and `%Y` are all exactly the same
as in `strftime`. `%T` is the same
as `%H%M%S`, `%R` is the same as`%H%M`, and `%D` is the same as`%m%d%y`. A `%` followed by any other
character just yields that character (`%%` gives you a
percent symbol). So `-oX 'scan-%T-%D.xml'` will use an XML
file with a name in the form of `scan-144840-121307.xml`.

### Controlling Verbosity of Output ###

[]()[]()[]()

After deciding which format(s) you wish results to be saved in, you
can decide how detailed those results should be. The first`-v` option enables verbosity with a level of one.
Specify `-v` twice for a slightly
greater effect.[]()Verbosity levels greater than two aren't useful. Most changes only
affect interactive output, and some also affect normal and script kiddie
output. The other output types are meant to be processed by machines,
so Nmap can give substantial detail by default in those formats
without fatiguing a human user. However, there are a few
changes in other modes where output size can be reduced substantially
by omitting some detail. For example, a comment line in the grepable
output that provides a list of all ports scanned is only printed in
verbose mode because it can be quite long. The following list
describes the major changes you get with at least one`-v` option.

[]()

[ Scan completion time estimates]()

[On scans that take more than a minute or two, you will see occasional updates like this in interactive output mode:]()

[`SYN Stealth Scan Timing: About 30.01% done; ETC: 16:04 (0:01:09 remaining)` ]()

[New updates are given if the estimates change significantly. All port scanning techniques except for idle scan and FTP bounce scan support completion time estimation, and so do other phases like version detection, script scanning, and traceroute.]()

[Open ports reported when discovered]()

[When verbosity is enabled, open ports are printed in interactive mode as they are discovered. They are still reported in the final interesting ports table as well. This allows users to begin investigating open ports before Nmap even completes. Open port alerts look like this:]()

[`Discovered open port 53/tcp on 64.13.134.52` ]()

[Additional warnings]()

[Nmap always prints warnings about obvious mistakes and critical problems. That standard is lowered when verbosity is enabled, allowing more warnings to be printed. There are dozens of these warnings, covering topics from targets experiencing excessive drops or extraordinarily long latency, to ports which respond to probes in unexpected ways. Rate limiting prevents these warnings from flooding the screen.]()

[Additional notes]()

[Nmap prints many extra informational notes when in verbose mode. For example, it prints out the time when each port scan is started along with the number of hosts and ports scanned. It later prints out a concluding line disclosing how long the scan took and briefly summarizing the results.]()

[Extra OS detection information]()

[With verbosity, results of the TCP ISN and IP ID sequence number predictability tests are shown. These are done as a byproduct of OS detection. With verbosity greater than one, the actual OS detection fingerprint is shown in more cases.]()

[Down hosts are printed in ping scan]()

[During a ping scan with verbosity enabled, down hosts will be printed, rather than just up ones. ]()

[]()[ Birthday wishes]()

[Nmap wishes itself a happy birthday when run in verbose mode on September 1.]()

[The changes that are usually only useful until Nmap finishes and
prints its report are only sent to interactive output mode. If you
send normal output to a file with `-oN`, that file
won't contain open port alerts or completion time estimates, though
they are still printed to
stdout.]()[]()[]()The assumption is that you will
review the file when Nmap is done and don't want a lot of extra cruft,
while you might watch Nmap's execution progress on standard output and
care about runtime progress. If you really want everything printed to
stdout sent to a file, use the output stream redirection provided by
your shell (e.g. **nmap -v scanme.nmap.org \>
scanoutput.nmap**).

The dozens of small changes contingent on verbosity (mostly
extra messages) are too numerous to cover here. They are also always
subject to change. An effective way to see them all is to unpack the
latest Nmap tarball and grep for them with a command such as**grep -A1 o.verbose \*.cc**. Representative excerpts
from the output are shown in [Example 13.2](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-ex-grep-verbose).

Example 13.2. Grepping for verbosity conditionals

```
output.cc:    if (o.verbose)
output.cc-      log_write(LOG_PLAIN, "Uptime guess: %.3f days (since %s)\n",
--
nmap.cc:  if (o.verbose)
nmap.cc-    output_ports_to_machine_parseable_output(&ports, o.TCPScan(),
                                o.UDPScan(), o.SCTPScan(), o.ipprotscan);
--
portlist.cc:  if ((state == PORT_OPEN && o.verbose) || (o.debugging > 1)) {
portlist.cc-    log_write(LOG_STDOUT, "Discovered %s port %hu/%s%s\n",
--
scan_engine.cc:    if (o.verbose && hss->sdn.delayms != olddelay)
scan_engine.cc-      log_write(LOG_PLAIN, "Increasing send delay for %s..."

```

The following two examples put all of this together.[Example 13.3](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-ex-nonverbose) shows the output of a
normal scan without the `-v` option.

Example 13.3. Interactive output without verbosity enabled

```
# nmap -T4 -A scanme.nmap.org

Starting Nmap ( https://nmap.org )
Nmap scan report for scanme.nmap.org (64.13.134.52)
Host is up (0.045s latency).
Not shown: 993 filtered ports
PORT      STATE  SERVICE VERSION
22/tcp    open   ssh     OpenSSH 4.3 (protocol 2.0)
| ssh-hostkey: 1024 60:ac:4d:51:b1:cd:85:09:12:16:92:76:1d:5d:27:6e (DSA)
|_2048 2c:22:75:60:4b:c3:3b:18:a2:97:2c:96:7e:28:dc:dd (RSA)
25/tcp    closed smtp
53/tcp    open   domain
70/tcp    closed gopher
80/tcp    open   http    Apache httpd 2.2.3 ((CentOS))
|_html-title: Go ahead and ScanMe!
| http-methods: Potentially risky methods: TRACE
|_See https://nmap.org/nsedoc/scripts/http-methods.html
113/tcp   closed auth
31337/tcp closed Elite
Device type: general purpose
Running: Linux 2.6.X
OS details: Linux 2.6.13 - 2.6.31, Linux 2.6.18
Network Distance: 13 hops

TRACEROUTE (using port 25/tcp)
HOP RTT      ADDRESS
[Cut first 10 hops for brevity]
11  44.63 ms layer42.car2.sanjose2.level3.net (4.59.4.78)
12  44.33 ms xe6-2.core1.svk.layer42.net (69.36.239.221)
13  44.59 ms scanme.nmap.org (64.13.134.52)

OS and Service detection performed. Please report any incorrect results at https://nmap.org/submit/ .
Nmap done: 1 IP address (1 host up) scanned in 22.06 seconds

```

[Example 13.4](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-ex-verbose) is the output of the
same scan with verbosity enabled. Features such as the extra OS
identification data, completion time estimates, open port alerts, and
extra informational messages are easily identified in the latter output.
This extra info is often helpful during interactive scanning, so I
always specify `-v` when scanning a single machine unless
I have a good reason not to.

Example 13.4. Interactive output with verbosity enabled

[]()

```
# nmap -v -T4 -A scanme.nmap.org

Starting Nmap ( https://nmap.org )
NSE: Loaded 49 scripts for scanning.
Initiating Ping Scan at 15:08
Scanning scanme.nmap.org (64.13.134.52) [4 ports]
Completed Ping Scan at 15:08, 0.05s elapsed (1 total hosts)
Initiating Parallel DNS resolution of 1 host. at 15:08
Completed Parallel DNS resolution of 1 host. at 15:08, 0.00s elapsed
Initiating SYN Stealth Scan at 15:08
Scanning scanme.nmap.org (64.13.134.52) [1000 ports]
Discovered open port 22/tcp on 64.13.134.52
Discovered open port 80/tcp on 64.13.134.52
Discovered open port 53/tcp on 64.13.134.52
Completed SYN Stealth Scan at 15:08, 4.77s elapsed (1000 total ports)
Initiating Service scan at 15:08
Scanning 3 services on scanme.nmap.org (64.13.134.52)
Completed Service scan at 15:08, 11.13s elapsed (3 services on 1 host)
Initiating OS detection (try #1) against scanme.nmap.org (64.13.134.52)
Initiating Traceroute at 15:08
Completed Traceroute at 15:08, 0.06s elapsed
Initiating Parallel DNS resolution of 13 hosts. at 15:08
Completed Parallel DNS resolution of 13 hosts. at 15:08, 0.00s elapsed
NSE: Script scanning 64.13.134.52.
NSE: Starting runlevel 1 (of 1) scan.
Initiating NSE at 15:08
Completed NSE at 15:08, 4.11s elapsed
Nmap scan report for scanme.nmap.org (64.13.134.52)
Host is up (0.044s latency).
Not shown: 993 filtered ports
PORT      STATE  SERVICE VERSION
22/tcp    open   ssh     OpenSSH 4.3 (protocol 2.0)
| ssh-hostkey: 1024 60:ac:4d:51:b1:cd:85:09:12:16:92:76:1d:5d:27:6e (DSA)
|_2048 2c:22:75:60:4b:c3:3b:18:a2:97:2c:96:7e:28:dc:dd (RSA)
25/tcp    closed smtp
53/tcp    open   domain
70/tcp    closed gopher
80/tcp    open   http    Apache httpd 2.2.3 ((CentOS))
| http-methods: GET HEAD POST OPTIONS TRACE
| Potentially risky methods: TRACE
|_See https://nmap.org/nsedoc/scripts/http-methods.html
|_html-title: Go ahead and ScanMe!
113/tcp   closed auth
31337/tcp closed Elite
Device type: general purpose
Running: Linux 2.6.X
OS details: Linux 2.6.13 - 2.6.31, Linux 2.6.18
Uptime guess: 23.640 days (since Thu Jun 24 23:46:34 2010)
Network Distance: 13 hops
TCP Sequence Prediction: Difficulty=206 (Good luck!)
IP ID Sequence Generation: All zeros

TRACEROUTE (using port 80/tcp)
HOP RTT      ADDRESS
[Cut first 10 hops for brevity]
11  44.09 ms layer42.car2.sanjose2.level3.net (4.59.4.78)
12  43.98 ms xe6-2.core1.svk.layer42.net (69.36.239.221)
13  44.73 ms scanme.nmap.org (64.13.134.52)

Read data files from: .
OS and Service detection performed. Please report any incorrect results at https://nmap.org/submit/ .
Nmap done: 1 IP address (1 host up) scanned in 22.28 seconds
           Raw packets sent: 2040 (91.266KB) | Rcvd: 40 (2.278KB)

```

[]()

### Enabling Debugging Output ###

[]()[]()

When even verbose mode doesn't provide sufficient data for you,
debugging is available to flood you with much more! As with the
verbosity option (`-v`), debugging is enabled with a
command-line flag (`-d`) and the debug level can be
increased by specifying it multiple times. Alternatively, you can set
a debug level by giving an argument to`-d`.[]()For
example, `-d9` sets level nine. That is the highest
effective level and will produce thousands of lines unless you run a
very simple scan with very few ports and targets.

Debugging output is useful when a bug is suspected in Nmap,
or if you are simply confused as to what Nmap is doing and why. As this
feature is mostly intended for developers, debug lines aren't always
self-explanatory. If you don't understand a line, your only recourses
are to ignore it, look it up in the source code, or request help from
the development list
(*nmap-dev*).[]()Some lines are self explanatory, but
messages become more obscure as the debug level is
increased. [Example 13.5](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-ex-sample-debugging)shows a few different debugging lines that resulted from a`-d5` scan of Scanme.

Example 13.5. Some representative debugging lines

[]()

```
Timeout vals: srtt: 27495 rttvar: 27495 to: 137475 delta -2753
              ==> srtt: 27150 rttvar: 21309 to: 112386
RCVD (15.3330s) TCP 64.13.134.52:25 > 132.239.1.115:50122 RA ttl=52
                id=0 iplen=40 seq=0 win=0 ack=4222318673
**TIMING STATS** (15.3350s): IP, probes active/freshportsleft/retry_stack/
                                        outstanding/retranwait/onbench,
                                 cwnd/ccthresh/delay, timeout/srtt/rttvar/
   Groupstats (1/1 incomplete): 83/*/*/*/*/* 82.80/75/* 100000/25254/4606
   64.13.134.52: 83/60836/0/777/316/4295 82.80/75/0 100000/26200/4223
Current sending rates: 711.88 packets / s, 31322.57 bytes / s.
Overall sending rates: 618.24 packets / s, 27202.62 bytes / s.
Discovered filtered port 10752/tcp on 64.13.134.52
Packet capture filter (device eth0): dst host 132.239.1.115 and
                                     (icmp or ((tcp or udp) and
                                     (src host 64.13.134.52)))
SCRIPT ENGINE: TCP 132.239.1.115:59045 > 64.13.134.52:53 | CLOSE

```

No full example is given here because debug logs are so long.
A scan against Scanme used 40 lines of text without verbosity
([Example 13.3, “Interactive output without verbosity enabled”](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-ex-nonverbose)), and 40 with it
([Example 13.4, “Interactive output with verbosity enabled”](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-ex-verbose)). The same
scan with `-d` instead of `-v` took 136
lines. With `-d2` it ballooned to 1,324 lines, and`-d5` output 6,391 lines! The debug option
implicitly enables verbosity, so there is no need to specify them
both.[]()

Determining the best output level for a certain debug task is a
matter of trial and error. I try a low level first to understand what
is going on, then increase it as necessary. As I learn more, I may
be able to better isolate the problem or question. I then try to
simplify the command in order to offset some increased verbiage of the
higher debug level.

Just as **grep** can be useful to identify the changes and levels
associated with verbosity, it also helps with investigating debug
output. I recommend running this command from the
nmap-*`<VERSION>`* directory in the Nmap source
tarball:

>
>
> **grep -A1 o.debugging \*.cc**
>
>

### Enabling Packet Tracing ###

[]()[]()

The `--packet-trace` option causes Nmap to print a
summary of every packet it sends and receives. This can be extremely
useful for debugging or understanding Nmap's behavior, as examples
throughout this book demonstrate. [Example 13.6](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-ex-packettrace) shows a simple ping scan of
Scanme with packet tracing enabled.

Example 13.6. Using `--packet-trace` to detail a ping scan of Scanme

[]()[]()

```
# nmap --packet-trace -n -sn scanme.nmap.org

Starting Nmap 5.35DC18 ( https://nmap.org ) at 2010-07-18 15:23 MDT
SENT (0.0130s) ICMP 132.239.1.115 > 64.13.134.52 Echo request (type=8/code=0) ttl=53 id=43882 iplen=28
SENT (0.0130s) TCP 132.239.1.115:39273 > 64.13.134.52:443 S ttl=44 id=18217 iplen=44  seq=215684135 win=1024 <mss 1460>
SENT (0.0130s) TCP 132.239.1.115:39273 > 64.13.134.52:80 A ttl=52 id=37510 iplen=40  seq=0 win=1024
SENT (0.0130s) ICMP 132.239.1.115 > 64.13.134.52 Timestamp request (type=13/code=0) ttl=52 id=54744 iplen=40
RCVD (0.0570s) TCP 64.13.134.52:80 > 132.239.1.115:39273 R ttl=56 id=0 iplen=40  seq=215684135 win=0
Nmap scan report for scanme.nmap.org (64.13.134.52)
Host is up (0.044s latency).
Nmap done: 1 IP address (1 host up) scanned in 0.07 seconds

```

Here you can see the default four-probe host discovery
combination from [the section called “Default Combination”](https://nmap.org/book/host-discovery-techniques.html#host-discovery-default).
This output shows three five extra lines caused by packet
tracing (each have been wrapped for readability). Each line contains
several fields. The first is whether a packet is sent or received by
Nmap, as abbreviated to `SENT` and`RCVD`. The next field is a time counter, providing
the elapsed time since Nmap started. The time is in seconds, and in
this case Nmap only required a tiny fraction of one. The next field
is the protocol: TCP, UDP, or ICMP. Next comes the source and
destination IP addresses, separated with a directional arrow.
For TCP or UDP packets, each IP is followed by a colon and the source
or destination port number.

The remainder of each line is protocol specific. As you can
see, ICMP provides a human-readable type if available (`Echo
request` in this case) followed by the ICMP type and code
values. The ICMP packet logs end with the IP TTL, ID, and packet length
field. TCP packets use a slightly different format after the
destination IP and port number. First comes a list of characters
representing the set TCP flags. The flag characters are`SAFRPUEC`, which stand for SYN, ACK, FIN, RST, PSH, URG,
ECE, and CWR, respectively.[]() The latter two flags are part of TCP
explicit congestion notification,[]()described in[RFC 3168](http://www.rfc-editor.org/rfc/rfc3168.txt).[]()

Because packet tracing can lead to thousands of output lines, it
helps to limit scan intensity to the minimum that still serves your
purpose. A scan of a single port on a single machine won't bury you
in data, while the output of a `--packet-trace` scan of
a whole network can be overwhelming. Packet tracing is automatically
enabled when the debug level (`-d`) is at least
three.[]()

Sometimes `--packet-trace` provides specialized
data that Nmap never shows otherwise, like TTLs. [Example 13.6, “Using `--packet-trace` to detail a ping scan of Scanme”](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-ex-packettrace) shows ICMP and TCP ping
packets sent to the target host, the target responding to the TCP ACK
packet. It is possible that the target host replied to other probes
as well—Nmap stops listening once it receives one response to a
ping scan since that is all it takes to determine that a host is
online.

### Resuming Aborted Scans ###

[]()[]()[]()[]()

Some extensive Nmap runs take a very long time—on the order
of days. Such scans don't always run to completion. Restrictions may
prevent Nmap from being run during working hours, the network could go
down, the machine Nmap is running on might suffer a planned or
unplanned reboot, or Nmap itself could crash. The administrator running Nmap
could cancel it for any other reason as well, by pressing**ctrl-C**. Restarting the whole scan from the beginning
may be undesirable. Fortunately, if normal (`-oN`) or
grepable (`-oG`) logs were kept, the user can ask Nmap
to resume scanning with the target it was working on when execution
ceased. Specify the `--resume` option and pass
the normal/grepable output file as its argument. No other arguments
are permitted, as Nmap parses the output file to use the same ones
specified previously. Simply call Nmap as **nmap --resume*`<logfilename>`***. Nmap will append
new results to the data files specified in the previous execution.

This feature does have some limitations. Resumption does not
support the XML output format because combining the two runs into one valid
XML file would be difficult. This feature only skips hosts for which all scanning was completed. If a scan was in progress against a certain target when Nmap was stopped, the `--resume` will restart scanning of that host from the beginning.

---

[Prev](https://nmap.org/book/output.html)Chapter 13. Nmap Output Formats

[Up](https://nmap.org/book/output.html)Chapter 13. Nmap Output Formats

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/output-formats-interactive.html)Interactive Output