---
title: "Manipulating XML Output with Perl | Nmap Network Scanning"
source_url: https://nmap.org/book/output-formats-xml-with-perl.html
fetched_at: 2025-09-17T16:47:34.301593+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 13. Nmap Output Formats](https://nmap.org/book/output.html)
* Manipulating XML Output with Perl

[Prev](https://nmap.org/book/output-formats-xml-output.html)

[Next](https://nmap.org/book/output-formats-cpe.html)

Manipulating XML Output with Perl
----------

[]()

Generic XML parsers are available for all popular programming
languages, often for free. Examples are the libxml C library and the
Apache Xerces parser for Java and C++ (with Perl and COM bindings).
While these parsers are sufficient for handling Nmap XML output,
developers have created custom modules for several languages which can
make the task of interoperating with Nmap XML even easier.

The language with the best custom Nmap XML support is Perl.
Max Schubert[]()(affectionately known as Perldork) has created a module named[Nmap::Scanner](http://sourceforge.net/projects/nmap-scanner/)while Anthony Persaud[]()created [Nmap::Parser](http://anthonypersaud.com/category/nmap-parser/). These two
modules have many similarities: they can execute Nmap themselves or
read from an output file, are well documented, come with
numerous example scripts, are part of the Comprehensive Perl Archive
Network (CPAN), and are popular with users. They each offer both a
callback based parser for interpreting data as Nmap runs as well as an
all-at-once parser for obtaining a fully parsed document once Nmap
finishes executing. Their APIs are a bit different—Nmap::Scanner
relies on type-safe classes while Nmap::Parser relies on lighter-weight
native Perl arrays. I recommend looking at each to decide which best
meets your needs and preferences.

[]()

[Example 13.11](https://nmap.org/book/output-formats-xml-with-perl.html#output-formats-ex-nmap-parser) is a simple demonstration of Nmap::Parser. It comes from the module's documentation
(which contains many other examples as well). It performs a quick scan, then
prints overall scan statistics as well as information on each
available target host. Notice how readable it is compared to scripts
using other Nmap output formats that are dominated by parsing logic
and regular expressions. Even people with poor Perl skills could
use this as a starting point to create simple programs to automate
their Nmap scanning needs.

Example 13.11. Nmap::Parser sample code

[]()

```
use Nmap::Parser;

      #PARSING
my $np = new Nmap::Parser;

$nmap_exe = '/usr/bin/nmap';
$np->parsescan($nmap_exe,'-sT -p1-1023', @ips);

#or

$np->parsefile('nmap_output.xml'); #using filenames

      #GETTING SCAN INFORMATION

print "Scan Information:\n";
$si = $np->get_scaninfo();
#get scan information by calling methods
print
'Number of services scanned: '.$si->num_of_services()."\n",
'Start Time: '.$si->start_time()."\n",
'Scan Types: ',(join ' ',$si->scan_types())."\n";

      #GETTING HOST INFORMATION

print "Hosts scanned:\n";
for my $host_obj ($np->get_host_objects()){
  print
  'Hostname  : '.$host_obj->hostname()."\n",
  'Address   : '.$host_obj->ipv4_addr()."\n",
  'OS match  : '.$host_obj->os_match()."\n",
  'Open Ports: '.(join ',',$host_obj->tcp_ports('open'))."\n";
       #... you get the idea...
}

#frees memory--helpful when dealing with memory intensive scripts
$np->clean();

```

[]()[]()

For comparison, [Example 13.12](https://nmap.org/book/output-formats-xml-with-perl.html#output-formats-ex-nmap-scanner)is a sample Perl script using Nmap::Scanner, copied from its documentation.
This one uses an event-driven callback approach, registering the
functions `scan_started` and`port_found` to print real-time alerts
when a host is found up and when each open port is discovered on the
host.

Example 13.12. Nmap::Scanner sample code

[]()

```
my $scanner = new Nmap::Scanner;
$scanner->register_scan_started_event(\&scan_started);
$scanner->register_port_found_event(\&port_found);
$scanner->scan('-sS -p 1-1024 -O --max-rtt-timeout 200ms somehost.org.net.it');

sub scan_started {
    my $self     = shift;
    my $host     = shift;

    my $hostname = $host->name();
    my $addresses = join(', ', map {$_->address()} $host->addresses());
    my $status = $host->status();

    print "$hostname ($addresses) is $status\n";
}

sub port_found {
    my $self     = shift;
    my $host     = shift;
    my $port     = shift;

    my $name = $host->name();
    my $addresses = join(', ', map {$_->addr()} $host->addresses());

    print "On host $name ($addresses), found ",
          $port->state()," port ",
          join('/',$port->protocol(),$port->portid()),"\n";
}

```

[]()[]()

---

[Prev](https://nmap.org/book/output-formats-xml-output.html)XML Output (-oX)

[Up](https://nmap.org/book/output.html)Chapter 13. Nmap Output Formats

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/output-formats-cpe.html)Common Platform Enumeration (CPE)