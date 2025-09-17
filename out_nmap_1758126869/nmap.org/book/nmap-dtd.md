---
title: "The Full DTD | Nmap Network Scanning"
source_url: https://nmap.org/book/nmap-dtd.html
fetched_at: 2025-09-17T16:48:57.094151+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Appendix A. Nmap XML Output DTD](https://nmap.org/book/app-nmap-dtd.html)
* The Full DTD

[Prev](https://nmap.org/book/app-nmap-dtd.html)

[Next](https://nmap.org/book/idx.html)

The Full DTD
----------

\<!--  
 nmap.dtd  
 This is the DTD for Nmap's XML output (-oX) format.  
 $Id$  

 Originally written by:  
 William McVey \<wam@cisco.com\> \<wam+nmap@wamber.net\>  

 Now maintained by Fyodor \<fyodor@nmap.org\> as part of Nmap.   

 To validate using this file, simply add a DOCTYPE line similar to:  
 \<!DOCTYPE nmaprun SYSTEM "nmap.dtd"\>  
 to the nmap output immediately below the prologue (the first line). This  
 should allow you to run a validating parser against the output (so long  
 as the DTD is in your parser's DTD search path).  

 Bugs:  
 Most of the elements are "locked" into the specific order that nmap  
 generates, when there really is no need for a specific ordering.  
 This is primarily because I don't know the xml DTD construct to  
 specify "one each of this list of elements, in any order". If there  
 is a construct similar to SGML's '&' operator, please let me know.  

 Portions Copyright (c) 2001-2022 Nmap Software LLC  
 Portions Copyright (c) 2001 by Cisco systems, Inc.  

 Permission to use, copy, modify, and distribute modified and  
 unmodified copies of this software for any purpose and without fee is  
 hereby granted, provided that (a) this copyright and permission notice  
 appear on all copies of the software and supporting documentation, (b)  
 the name of Cisco Systems, Inc. not be used in advertising or  
 publicity pertaining to distribution of the program without specific  
 prior permission, and (c) notice be given in supporting documentation  
 that use, modification, copying and distribution is by permission of  
 Cisco Systems, Inc.  

 Cisco Systems, Inc. makes no representations about the suitability  
 of this software for any purpose. THIS SOFTWARE IS PROVIDED ``AS  
 IS'' AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING,  
 WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND  
 FITNESS FOR A PARTICULAR PURPOSE.  
\--\>  

\<!-- parameter entities to specify common "types" used elsewhere in the DTD --\>  
\<!ENTITY % attr\_alpha "CDATA" \>  
\<!ENTITY % attr\_numeric "CDATA" \>  
\<!ENTITY % attr\_ipaddr "CDATA" \>  
\<!ENTITY % attr\_percent "CDATA" \>  
\<!ENTITY % attr\_type "(ipv4 | ipv6 | mac)" \>  
\<!ENTITY % attr\_bool "(true | false)" \>  

\<!ENTITY % host\_states "(up|down|unknown|skipped)" \>  

\<!-- see: nmap.c:statenum2str for list of port states --\>  
\<!-- Maybe they should be enumerated as in scan\_types below , but I --\>  
\<!-- don't know how to escape states like open|filtered --\>  
\<!ENTITY % port\_states "CDATA" \>  

\<!ENTITY % hostname\_types "(user|PTR)" \>  

\<!-- see output.c:output\_xml\_scaninfo\_records for scan types --\>  
\<!ENTITY % scan\_types "(syn|ack|bounce|connect|null|xmas|window|maimon|fin|udp|sctpinit|sctpcookieecho|ipproto)" \>  

\<!-- \<!ENTITY % ip\_versions "(ipv4)" \> --\>  

\<!ENTITY % port\_protocols "(ip|tcp|udp|sctp)" \>  

\<!-- Service detection confidence level (portlist.h:struct serviceDeductions)  
\--\>   
\<!ENTITY % service\_confs "( 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10)" \>  

\<!-- This element was started in nmap.c:nmap\_main().  
 It represents to the topmost element of the output document.  
\--\>  
\<!ELEMENT nmaprun (scaninfo\*, verbose, debugging,  
 ( target | taskbegin | taskprogress | taskend | hosthint |  
 prescript | postscript | host | output)\*,  
 runstats) \>  
\<!ATTLIST nmaprun  
 scanner (nmap) #REQUIRED  
 args CDATA #IMPLIED  
 start %attr\_numeric;	#IMPLIED  
 startstr	CDATA #IMPLIED  
 version CDATA #REQUIRED  
 profile\_name	CDATA #IMPLIED  
 xmloutputversion CDATA #REQUIRED  
\>  

\<!-- this element is written in output.c:doscaninfo() --\>  
\<!ELEMENT scaninfo	EMPTY \>  
\<!ATTLIST scaninfo  
 type %scan\_types; #REQUIRED  
 scanflags	CDATA #IMPLIED  
 protocol	%port\_protocols; #REQUIRED  
 numservices	%attr\_numeric; #REQUIRED  
 services	CDATA #REQUIRED  
\>  

\<!-- these elements are written in nmap.c:nmap\_main() --\>  
\<!ELEMENT verbose	EMPTY \>  
\<!ATTLIST verbose	level %attr\_numeric;	#IMPLIED \>  

\<!ELEMENT debugging EMPTY \>  
\<!ATTLIST debugging	level %attr\_numeric;	#IMPLIED \>  

\<!ELEMENT target	EMPTY \>  
\<!ATTLIST target	specification	CDATA #REQUIRED  
 status (skipped)	#IMPLIED  
 reason (invalid)	#IMPLIED  
\>  

\<!-- this element is written in timing.c:beginOrEndTask() --\>  
\<!ELEMENT taskbegin	EMPTY \>  
\<!ATTLIST taskbegin  
 task CDATA #REQUIRED  
 time %attr\_numeric;	#REQUIRED  
 extrainfo	CDATA #IMPLIED  
\>  

\<!-- this element is written in timing.c:printStats() --\>  
\<!ELEMENT taskprogress	EMPTY \>  
\<!ATTLIST taskprogress  
 task CDATA #REQUIRED  
 time %attr\_numeric;	#REQUIRED  
 percent %attr\_percent;	#REQUIRED  
 remaining	%attr\_numeric;	#IMPLIED  
 etc %attr\_numeric;	#IMPLIED  
\>  

\<!-- this element is written in timing.c:beginOrEndTask() --\>  
\<!ELEMENT taskend	EMPTY \>  
\<!ATTLIST taskend  
 task CDATA #REQUIRED  
 time %attr\_numeric;	#REQUIRED  
 extrainfo	CDATA #IMPLIED  
\>  

\<!--   
 this element is started in nmap.c:nmap\_main() and filled by  
 output.c:write\_host\_status(), output.c:printportoutput(), and  
 output.c:printosscanoutput()  
\--\>  
\<!ELEMENT host ( status, address , (address | hostnames |  
 smurf | ports | os | distance | uptime |   
 tcpsequence | ipidsequence | tcptssequence |  
 hostscript | trace)\*, times? ) \>  
\<!ATTLIST host  
 starttime	%attr\_numeric;	#IMPLIED  
 endtime %attr\_numeric;	#IMPLIED  
 timedout	%attr\_bool;	#IMPLIED  
 comment CDATA #IMPLIED  
\>  

\<!-- these elements are written by scan\_engine.c:ultrascan\_host\_pspec\_update() --\>  
\<!ELEMENT hosthint (status,address+,hostnames?) \>  

\<!-- these elements are written by output.c:write\_xml\_initial\_hostinfo() --\>  
\<!ELEMENT status	EMPTY \>  
\<!ATTLIST status	state %host\_states;	#REQUIRED   
 reason CDATA	#REQUIRED   
 reason\_ttl	CDATA	#REQUIRED   
 \>  

\<!ELEMENT address	EMPTY \>  
\<!ATTLIST address	  
 addr %attr\_ipaddr;	#REQUIRED  
 addrtype	%attr\_type;	"ipv4"  
 vendor CDATA #IMPLIED  
\>  

\<!ELEMENT hostnames	(hostname)\* \>  
\<!ELEMENT hostname	EMPTY \>  
\<!ATTLIST hostname  
 name CDATA #IMPLIED  
 type %hostname\_types; #IMPLIED  
\>  

\<!-- this element is written by output.c:write\_host\_status() --\>  
\<!ELEMENT smurf EMPTY \>  
\<!ATTLIST smurf responses	%attr\_numeric;	#REQUIRED \>  

\<!-- these elements are written by output.c:printportoutput() --\>  

\<!ELEMENT ports (extraports\* , port\*) \>  

\<!ELEMENT extraports	(extrareasons)\* \>  
\<!ATTLIST extraports  
 state %port\_states;	#REQUIRED  
 count %attr\_numeric; #REQUIRED	  
\>  

\<!ELEMENT extrareasons EMPTY \>  
\<!ATTLIST extrareasons  
 reason CDATA	#REQUIRED  
 count CDATA	#REQUIRED  
 proto	%port\_protocols; #IMPLIED  
 ports CDATA	#IMPLIED  
\>  

\<!ELEMENT port (state , owner? , service?, script\*) \>  
\<!ATTLIST port  
 protocol	%port\_protocols;	#REQUIRED  
 portid %attr\_numeric;	#REQUIRED  
\>  

\<!ELEMENT state EMPTY \>  
\<!ATTLIST state  
 state %port\_states;	#REQUIRED   
 reason CDATA	#REQUIRED  
 reason\_ttl	CDATA	#REQUIRED   
 reason\_ip	CDATA	#IMPLIED  
\>  

\<!ELEMENT owner EMPTY \>  
\<!ATTLIST owner name CDATA #REQUIRED \>  

\<!ELEMENT service	(cpe\*) \>  
\<!ATTLIST service  
 name CDATA #REQUIRED  
 conf %service\_confs;	#REQUIRED  
 method (table|probed) #REQUIRED  
 version CDATA #IMPLIED  
 product CDATA #IMPLIED  
 extrainfo CDATA #IMPLIED  
 tunnel (ssl) #IMPLIED  
 proto (rpc) #IMPLIED  
 rpcnum %attr\_numeric;	#IMPLIED  
 lowver %attr\_numeric;	#IMPLIED  
 highver %attr\_numeric;	#IMPLIED  
 hostname CDATA #IMPLIED  
 ostype CDATA #IMPLIED  
 devicetype CDATA #IMPLIED  
 servicefp CDATA #IMPLIED  
\>  

\<!ELEMENT cpe (#PCDATA)\>  

\<!ELEMENT script	(#PCDATA|table|elem)\* \>  
\<!ATTLIST script	  
 id	CDATA	#REQUIRED  
 output	CDATA	#REQUIRED  
\>  

\<!ELEMENT table	(table|elem)\* \>  
\<!ATTLIST table  
 key CDATA #IMPLIED  
\>  

\<!ELEMENT elem	(#PCDATA)\>  
\<!ATTLIST elem  
 key CDATA #IMPLIED  
\>  

\<!ELEMENT os ( portused\* , osmatch\*, osfingerprint\* ) \>  

\<!ELEMENT portused	EMPTY \>  
\<!ATTLIST portused  
 state %port\_states;	#REQUIRED  
 proto %port\_protocols; #REQUIRED  
 portid %attr\_numeric;	#REQUIRED  
\>  
\<!ELEMENT osclass (cpe\*) \>  
\<!ATTLIST osclass  
 vendor CDATA #REQUIRED  
 osgen CDATA #IMPLIED  
 type CDATA #IMPLIED  
 accuracy CDATA #REQUIRED  
 osfamily CDATA #REQUIRED  
\>  

\<!ELEMENT osmatch	(osclass\*) \>  
\<!ATTLIST osmatch  
 name CDATA #REQUIRED  
 accuracy	%attr\_numeric;	#REQUIRED  
 line %attr\_numeric;	#REQUIRED  
\>  

\<!ELEMENT osfingerprint	EMPTY \>  
\<!ATTLIST osfingerprint  
 fingerprint	CDATA #REQUIRED  
\>  

\<!ELEMENT distance	EMPTY \>  
\<!ATTLIST distance  
 value %attr\_numeric;	#REQUIRED  
\>  

\<!ELEMENT uptime	EMPTY \>  
\<!ATTLIST uptime  
 seconds %attr\_numeric;	#REQUIRED  
 lastboot	CDATA #IMPLIED  
\>  

\<!ELEMENT tcpsequence	EMPTY \>  
\<!ATTLIST tcpsequence  
 index %attr\_numeric;	#REQUIRED  
 difficulty	CDATA #REQUIRED  
 values CDATA #REQUIRED  
\>  

\<!ELEMENT ipidsequence	EMPTY \>  
\<!ATTLIST ipidsequence  
 class CDATA #REQUIRED  
 values CDATA #REQUIRED  
\>  

\<!ELEMENT tcptssequence	EMPTY \>  
\<!ATTLIST tcptssequence  
 class CDATA #REQUIRED  
 values CDATA #IMPLIED  
\>  

\<!ELEMENT trace (hop\*) \>  
\<!ATTLIST trace  
 proto CDATA #IMPLIED  
 port CDATA #IMPLIED  
\>  

\<!ELEMENT hop EMPTY\>  
\<!ATTLIST hop  
 ttl CDATA #REQUIRED  
 rtt CDATA #IMPLIED  
 ipaddr CDATA #IMPLIED  
 host CDATA #IMPLIED  
\>  

\<!ELEMENT times EMPTY\>  
\<!ATTLIST times  
 srtt	CDATA	#REQUIRED  
 rttvar	CDATA	#REQUIRED  
 to	CDATA	#REQUIRED  
\>  

\<!-- For embedding another type of output (screen output) like Zenmap does. --\>  
\<!ELEMENT output (#PCDATA)\>  
\<!ATTLIST output type (interactive) #IMPLIED\>  

\<!-- these elements are generated in output.c:printfinaloutput() --\>  
\<!ELEMENT runstats	(finished, hosts)\>  

\<!ELEMENT finished	EMPTY \>  
\<!ATTLIST finished	time %attr\_numeric;	#REQUIRED   
 timestr CDATA #IMPLIED  
 elapsed %attr\_numeric;	#REQUIRED  
 summary CDATA #IMPLIED  
 exit (error|success) #IMPLIED  
 errormsg	CDATA #IMPLIED  
\>  

\<!ELEMENT hosts EMPTY \>  
\<!ATTLIST hosts  
 up %attr\_numeric;	"0"  
 down %attr\_numeric;	"0"  
 total %attr\_numeric;	#REQUIRED  
\>  

\<!ELEMENT hostscript ( script+ )\>  
\<!ELEMENT prescript ( script+ )\>  
\<!ELEMENT postscript ( script+ )\>  

---

[Prev](https://nmap.org/book/app-nmap-dtd.html)Appendix A. Nmap XML Output DTD

[Up](https://nmap.org/book/app-nmap-dtd.html)Appendix A. Nmap XML Output DTD

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/idx.html)Index