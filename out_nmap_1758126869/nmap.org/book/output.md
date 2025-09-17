---
title: "Chapter 13. Nmap Output Formats | Nmap Network Scanning"
source_url: https://nmap.org/book/output.html
fetched_at: 2025-09-17T16:47:10.311467+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* Chapter 13. Nmap Output Formats

[Prev](https://nmap.org/book/zenmap-history.html)

[Next](https://nmap.org/book/output-formats-commandline-flags.html)

Chapter 13. Nmap Output Formats
==========

Table of Contents

* [Introduction](https://nmap.org/book/output.html#output-formats-intro)
* [Command-line Flags](https://nmap.org/book/output-formats-commandline-flags.html)
  * [Controlling Output Type](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-flags-type)
  * [Controlling Verbosity of Output](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-flags-verbosity)
  * [Enabling Debugging Output](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-flags-debugging)
  * [Enabling Packet Tracing](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-flags-packet-trace)
  * [Resuming Aborted Scans](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-flags-resume)

* [Interactive Output](https://nmap.org/book/output-formats-interactive.html)
* [Normal Output (`-oN`)](https://nmap.org/book/output-formats-normal-output.html)
* [$crIpT kIddI3 0uTPut (`-oS`)](https://nmap.org/book/output-formats-script-kiddie.html)
* [XML Output (`-oX`)](https://nmap.org/book/output-formats-xml-output.html)
  * [Using XML Output](https://nmap.org/book/output-formats-xml-output.html#output-formats-xml-usage)

* [Manipulating XML Output with Perl](https://nmap.org/book/output-formats-xml-with-perl.html)
* [Common Platform Enumeration (CPE)](https://nmap.org/book/output-formats-cpe.html)
  * [Structure of a CPE Name](https://nmap.org/book/output-formats-cpe.html#output-formats-cpe-structure)

* [Output to a Database](https://nmap.org/book/output-formats-output-to-database.html)
* [Creating HTML Reports](https://nmap.org/book/output-formats-output-to-html.html)
  * [Saving a Permanent HTML Report](https://nmap.org/book/output-formats-output-to-html.html#output-formats-html-permanent)

* [Grepable Output (`-oG`)](https://nmap.org/book/output-formats-grepable-output.html)
  * [Grepable Output Fields](https://nmap.org/book/output-formats-grepable-output.html#output-formats-grepable-fields)
    * [`Host` field](https://nmap.org/book/output-formats-grepable-output.html#output-formats-grepable-fields-host)
    * [`Status` field](https://nmap.org/book/output-formats-grepable-output.html#output-formats-grepable-fields-status)
    * [`Ports` field](https://nmap.org/book/output-formats-grepable-output.html#output-formats-grepable-fields-ports)
    * [`Protocols` field](https://nmap.org/book/output-formats-grepable-output.html#output-formats-grepable-fields-protocol)
    * [`Ignored State` field](https://nmap.org/book/output-formats-grepable-output.html#output-formats-grepable-fields-ignored)
    * [`OS` field](https://nmap.org/book/output-formats-grepable-output.html#output-formats-grepable-fields-os)
    * [`Seq Index` field](https://nmap.org/book/output-formats-grepable-output.html#output-formats-grepable-fields-seqindex)
    * [`IP ID Seq` field](https://nmap.org/book/output-formats-grepable-output.html#output-formats-grepable-fields-ipid)

  * [Parsing Grepable Output on the Command Line](https://nmap.org/book/output-formats-grepable-output.html#output-formats-grepable-commandline-parsing)

[]()[]()[]()[]()[]()[]()

Introduction
----------

A common problem with open-source security tools is confusing
and disorganized output. They often spew out many lines of irrelevant
debugging information, forcing users to dig through pages of output
trying to discern important results from the
noise.[]()Program authors
often devote little effort to organizing and presenting results
effectively. The output messages can be difficult to understand and
poorly documented. This shouldn't be too surprising—writing clever
code to exploit some TCP/IP weakness is usually more gratifying than
documentation or UI work. Since open source authors are rarely paid,
they do what they enjoy.

At the risk of offending my friend
Dan Kaminsky,[]()I'll name his[Scanrand](https://sectools.org/tools2006.html#scanrand)[]()port
scanner as an example of a program that was clearly developed with
far more emphasis on neat technical tricks than a user friendly UI. The
sample output in [Example 13.1](https://nmap.org/book/output.html#output-formats-ex-scanrand) is from the Scanrand
documentation page.

Example 13.1. Scanrand output against a local network

```
bash-2.05a# scanrand 10.0.1.1-254:quick
  UP:        10.0.1.38:80    [01]   0.003s
  UP:       10.0.1.110:443   [01]   0.017s
  UP:       10.0.1.254:443   [01]   0.021s
  UP:        10.0.1.57:445   [01]   0.024s
  UP:        10.0.1.59:445   [01]   0.024s
  UP:        10.0.1.38:22    [01]   0.047s
  UP:       10.0.1.110:22    [01]   0.058s
  UP:       10.0.1.110:23    [01]   0.058s
  UP:       10.0.1.254:22    [01]   0.077s
  UP:       10.0.1.254:23    [01]   0.077s
  UP:        10.0.1.25:135   [01]   0.088s
  UP:        10.0.1.57:135   [01]   0.089s
  UP:        10.0.1.59:135   [01]   0.090s
  UP:        10.0.1.25:139   [01]   0.097s
  UP:        10.0.1.27:139   [01]   0.098s
  UP:        10.0.1.57:139   [01]   0.099s
  UP:        10.0.1.59:139   [01]   0.099s
  UP:        10.0.1.38:111   [01]   0.127s
  UP:        10.0.1.57:1025  [01]   0.147s
  UP:        10.0.1.59:1025  [01]   0.147s
  UP:        10.0.1.57:5000  [01]   0.156s
  UP:        10.0.1.59:5000  [01]   0.157s
  UP:        10.0.1.53:111   [01]   0.182s
bash-2.05a#

```

While this does get the job done, it is difficult to interpret.
Output is printed based on
when the response was received, without any option for sorting the port
numbers or even grouping all open ports on a target host together. A
bunch of space is wasted near the beginning of each line
and no summary of results is provided.

Nmap's output is also far from perfect, though I do try pretty
hard to make it readable, well-organized, and flexible. Given the
number of ways Nmap is used by people and other software, no single
format can please everyone. So Nmap offers several formats,
including the interactive mode for humans to read directly and
XML for easy parsing by software.

In addition to offering different output formats, Nmap provides
options for controlling the verbosity of output as well as debugging
messages. Output types may be sent to standard output or to named
files, which Nmap can append to or clobber. Output files may also be
used to resume aborted scans. This chapter includes full details on
these options and every output format.

[]()

---

[Prev](https://nmap.org/book/zenmap-history.html)History

[Up](https://nmap.org/book/toc.html)Nmap Network Scanning

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/output-formats-commandline-flags.html)Command-line Flags