---
title: "Output | Nmap Network Scanning"
source_url: https://nmap.org/book/man-output.html
fetched_at: 2025-09-17T16:36:13.557879+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 15. Nmap Reference Guide](https://nmap.org/book/man.html)
* Output

[Prev](https://nmap.org/book/man-bypass-firewalls-ids.html)

[Next](https://nmap.org/book/man-misc-options.html)

Output
----------

[]()

Any security tool is only as useful as the output it generates. Complex tests and algorithms are of little value if they aren't presented in an organized and comprehensible fashion. Given the number of ways Nmap is used by people and other software, no single format can please everyone. So Nmap offers several formats, including the interactive mode for humans to read directly and XML for easy parsing by software.

In addition to offering different output formats, Nmap provides
options for controlling the verbosity of output as well as debugging
messages. Output types may be sent to standard output or to named
files, which Nmap can append to or clobber. Output files may also be
used to resume aborted scans.

Nmap makes output available in five different formats.
The default is called*interactive output*,[]()and it is sent to
standard output (stdout).[]()There is also*normal output*,[]()which is similar to interactive except that it
displays less runtime information and warnings since it is expected to
be analyzed after the scan completes rather than interactively.

*XML output*[]()is one of the most important output types, as it can
be converted to HTML, easily parsed by programs such as Nmap graphical
user interfaces, or imported into databases.

The two remaining output types are the simple*grepable output*[]()which includes most information for a target host on
a single line, and*sCRiPt KiDDi3 0utPUt*[]()for users
who consider themselves |\<-r4d.

While interactive output is the default and has no associated
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
in a directory named after the company I'm scanning.

While these options save results to files, Nmap still prints
interactive output to stdout as usual. For example, the command**nmap -oX myscan.xml target** prints XML to`myscan.xml` and fills standard output with the same interactive results it would have printed if `-oX`wasn't specified at all. You can change this by passing a hyphen
character as the argument to one of the format types. This causes
Nmap to deactivate interactive output, and instead print
results in the format you specified to the standard output stream. So the
command **nmap -oX - target** will send only XML output to
stdout.[]()Serious errors may still be printed to the normal error
stream, stderr.[]()

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

Nmap also offers options to control scan verbosity and to append
to output files rather than clobbering them. All of these options are
described below.

Nmap Output Formats

`-oN *`<filespec>`*` (normal output) []()[]()

[Requests that normal output be directed to the given filename. As discussed above, this differs slightly from `interactive output`.]()

[`-oX *`<filespec>`*` (XML output) ]()[ ]()[]()

[Requests that XML output be directed to the given filename. Nmap includes a document type definition (DTD) which allows XML parsers to validate Nmap XML output. While it is primarily intended for programmatic use, it can also help humans interpret Nmap XML output. The DTD defines the legal elements of the format, and often enumerates the attributes and values they can take on. The latest version is always available from ]()[`https://svn.nmap.org/nmap/docs/nmap.dtd`](https://svn.nmap.org/nmap/docs/nmap.dtd).

XML offers a stable format that is easily parsed by software. Free XML parsers are available for all major computer languages, including C/C++, Perl, Python, and Java. People have even written bindings for most of these languages to handle Nmap output and execution specifically. Examples are [Nmap::Scanner](http://sourceforge.net/projects/nmap-scanner/)[ and ]()[Nmap::Parser](http://nmapparser.wordpress.com/)[ in Perl CPAN. In almost all cases that a non-trivial application interfaces with Nmap, XML is the preferred format.]()

[The XML output references an XSL stylesheet which can be used to format the results as HTML. The easiest way to use this is simply to load the XML output in a web browser such as Firefox or IE. By default, this will only work on the machine you ran Nmap on (or a similarly configured one) due to the hard-coded `nmap.xsl` filesystem path. Use the `--webxml` or `--stylesheet` options to create portable XML files that render as HTML on any web-connected machine.]()

[`-oS *`<filespec>`*` (ScRipT KIdd|3 oUTpuT) ]()[ ]()[]()

[Script kiddie output is like interactive output, except that it is post-processed to better suit the l33t HaXXorZ who previously looked down on Nmap due to its consistent capitalization and spelling. Humor impaired people should note that this option is making fun of the script kiddies before flaming me for supposedly “helping them”.]()

[`-oG *`<filespec>`*` (grepable output) ]()[ ]()[]()

[This output format is covered last because it is deprecated.
The XML output format is far more powerful, and is nearly as
convenient for experienced users. XML is a standard for which dozens
of excellent parsers are available, while grepable output is my own
simple hack. XML is extensible to support new Nmap features as they
are released, while I often must omit those features from grepable
output for lack of a place to put them.]()

[Nevertheless, grepable output is still quite popular. It is a
simple format that lists each host on one line and can be trivially
searched and parsed with standard Unix tools such as grep, awk, cut,
sed, diff, and Perl. Even I usually use it for one-off tests done at the
command line. Finding all the hosts with the SSH port open or that
are running Solaris takes only a simple grep to identify the hosts,
piped to an awk or cut command to print the desired fields.]()

[Grepable output consists of comments (lines starting with a
pound (#))]()[and target lines. A target line includes a combination
of six labeled fields, separated by tabs and followed with a colon.
The fields are `Host`, `Ports`,`Protocols`, `Ignored State`,`OS`, `Seq Index`,`IP ID`, and `Status`.]()

[The most important of these fields is generally`Ports`, which gives details on each interesting
port. It is a comma separated list of port entries. Each port entry
represents one interesting port, and takes the form of seven slash
(/) separated subfields. Those subfields are: `Port
number`, `State`, `Protocol`,`Owner`, `Service`, `SunRPC
info`, and `Version info`.]()

[As with XML output, this man page does not allow for documenting
the entire format. A more detailed look at the Nmap grepable output
format is available in ]()[the section called “Grepable Output (`-oG`)”](https://nmap.org/book/output-formats-grepable-output.html).

`-oA *`<basename>`*` (Output to all formats) []()

[ As a convenience, you may specify `-oA *`<basename>`*` to store scan results in normal, XML, and grepable formats at once. They are stored in `*`<basename>`*.nmap`, `*`<basename>`*.xml`, and `*`<basename>`*.gnmap`, respectively. As with most programs, you can prefix the filenames with a directory path, such as `~/nmaplogs/foocorp/` on Unix or `c:\hacking\sco` on Windows.]()

[Verbosity and debugging options]()

[`-v` (Increase verbosity level) ]()[ ]()[ , `-v*`<level>`*` (Set verbosity level) ]()

[Increases the verbosity level, causing Nmap to print more information about the scan in progress. Open ports are shown as they are found and completion time estimates are provided when Nmap thinks a scan will take more than a few minutes. Use it twice or more for even greater verbosity: `-vv`, or give a verbosity level directly, for example `-v3`.]()[ ]()

[Most changes only affect interactive output, and some also affect normal and script kiddie output. The other output types are meant to be processed by machines, so Nmap can give substantial detail by default in those formats without fatiguing a human user. However, there are a few changes in other modes where output size can be reduced substantially by omitting some detail. For example, a comment line in the grepable output that provides a list of all ports scanned is only printed in verbose mode because it can be quite long.]()

[`-d` (Increase debugging level) ]()[ ]()[ , `-d*`<level>`*` (Set debugging level) ]()

[When even verbose mode doesn't provide sufficient data for you,
debugging is available to flood you with much more! As with the
verbosity option (`-v`), debugging is enabled with a
command-line flag (`-d`) and the debug level can be
increased by specifying it
multiple times,]()[as in `-dd`, or by setting a level directly. For
example, `-d9` sets level nine. That is the highest
effective level and will produce thousands of lines unless you run a
very simple scan with very few ports and targets.]()

[Debugging output is useful when a bug is suspected in Nmap,
or if you are simply confused as to what Nmap is doing and why. As this
feature is mostly intended for developers, debug lines aren't always
self-explanatory. You may get something like: `Timeout
vals: srtt: -1 rttvar: -1 to: 1000000 delta 14987 ==> srtt: 14987
rttvar: 14987 to: 100000`. If you don't understand a line, your only recourses
are to ignore it, look it up in the source code, or request help from
the development list
(*nmap-dev*).]()[Some lines are self explanatory, but
the messages become more obscure as the debug level is
increased.]()

[`--reason` (Host and port state reasons) ]()[ ]()[ ]()

[Shows the reason each port is set to a specific state and the reason
each host is up or down. This option displays the type of the packet
that determined a port or hosts state. For example, A `RST` packet from
a closed port or an echo reply from an alive host. The information
Nmap can provide is determined by the type of scan or ping. The SYN
scan and SYN ping (`-sS` and `-PS`) are very detailed, but the
TCP connect scan (`-sT`) is limited by the
implementation of the `connect` system call. This feature is automatically enabled by
the debug option
(`-d`)]()[and the results are stored in XML log files
even if this option is not specified. ]()

[`--stats-every *`<time>`*` (Print periodic timing stats) ]()[ ]()

[ Periodically prints a timing status message after each interval of *`<time>`*. The time is a specification of the kind described in ]()[the section called “Timing and Performance”](https://nmap.org/book/man-performance.html); so for example, use `--stats-every 10s` to get a status update every 10 seconds. Updates are printed to interactive output (the screen) and XML output.

`--packet-trace` (Trace packets and data sent and received) []()

[Causes Nmap to print a summary of every packet sent or received. This is often used for debugging, but is also a valuable way for new users to understand exactly what Nmap is doing under the covers. To avoid printing thousands of lines, you may want to specify a limited number of ports to scan, such as `-p20-30`. If you only care about the goings on of the version detection subsystem, use `--version-trace` instead. If you only care about script tracing, specify `--script-trace`. With `--packet-trace`, you get all of the above.]()

[`--open` (Show only open (or possibly open) ports) ]()[ ]()

[Sometimes you only care about ports you can actually connect to
(`open` ones), and don't want results cluttered with`closed`, `filtered`, and`closed|filtered` ports. Output customization is
normally done after the scan using tools such asgrep, awk, andPerl, but this feature was added due to
overwhelming requests. Specify `--open` to only see
hosts with at least one`open`, `open|filtered`, or`unfiltered` port, and only see ports in those states. These three states are treated just as they normally are, which means that `open|filtered` and `unfiltered` may be condensed into counts if there are an overwhelming number of them.]()

[Beginning with Nmap 7.40, the `--open` option implies ]()[ `--defeat-rst-ratelimit`, because that option only affects `closed` and `filtered` ports, which are hidden by `--open`.]()

[`--iflist` (List interfaces and routes) ]()[ ]()

[Prints the interface list and system routes as detected by Nmap and quits. This is useful for debugging routing problems or device mischaracterization (such as Nmap treating a PPP connection as ethernet).]()

[Miscellaneous output options]()

[`--append-output` (Append to rather than clobber output files) ]()[ ]()

[When you specify a filename to an output format flag such as `-oX` or `-oN`, that file is overwritten by default. If you prefer to keep the existing content of the file and append the new results, specify the `--append-output` option. All output filenames specified in that Nmap execution will then be appended to rather than clobbered. This doesn't work well for XML (`-oX`) scan data as the resultant file generally won't parse properly until you fix it up by hand.]()

[`--resume *`<filename>`*` (Resume aborted scan) ]()[ ]()[ ]()

[Some extensive Nmap runs take a very long time—on the order of days. Such scans don't always run to completion. Restrictions may prevent Nmap from being run during working hours, the network could go down, the machine Nmap is running on might suffer a planned or unplanned reboot, or Nmap itself could crash. The administrator running Nmap could cancel it for any other reason as well, by pressing **ctrl-C**. Restarting the whole scan from the beginning may be undesirable. Fortunately, if scan output files were kept, the user can ask Nmap to resume scanning with the target it was working on when execution ceased. Simply specify the `--resume` option and pass the output file as its argument. No other arguments are permitted, as Nmap parses the output file to use the same ones specified previously. Simply call Nmap as **nmap --resume *`<logfilename>`***. Nmap will append new results to the data files specified in the previous execution. Scans can be resumed from any of the 3 major output formats: Normal, Grepable, or XML]()

[`--noninteractive` (Disable runtime interactions) ]()[ ]()[ ]()

[At times, such as when running Nmap in a shell background, it might be undesirable for Nmap to monitor and respond to user keyboard input when running. (See ]()[the section called “Runtime Interaction”](https://nmap.org/book/man-runtime-interaction.html) about how to control Nmap during a scan.) Use option `--noninteractive` to prevent Nmap taking control of the terminal.

`--stylesheet *`<path or URL>`*` (Set XSL stylesheet to transform XML output) []()

[Nmap ships with an XSL]()[ stylesheet]()[ named `nmap.xsl`]()[ for viewing or translating XML output to HTML.]()[ The XML output includes an `xml-stylesheet` directive which points to `nmap.xml` where it was initially installed by Nmap. Run the XML file through an XSLT processor such as ]()[xsltproc](http://xmlsoft.org/XSLT/)[ to produce an HTML file. Directly opening the XML file in a browser no longer works well because modern browsers limit the locations a stylesheet may be loaded from. If you wish to use a different stylesheet, specify it as the argument to `--stylesheet`. You must pass the full pathname or URL. One common invocation is `--stylesheet https://nmap.org/svn/docs/nmap.xsl`. This tells an XSLT processor to load the latest version of the stylesheet from Nmap.Org. The `--webxml` option does the same thing with less typing and memorization. Loading the XSL from Nmap.Org makes it easier to view results on a machine that doesn't have Nmap (and thus `nmap.xsl`) installed. So the URL is often more useful, but the local filesystem location of `nmap.xsl` is used by default for privacy reasons.]()

[`--webxml` (Load stylesheet from Nmap.Org) ]()[ ]()

[This is a convenience option, nothing more than an alias for `--stylesheet https://nmap.org/svn/docs/nmap.xsl`.]()

[`--no-stylesheet` (Omit XSL stylesheet declaration from XML) ]()[ ]()

[Specify this option to prevent Nmap from associating any XSL stylesheet with its XML output. The `xml-stylesheet` directive is omitted.]()

[]()

---

[Prev](https://nmap.org/book/man-bypass-firewalls-ids.html)Firewall/IDS Evasion and Spoofing

[Up](https://nmap.org/book/man.html)Chapter 15. Nmap Reference Guide

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/man-misc-options.html)Miscellaneous Options