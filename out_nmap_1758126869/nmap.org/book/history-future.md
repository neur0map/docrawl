---
title: "The History and Future of Nmap | Nmap Network Scanning"
source_url: https://nmap.org/book/history-future.html
fetched_at: 2025-09-17T16:38:44.804495+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 1. Getting Started with Nmap](https://nmap.org/book/intro.html)
* The History and Future of Nmap

[Prev](https://nmap.org/book/legal-issues.html)

[Next](https://nmap.org/book/install.html)

The History and Future of Nmap
----------

Many ancient and well loved security tools, such as
Netcat,[]()tcpdump,[]()and
John the Ripper,[]()haven't changed much over the years.
Others, including
Wireshark,[]()Metasploit,[]()Cain and Abel,[]()and Snort,[]()have been under constant development since the day they were released.
Nmap is in that second category. It was released as a simple
Linux-only port scanner in 1997. Over the next 16+ years it sprouted a
myriad of valuable features, including OS detection, version
detection, the Nmap Scripting Engine, a Windows port, a graphical user
interface, Ncat, Nping, Ndiff, and more. We plan to continue this rapid development pace in the future as well!

### The History of Nmap ###

[]()

This section provides a timeline of the most
important milestones over 16 years of Nmap history. For all significant Nmap changes
(thousands of them), read the[Nmap Changelog](https://nmap.org/changelog.html).[]()Old releases of Nmap can be found at [`https://nmap.org/dist/`](https://nmap.org/dist/), and ancient versions at[`https://nmap.org/dist-old/`](https://nmap.org/dist-old/).[]()

* **September 1, 1997** — Nmap is first released in [*Phrack* magazine Issue 51, Article 11](https://nmap.org/p51-11.html)[](). It doesn't have a version number because new releases aren't planned. Nmap is about 2,000 lines long, and compilation is as simple as **gcc -O6 -o nmap nmap.c -lm**.

* **September 5, 1997** — Due to popular demand, a slightly improved version of the *Phrack* code is released, calling itself version 1.25. The gzipped tarball is 28KB. Version 1.26 (48KB) is released 19 days later.

* **January 11, 1998** — Insecure.Org is registered and Nmap moves there from its previous home at the [DataHaven Project](http://www.dhp.com/) ISP.

* **March 14, 1998** — Renaud Deraison writes to inform me that he is writing a security scanner, and asks if he can use some Nmap source code. Of course I say yes. Nine days later he sends me a pre-release version of
  Nessus,[]()noting that it “is designed for sysadmins, not 3l33t H4ck3rZ”.

* **December 12, 1998** — Nmap version 2.00 is publicly released, introducing Nmap OS detection for the first time after several months of private development. An paper describing the techniques was released in[*Phrack* 54, Article 9](https://nmap.org/phrack54-09.txt).
  By this point Nmap is broken up into many files, consists of about 8,000 lines of code, is kept in a private CVS revision control system, and the tarball size is 275KB. The*nmap-hackers* mailing list[]()is started, and later grows to more than 80,000 members.

* **April 11, 1999** — Nmap 2.11BETA1 is released. This is the first version to contain a graphical user interface as an alternative to the traditional command-line usage. The bundled Unix-only NmapFE[]() GUI was originally written by
  Zach Smith.[]()Some people like it, but most prefer command-line execution.

* **April 28, 2000** — Nmap 2.50 is [released](https://seclists.org/nmap-hackers/2000/140). By this point the tarball has grown to 461KB. This release includes timing modes such as `-T aggressive`, direct SunRPC scanning, and the Window and ACK scan methods.

* **May 28, 2000** —
  Gerhard Rieger[]()sends a [message](https://seclists.org/nmap-hackers/2000/217) to the*nmap-dev* list[]()describing a new “protocol scan” he has developed for Nmap, and he even includes a patch. This is so cool that I [release](https://seclists.org/nmap-hackers/2000/219) Nmap 2.54BETA1 with his patch less than 12 hours later.

* **December 7, 2000** — Nmap 2.54BETA16 is [released](https://seclists.org/nmap-dev/2000/q4/13) as the first official version to compile and run on Microsoft Windows. The Windows porting work was done by
  Ryan Permeh[]()and Andy Lutomirski.[]()

* **July 9, 2001** — The Nmap IP ID idle scan is introduced with Nmap 2.54BETA26. A paper describing the technique is released concurrently. This extremely cool (though not always practical) scan technique is described in [the section called “TCP Idle Scan (`-sI`)”](https://nmap.org/book/idlescan.html).

* **July 25, 2002** — I quit my job at Netscape/AOL and start my dream job working on Nmap full time.

* **July 31, 2002** — Nmap 3.00 is [released](https://insecure.org/stf/Nmap-3.00-Release.html). The tarball is 922K. This release includes Mac OS X support, XML output, and uptime detection.

* **August 28, 2002** — Nmap is converted from C to C++ and IPv6 supported is added as part of the [Nmap 3.10ALPHA1 release](https://seclists.org/nmap-dev/2002/q3/41).

* **May 15, 2003** —
  Nmap is featured in the movie*The Matrix Reloaded*,[]()where Trinity uses it (followed by a real SSH
  exploit) to hack a power station and save the world. This leads to
  more publicity for Nmap than it had ever seen before. Hollywood takes
  notice, and Nmap becomes a standard prop for hacking scenes in movies.
  Nmap is later seen in *The Bourne Ultimatum*,*Die Hard 4*, *The Girl with the Dragon
  Tattoo*, *Dredd*, and many others. Details and screen shots from all these movies are available at[`https://nmap.org/movies/`](https://nmap.org/movies/).

* **July 21, 2003** —
  I finish a first implementation of Nmap service/version detection
  ([Chapter 7, *Service and Application Version Detection*](https://nmap.org/book/vscan.html)) and release it to a couple dozen top Nmap
  developers and users as Nmap 3.40PVT1. That is followed up by 16 more
  private releases over the next couple months as we improve the system
  and add signatures.

* **September 16, 2003**— Nmap service detection is publicly[released](https://seclists.org/nmap-hackers/2003/30)as part of Nmap 3.45.

* **February 20, 2004** —
  Nmap 3.50 is [released](https://insecure.org/stf/Nmap-3.50-Release.html).
  The tarball is now 1,571KB.
  SCO Corporation[]()is banned from redistributing Nmap because they refuse to comply with the
  GPL.[]()They
  have to rebuild their Caldera release ISOs to remove Nmap. This
  release includes the packet tracing and UDP ping options, as well as an OS classification system.

* **August 31, 2004** —
  The core Nmap port scanning engine is rewritten for [Nmap 3.70](https://seclists.org/nmap-hackers/2004/10). The
  new engine, named`ultra_scan`[]()features
  dramatically improved algorithms and parallelization support to
  improve both accuracy and speed. The differences are particularly
  dramatic for hosts behind strict firewalls.

* **June 25, 2005** —
  Google sponsors 10 college and graduate students to work on Nmap full
  time for the summer as part of
  Google's [Summer of
  Code](http://code.google.com/soc)[]()[]()initiative. Projects include a modern reimplementation of Netcat named Ncat (Chris Gibson),[]() a second generation OS
  detection system
  (Zhao Lei),[]()a new cross-platform GUI which later becomes Zenmap[]()(Adriano Monteiro Marques),[]()and many other cool projects described at[`https://seclists.org/nmap-hackers/2005/8`](https://seclists.org/nmap-hackers/2005/8).

* **September 8, 2005** —
  Nmap gains raw ethernet frame sending support with the release of version [3.90](https://seclists.org/nmap-hackers/2005/12). This allows for ARP
  scanning (see [the section called “ARP Scan (`-PR`)”](https://nmap.org/book/host-discovery-techniques.html#arp-scan)) and MAC address spoofing as
  well as evading the raw IP packet ban introduced by Microsoft in
  Windows XP SP2.

* **January 31, 2006** —
  Nmap 4.00 is [released](https://insecure.org/stf/Nmap-4.00-Release.html).
  The tarball is now 2,388KB. This release includes runtime interaction
  to provide on-demand completion estimates, a Windows executable
  installer, NmapFE updates to support GTK2, and much
  more.

* **May 24, 2006** —
  Google sponsors 10 more Nmap summer developers as part of their SoC
  program. Zhao and Adriano return as part of 2006 SoC to further
  develop their respective projects. Diman Todorov is sponsored to help
  develop the Nmap Scripting Engine ([Chapter 9, *Nmap Scripting Engine*](https://nmap.org/book/nse.html)).
  These and seven other talented students and their projects are
  described at[`https://seclists.org/nmap-hackers/2006/9`](https://seclists.org/nmap-hackers/2006/9).

* **June 24, 2006** —
  After two years of development and testing, the 2nd generation OS
  detection system is integrated
  into [Nmap
  4.20ALPHA1](https://seclists.org/nmap-dev/2006/q2/444). This new system is based on everything we've
  learned and the new ideas we've conceived since the first generation
  system debuted eight years earlier. The new system, described in [Chapter 8, *Remote OS Detection*](https://nmap.org/book/osdetect.html),
  proves much more accurate and granular than the first generation, although it takes us 2.5 years to reach the size of the old DB.

* **December 10, 2006**— The Nmap Scripting Engine
  is [released](https://seclists.org/nmap-dev/2006/q4/184)as part of Nmap 4.21ALPHA1. NSE allows users to write (and share)
  simple scripts to automate a wide variety of networking tasks. This
  initial version includes 23 scripts for simple tasks such as detecting
  the insecure SSHv1 and SSLv2 protocols, obtaining service owner
  information from identd, and testing whether DNS servers support
  recursive queries.

* **December 20, 2006** —
  Nmap's Subversion source code repository [opens to the public](https://seclists.org/nmap-dev/2006/q4/253).
  Until this time, only a handful of developers had access to the
  private source repository. Everyone else had to wait for releases.
  Now everyone can follow Nmap development day to day. A new`nmap-svn` mailing list provides real-time change
  notification by email. For details, see [the section called “Obtaining Nmap from the Subversion (SVN) Repository”](https://nmap.org/book/install.html#inst-svn).

* **May 28, 2007** —
  Google sponsors six summer Nmap developers as part of their SoC
  program. Among the sponsored students was David Fifield, who
  continued long after the summer ended and became Nmap's co-maintainer.
  He even helped greatly in producing this book! The Nmap students and their projects are listed at[`https://seclists.org/nmap-hackers/2007/3`](https://seclists.org/nmap-hackers/2007/3).

* **July 8, 2007**  —
  The Zenmap graphical front end (back then it was called Umit) is
  improved and integrated into
  the [Nmap 4.22SOC1
  release](https://seclists.org/nmap-dev/2007/q3/30) for testing. The venerable NmapFE GUI is removed.
  Zenmap is covered in [Chapter 12, *Zenmap GUI Users' Guide*](https://nmap.org/book/zenmap.html).

* **December 13, 2007**— Nmap 4.50 is [released](https://insecure.org/stf/Nmap-4.50-Release.html) to celebrate Nmap's 10th anniversary!

* **June 1, 2008** —
  Nmap 4.65
  is [released](https://seclists.org/nmap-dev/2008/q2/558)and includes, for the first time, an executable Mac OS X
  installer. The Nmap source tarball is now four megabytes. This release
  includes 41 NSE scripts, 1,307 OS fingerprints, and 4,706 version
  detection signatures.

* **August 18, 2008** —
  The Nmap project completes its fourth Summer of Code, with our highest
  success percentage ever (six out of seven sponsored students succeeded). They
  greatly improved Zenmap, the Nmap Scripting Engine, OS detection, and
  Ncat, as described at[`https://seclists.org/nmap-dev/2008/q4/193`](https://seclists.org/nmap-dev/2008/q4/193).

* **September 8, 2008** —
  Nmap 4.75 is[released](https://seclists.org/nmap-hackers/2008/4)with almost 100 significant improvements over 4.68. These include the
  Zenmap network topology viewer, which draws an interactive diagram of discovered network devices. After 11 years of distributing Nmap (“the Network Mapper”), it can now actually draw network maps. This version also includes scan aggregation, allowing several scans to be combined into one unified view. Both of these features are described in [Chapter 12, *Zenmap GUI Users' Guide*](https://nmap.org/book/zenmap.html)). This release also includes port-frequency data from
  my Worldscan project, which
  I [presented](https://nmap.org/presentations/)at Black Hat and Defcon the previous month. This allows Nmap to focus on the ports which have been empirically shown to be the most popular.

* **October 4, 2008** —
  The Nmap Documentation Portal is launched at [`https://nmap.org/nsedoc/`](https://nmap.org/nsedoc/). This allows users to brows all of Nmap's NSE script and library documentation in one place. The documentation is generated from specially-formed comments in the source code (see [the section called “Writing Script Documentation (NSEDoc)”](https://nmap.org/book/nsedoc.html)).

* **January 1, 2009** —
  The first version of this book, [*Nmap Network Scanning*](https://nmap.org/book/), is released. The answer to whether anyone besides me would be interested in a 450+ page book on port scanning turns out to be a resounding yes! More than 10,000 copies are sold in the first 18 months. It is also released by other publishers in the German, Korean, and Brazilian Portuguese languages.

* **January 23, 2009** —
  Added Ncat, a feature-packed networking utility which reads and writes data across a network from the command line. It is a reimplementation of the classic Netcat tool which adds support for modern networking features such as SSL, connection redirection, and proxying. See [Chapter 17, *Ncat Reference Guide*](https://nmap.org/book/ncat-man.html).

* **January 23, 2009** —
  Added the Ndiff utility which compares the results of two Nmap
  scans, making it easy to scan networks on a regular basis and create a report (XML or text) showing any changes. See [Chapter 16, *Ndiff Reference Guide*](https://nmap.org/book/ndiff-man.html).

* **March 30, 2009** —
  A special Nmap release (4.85BETA5) is produced to remotely detect the Conficker worm which has infected millions of machines on the Internet. Demand is so high that Nmap is booted from Dreamhost's “unlimited bandwidth” web hosting plan for using too much bandwidth. The University of California, San Diego steps up and provides a fast mirror server. The next version of the Conficker worm blocks infected users from reaching Insecure.Org and Nmap.Org. We take that as a compliment to Nmap's effectiveness.

* **June 12, 2009** —
  Added SCTP port scanning and host discovery support to Nmap. SCTP is a
  layer 4 protocol used mostly for telephony related
  applications.

* **July 16, 2009** —[Released Nmap 5.00](https://nmap.org/5/). The tarball is more than 27MB and contains 2,003 OS fingerprints, 5,512 version detection signatures, and 59 NSE scripts.

* **August 24, 2009** —
  The Nmap project [completes](https://seclists.org/nmap-dev/2009/q4/148) its fifth Summer of Code. For the first time, every student passed! They created Nping as well as the network authentication cracker [Ncrack](https://nmap.org/ncrack/) and made dramatic improvements to the Nmap Scripting Engine, Ncat, and Zenmap.

* **March 29, 2010** —
  Added Nping, a handy tool for network packet generation, response analysis and response time measurement (see [Chapter 18, *Nping Reference Guide*](https://nmap.org/book/nping-man.html)).

* **July 28, 2010** —
  David Fifield and I present on Mastering the Nmap Scripting Engine at
  the Black Hat and Defcon conferences, including live demos (video: [`https://nmap.org/presentations/`](https://nmap.org/presentations/)).

* **August 17, 2010** —
  Scanning the top million web sites for fun leads to a spinoff art
  project as we release [Icons of
  the Web](https://nmap.org/favicon/), a mosaic of web site icons scaled by their popularity.
  It is written up in the [New
  York Times](http://bits.blogs.nytimes.com/2010/08/26/bits-pics-visualizing-the-webs-icons/),[The
  Telegraph](http://www.telegraph.co.uk/technology/internet/7963908/The-worlds-top-300000-websites-visualised-by-favicon.html),[Slashdot](http://news.slashdot.org/story/10/08/23/1911247/Nmap-Developers-Release-a-Picture-of-the-Web),[Gizmodo](http://gizmodo.com/5620681/all-300000-biggest-websites-visualized-with-their-icons), [Engadget](http://www.engadget.com/2010/08/25/visualized-worlds-most-trafficked-websites-and-their-favicons/),
  and many others. The [Newseum](http://www.newseum.org/) in Washington
  D.C. exhibits a giant version and it was even featured in the Guinness World Records book.

* **January 28, 2011** —[Released Nmap
  5.50](https://seclists.org/nmap-announce/2011/0). We added support for the ancient Gopher
  protocol for just fun, but serious improvements included tripling the NSE
  scripts to 177 and adding a new Zenmap script selection interface. We
  also added a novel echo mode feature to Nping which can ease network
  analysis and debugging (see [the section called “Echo Mode”](https://nmap.org/book/nping-man.html#nping-man-echo-mode)).
  Performance was a high priority too, with certain scan times reduced from
  hours to minutes and memory consumption reduced 90% in our benchmark scan.

* **June 8, 2011** —
  We made a commermorative World IPv6 day release with numerous new IPv6
  features, plus we ensured that all of our websites were IPv6-enabled.
  While Nmap has supported basic IPv6 scanning since 2002, we added
  advanced host discovery and raw port scanning. We also soon added a
  new machine language based IPv6 OS detection system (see [the section called “IPv6 fingerprinting”](https://nmap.org/book/osdetect-ipv6-methods.html)).

* **November 4, 2011** —
  After surveying more than 3,000 Nmap users for their favorite security
  tools, we releaunched our [SecTools.org](https://sectools.org/) web
  site with the new data. Our previous sites were just static HTML files, but this new
  release includes interactive rating and review features to better
  share information. 49 of the the top 125 tools profiled on the site
  are [new to the list](https://sectools.org/tag/new/). Of course the Nmap family of tools are excluded
  from the site due to potential conflicts of interest.

* **May 21, 2012** —[Nmap 6 was released](https://nmap.org/6/)! The tarball
  is more than 54MB and contains 3,572 OS fingerprints, 8,165 version
  detection signatures, and 348 NSE scripts.

[]()

### The Future of Nmap ###

[]()

While it is easy to catalogue the history of Nmap, the future is
uncertain. Nmap didn't start off with any grand development plan, and
most of the milestones in the preceding timeline were not planned more
than a year or two in advance. Instead of trying to predict the shape of the
Internet and networking way out in the future, I closely study where
it is now and decide what will be most useful for Nmap in the near
term. So I have no idea where Nmap will be 10 years from now, though
I expect it to be as popular and vibrant as ever. The Nmap community
is large enough to guide Nmap wherever it needs to go. Nmap has faced
curve balls before, such as the sudden removal of raw packet support
in Windows XP SP2, dramatic changes in network filtering practices and
technology, and the slow emergence of IPv6. Each of those required
significant changes to Nmap, and we will have to do the same to embrace
or at least cope with future networking changes.

While the 10-year plan is up in the air, we do have plans and
guiding priorities for the next several years:

* **Nmap Scripting Engine**— NSE has exploded in capabilities and popularity. As of
  mid-2010, it contains 131 scripts—68% higher than the year
  before. NSE scripts are also growing more powerful thanks to the
  ever-increasing library and infrastructure feature support. I would
  like to see NSE grow to 500+ scripts in the next few years. Nmap will
  soon support `prescan`[]()and `postscan`[]() scripts which are not
  tied to an individual host or open port like the existing `hostrule` and `portrule` scripts are.

* **Scanning web sites** —[]()Nmap will also grow in its ability to handle web scanning. When Nmap
  was first developed, different services were often provided as
  separate daemons identified by the port number they listen on. Now,
  many new services simply run over HTTP and are identified by a URL
  path name rather than port number. Scanning for known URL paths is
  similar in many ways to port scanning (and to the SunRPC scanning
  which Nmap has also done for many years). The Nmap Scripting Engine
  already supports many web scanning features, but I'd like to see http
  brute force password cracking, better spidering support, an HTML/XML parser, better
  web application fingerprinting, and the ability to proxy certain requests through
  HTTP and SOCKS proxies. Nmap should be able to port scan
  through those proxies too.

* **Web infrastructure improvements** —
  Nmap is a cooperative project developed by many individuals. As the number of people involved increases, we also need to scale the tools and infrastructure resources to facilitate that cooperation. We're planning an Nmap wiki, an improved Subversion source control server, and possibly a web discussion forum. We are also moving the infrastructure to virtual machines for security isolation and to better share administration tasks. The web sites desperately need a redesign as well, unless 1990's table-based layouts come back in favor.

* **Online scanning web service** —
  We're working on a service which allows users to scan their own machines and networks “from the cloud” to see what is exposed from an outside perspective. Repeat scans can be scheduled with changes sent in email alerts or browsed online.

 In addition to those high-level priorities, we have some concrete tasks in mind:

* Internationalize Nmap to support many different languages rather than just English. The Zenmap front-end already does this, supporting five languages, as described in [the section called “Zenmap in Your Language”](https://nmap.org/book/zenmap-lang.html).

* Add a testing harness system to more easily catch regressions (bugs). Ncat has already implemented such a system and it has proven extremely useful.

* The OS and Version detection systems may be augmented to support the [NIST Common Platform Enumeration (CPE) standard](http://cpe.mitre.org/) for better compatibility with other tools.

* Zenmap will soon offer an advanced script selection interface, making it easy to browse and choose scripts and script-arguments supported by the version of Nmap installed on a machine.

* We plan to produce, encourage the production of, and distribute more Nmap-related videos to introduce people to Nmap and teach them tricks and techniques for using it effectively.

For a rougher and lower-level list of what we're working on *right now* and planned for the near future, you can always read the Nmap todo list at [`https://nmap.org/svn/todo/nmap.txt`](https://nmap.org/svn/todo/nmap.txt).

Some of the coolest Nmap features in the past, such as OS
and version detection, were developed in secret and given a
surprise release. You can expect more of these in coming years
because they are so much fun!

[]()

---

[Prev](https://nmap.org/book/legal-issues.html)Legal Issues

[Up](https://nmap.org/book/intro.html)Chapter 1. Getting Started with Nmap

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/install.html)Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap