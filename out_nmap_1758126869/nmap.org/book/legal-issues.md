---
title: "Legal Issues | Nmap Network Scanning"
source_url: https://nmap.org/book/legal-issues.html
fetched_at: 2025-09-17T16:38:38.812061+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 1. Getting Started with Nmap](https://nmap.org/book/intro.html)
* Legal Issues

[Prev](https://nmap.org/book/nmap-phases.html)

[Next](https://nmap.org/book/history-future.html)

Legal Issues
----------

[]()

When used properly, Nmap helps protect your network from
invaders. But when used improperly, Nmap can (in rare cases) get you
sued, fired, expelled, jailed, or banned by your
ISP.[]()[]()Reduce your
risk by reading this legal guide before launching Nmap.

### Is Unauthorized Port Scanning a
Crime? ###

The legal ramifications of scanning networks with
Nmap are complex and so controversial that third-party organizations
have even printed T-shirts and bumper stickers promulgating opinions
on the matter[<sup class="footnote" id="idm45818757338768">[6]</sup>](https://nmap.org/book/legal-issues.html#ftn.idm45818757338768), as shown in[Figure 1.3](https://nmap.org/book/legal-issues.html#intro-psinac-fig). The topic also draws many
passionate but often unproductive debates and flame wars. If you ever
participate in such discussions, try to avoid the overused and
ill-fitting analogies to knocking on someone's home door or testing
whether his door and windows are locked.

Figure 1.3. Strong opinions on port scanning legality and morality

![Strong opinions on port scanning legality and morality](images/PSINAC_Combo_820x500.jpg)

While I agree with the sentiment that port scanning*should not* be illegal, it is rarely wise to take
legal advice from a T-shirt. Indeed, taking it from a software
engineer and author is only slightly better. Speak to a competent
lawyer[]()within your jurisdiction for a better understanding of how the
law applies to your particular situation. With that important
disclaimer out of the way, I'll provide some general information that may
prove helpful.

The best way to avoid controversy when using Nmap is to always
secure written authorization from the target network representatives
before initiating any scanning. There is still a chance that your
ISP[]()will give you trouble if they notice it (or if the target
administrators accidentally send them an abuse report), but this is
usually easy to resolve. When you are performing a
penetration test,[]()this authorization should be in the Statement of Work. When testing
your own company, make certain that this activity clearly falls within
your job description. Security consultants should be familiar with
the excellent [Open Source Security
Testing Methodology Manual (OSSTMM)](http://www.osstmm.org/),[]()[]()which provides best practices for these situations.

While civil and (especially) criminal court cases are the
nightmare scenario for Nmap users, these are very rare. After
all, no United States federal laws explicitly criminalize port scanning. A much
more frequent occurrence is that the target network will notice a scan
and send a complaint to the network service provider where the
scan initiated (your ISP). Most network administrators do not seem to care or
notice the many scans bouncing off their networks daily, but a few
complain. The scan source ISP may track down the user corresponding to
the reported IP address and time, then chide the user or even kick
him off the service. Port scanning without authorization is
sometimes against the provider's acceptable use policy
(AUP).[]()[]()For
example, the AUP for the huge cable-modem ISP Comcast says:

>
>
> Network probing or port scanning tools are
> only permitted when used in conjunction with a residential home
> network, or if explicitly authorized by the destination host and/or
> network. Unauthorized port scanning, for any reason, is strictly
> prohibited.
>
>

Even if an ISP does not explicitly ban unauthorized port
scanning, they might claim that some “anti-hacking” provision applies.
Of course this does *not* make port scanning
illegal. Many perfectly legal and (in the United States)
constitutionally protected activities are banned by ISPs. For
example, the AUP quoted above also prohibits users from
transmitting, storing, or posting “any information or material
which a reasonable person could deem to be objectionable, offensive,
indecent, pornographic, ... embarrassing, distressing, vulgar,
hateful, racially or ethnically offensive, or otherwise inappropriate,
regardless of whether this material or its dissemination is
unlawful”. In other words, some ISPs ban any behavior that
could possibly offend or annoy someone[<sup class="footnote" id="idm45818757320224">[7]</sup>](https://nmap.org/book/legal-issues.html#ftn.idm45818757320224). Indiscriminate scanning of
other people's networks does have that potential.
If you decide to perform such controversial scanning anyway, never do
it from work, school, or any other service provider that has
substantial control over your well-being. Use a commercial
broadband or wireless provider instead. Losing your DSL connection and having to change
providers is a slight nuisance, but it is immeasurably preferable to
being expelled or fired.

While legal cases involving port scanning (without follow-up
hacking attacks) are rare, they do happen. One of the most notable
cases involved a man named
Scott Moulton[]()who had an ongoing consulting
contract to maintain the Cherokee County, Georgia emergency 911
system. In December 1999, he was tasked with setting up a router
connecting the Canton, Georgia Police Department with the E911 Center.
Concerned that this might jeopardize the E911 Center security, Scott
initiated some preliminary port scanning of the networks involved. In
the process he scanned a Cherokee County web server that was owned and
maintained by a competing consulting firm named VC3. They noticed the
scan and emailed Scott, who replied that he worked for the 911
Center and was testing security. VC3 then reported the activity to
the police. Scott lost his E911 maintenance contract and was
arrested for allegedly violating the
Computer Fraud and Abuse Act[]()of
America [Section
1030(a)(5)(B)](http://www4.law.cornell.edu/uscode/18/1030.html).
This act applies against anyone who “intentionally accesses a
protected computer without authorization, and as a result of such
conduct, causes damage” (and meets other requirements). The
damage claimed by VC3 involved time spent investigating the port scan
and related activity. Scott sued VC3 for defamation, and VC3 countersued for violation of the Computer Fraud and Abuse Act
as well as the Georgia Computer Systems Protection Act.

The civil case against Scott was dismissed before trial, implying a complete lack of merit. The ruling made many Nmap users smile:

>
>
> “Court holds that plaintiff's act of conducting an unauthorized port scan and throughput test of defendant's servers does not constitute a violation of either the Georgia Computer Systems Protection Act or the Computer Fraud and Abuse Act.”—Civ. Act. No. 1:00-CV-434-TWT (N.D. Ga. November 6, 2000)
>
>

This was an exciting victory in the civil case, but Scott still
had the criminal charges pending. Fortunately he kept
his spirits high, sending the following [note](https://seclists.org/nmap-hackers/2001/26) to
the*nmap-hackers* mailing list:[]()

>
>
> I am proud that I could be of some benefit to the
> computer society in defending and protecting the rights of specialists
> in the computer field, however it is EXTREMELY costly to support such
> an effort, of which I am not happy about. But I will continue to fight
> and prove that there is nothing illegal about port scanning especially
> when I was just doing my job.
>
>

Eventually, the criminal court came to the same conclusion and
all charges were dropped. While Scott was vindicated in the end, he
suffered six-figure legal bills and endured stressful years
battling through the court system. The silver lining is that after spending so much time educating his lawyers about the technical issues involved, Scott started a [successful forensics services company](http://www.forensicstrategy.com/).

While the Moulton case sets a good example (if not legal
precedent), different courts or situations could still lead to worse
outcomes. Remember that many states have their own computer abuse
laws, some of which can arguably make even pinging a remote machine
without authorization illegal[<sup id="idm45818757306000" class="footnote">[8]</sup>](https://nmap.org/book/legal-issues.html#ftn.idm45818757306000).

Laws in other nations obviously differ as well. For example, A
17-year-old youth was [convicted in Finland](https://insecure.org/stf/fin.html) of attempted computer intrusion for simply port
scanning a bank. He was fined to cover the target's investigation expenses. The
Moulton ruling might have differed if the VC3 machine had
actually crashed and they were able to justify the $5,000 damage
figure required by the act.

At the other extreme, an Israeli judge [acquitted](http://www.theregister.co.uk/2004/03/01/mossad_website_hacker_walks_free/)Avi Mizrahi[]()in early 2004 for vulnerability scanning the Mossad secret service.
Judge Abraham Tennenbaum even commended Avi in his ruling:

>
>
> In a way, Internet surfers who check the
> vulnerabilities of Web sites are acting in the public good. If their
> intentions are not malicious and they do not cause any damage, they
> should even be praised.
>
>

In 2007 and 2008, broad new cybercrime laws took effect in[Germany](http://www.beskerming.com/commentary/2007/08/12/249/German_Security_Professionals_in_the_Mist)and [England](http://www.theregister.co.uk/2008/01/02/hacker_toll_ban_guidance/).
These laws are meant to ban the distribution, use, and even possession
of “hacking tools”. For example, the UK amendment to the
Computer Misuse Act[]()makes it illegal to “supply or offer to
supply [a program], believing that it is likely to be used to commit, or to assist
in the commission of [a Computer Misuse Act violation]”. These
laws have already led some security tool authors to close shop or move
their projects to other countries. The problem is that most security
tools can be used by both ethical professionals
(white-hats)[]()to defend their networks and
black-hats[]()to attack. These dangerous laws are
based on the tool author or user's intent, which is subjective and
hard to divine. Nmap was designed to help secure the Internet, but
I'd hate to be arrested and forced to defend my intentions to a judge
and jury, especially in a foreign country like Germany where I don't even speak the language. These laws are unlikely to affect tools as widespread and
popular as Nmap, but they have had a chilling effect on smaller tools
and those which are more commonly abused by computer criminals (such
as exploitation frameworks).

Regardless of the legal status of port scanning, ISP accounts
will continue to be terminated if many complaints are generated. The
best way to avoid ISP abuse reports or civil/criminal charges is to
avoid annoying the target network administrators in the first place.
Here are some practical suggestions:

[]()

* Ensure that you have permission to scan. Probably at least 90% of network scanning is
  non-controversial. You are rarely badgered for scanning your own
  machine or the networks you administer. The controversy comes when
  scanning other networks. There are many reasons (good and bad) for
  doing this sort of network exploration. Perhaps you are scanning the
  other systems in your dorm or department to look for publicly shared
  files (FTP, SMB, WWW, etc.). Or maybe you are just trying to find the
  IP address of a certain printer. You might have scanned your favorite web
  site to see if they are offering any other services, or because you
  were curious what OS they run. Perhaps you are just trying to test
  connectivity, or maybe you wanted to do a quick security sanity check
  before handing off your credit card details to that e-commerce
  company. You might be conducting Internet research. Or are you
  performing initial reconnaissance in preparation for a break-in
  attempt? The remote administrators rarely know your true intentions,
  and do sometimes get suspicious. The best approach is to get
  permission first. I have seen a few people with non-administrative
  roles land in hot water after deciding to “prove” network
  insecurity by launching an intrusive scan of the entire company or
  campus. Administrators tend to be more cooperative when asked in
  advance than when woken up at 3:00 AM by an IDS alarm claiming they are
  under massive attack. So whenever possible, obtain written
  authorization before scanning a network. Adrian Lamo would probably
  have avoided jail if he had asked the New York Times to test their
  security rather than telling reporters about the flaws afterward.
  Unfortunately they might have said no. Be prepared for this
  answer.

* Target your scan as tightly as possible. Any machine
  connected to the Internet is scanned regularly enough that most
  administrators ignore such Internet background noise. But scanning enough
  networks or executing very noisy/intrusive scans increases the
  probability of generating complaints. So if you are only looking for
  web servers, specify `-p80` rather than scanning all 65,536 TCP ports on
  each machine. If you are only trying to find available hosts, do an
  Nmap ping scan rather than full port scan. Do not scan a
  CIDR[]()/16 (65K hosts) when a /24 netblock
  suffices. The random scan mode now takes an argument specifying
  the number of hosts, rather than running forever. So consider `-iR 1000`rather than `-iR 10000` if the former is sufficient. Use the default
  timing (or even `-T polite`) rather than`-T insane`.[]()Avoid noisy
  and relatively intrusive scans such as version detection (`-sV`) or NSE (`--script`).
  Similarly, a SYN scan
  (`-sS`)[]()is quieter than a connect scan
  (`-sT`)[]()while providing the same information and often being
  faster.

* As noted previously, do not do anything controversial
  from your work or school connections. Even though your intentions may
  be good, you have too much to lose if someone in power (e.g. boss, dean)
  decides you are a malicious cracker. Do you really want to explain
  your actions to someone who may not even know what port scanning means? Spend $40 a month for a shell, cell data, or residential broadband account. Not only are the repercussions less
  severe if you offend someone from such an account, but target network
  administrators are less likely to even bother complaining to mass-market
  providers. Also read the relevant AUP and choose a provider
  accordingly. If your provider (like Comcast discussed above) bans any
  unauthorized port scanning and posting of “offensive” material, do not
  be surprised if you are kicked off for this activity. In general, the
  more you pay to a service provider the more accommodating they are.
  A T1 provider is highly unlikely to yank your connection without
  notice because someone reported being port scanned. A dialup or
  residential DSL/cable provider very well might. This can happen even
  when the scan was forged by someone
  else.

* Nmap offers many options for stealthy scans, including
  source-IP spoofing, decoy scanning, and the more recent idle scan
  technique. These are discussed in the IDS evasion chapter. But
  remember that there is always a trade-off. You are harder to find if
  you launch scans from an open WAP far from your house, with 17 decoys,
  while doing subsequent probes through a chain of nine open proxies. But if
  anyone does track you down, they will be mighty suspicious of your
  intentions.

* Always have a legitimate reason for performing scans.
  An offended administrator might write to you first (or your ISP might forward his
  complaint to you) expecting some sort of justification for the
  activity. In the
  Scott Moulton[]()case discussed above, VC3 first emailed
  Scott to ask what was going on. If they had been satisfied with his
  answer, matters might have stopped there rather than escalating into
  civil and criminal litigation. When I scan large portions of the
  Internet for research purposes, I use a reverse-DNS name that
  describes the project and run a web server on that IP address with detailed information and opt-out instructions.

Also remember that ancillary and subsequent actions are often
used as evidence of intent. A port scan by itself does not always
signify an attack. A port scan followed closely by an IIS exploit,
however, broadcasts the intention loud and clear. This is important
because decisions to prosecute (or fire, expel, complain, etc.) are
often based on the whole event and not just one component (such as a
port scan).

One dramatic case involved a Canadian man named Walter
Nowakowski, who was apparently the first person to be charged in
Canada with theft of communications ([Canadian
Criminal Code Section S.342.1](http://www.crime-research.org/library/Canada_Code.html)) for accessing the Internet
through someone's unsecured Wi-Fi network. Thousands of Canadian“war drivers” do this every day, so why was he singled
out? Because of ancillary actions and intent. He was allegedly[caught](http://www.theregister.co.uk/2003/11/26/wifi_hacker_caught_downloading_child/) driving the wrong way on a one-way street, naked from the waist
down, with laptop in hand, while downloading child pornography through
the aforementioned unsecured wireless access
point.
The police apparently considered his activity egregious enough that
they brainstormed for relevant charges and tacked on theft of
communications to the many child pornography-related charges.

Similarly, charges involving port scanning are usually reserved
for the most egregious cases. Even when paranoid administrators
notify the police that they have been scanned, prosecution (or any
further action) is exceedingly rare. The fact that a 911 emergency
service was involved is likely what motivated prosecutors in the
Moulton case. Your author scanned millions of Internet hosts while
writing this book and received fewer than ten complaints.

To summarize this whole section, the question of whether port
scanning is legal does not have a simple answer. I cannot
unequivocally say “port scanning is never a crime”, as much as I would
like to. Laws differ dramatically between jurisdictions, and cases
hinge on their particular details. Even when facts are nearly
identical, different judges and prosecutors do not always interpret
them the same way. I can only urge caution and reiterate the
suggestions above.

For testing purposes, you have permission to scan the host`scanme.nmap.org`.[]()You may have noticed it
used in several examples already. Note that this permission only
includes scanning with Nmap and not testing exploits or denial of
service attacks. To conserve bandwidth, please do not initiate more
than a dozen scans against that host per day. If this free scanning
target service is abused, it will be taken down and Nmap will report`Failed to resolve given hostname/IP:
scanme.nmap.org`.

### Can Port Scanning Crash the Target Computer/Networks? ###

[]()

Nmap does not have any features designed to crash target
networks. It usually tries to tread lightly. For example,
Nmap detects dropped packets and slows down when they occur in order
to avoid overloading the network. Nmap also does not send any corrupt
packets. The IP, TCP, UDP, and ICMP headers are always appropriate, though the
destination host is not necessarily expecting the packets. For these
reasons, no application, host, or network component*should* ever crash based on an Nmap scan. If they
do, that is a bug in the system which should be repaired by the
vendor.

Reports of systems being crashed by Nmap are rare, but they do
happen. Many of these systems were probably unstable in the first
place and Nmap either pushed them over the top or they crashed at the
same time as an Nmap scan by pure coincidence. In other cases, poorly
written applications, TCP/IP stacks, and even operating systems have
been demonstrated to crash reproducibly given a certain Nmap command.
These are usually older legacy devices, as newer equipment is rarely
released with these problems. Smart companies use Nmap and many other
common network tools to test devices prior to shipment. Those who omit such pre-release testing often find out about the problem in early beta tests when a
box is first deployed on the Internet. It rarely takes long for a
given IP to be scanned as part of Internet background noise. Keeping
systems and devices up-to-date with the latest vendor patches and
firmware should reduce the susceptibility of your machines to these problems, while also improving the security and usability of your network.

In many cases, finding that a machine crashes from a certain
scan is valuable information. After all, attackers can do anything
Nmap can do by using Nmap itself or their own custom scripts. Devices
should not crash from being scanned and if they do, vendors should be pressured
to provide a patch. In some usage scenarios, detecting fragile machines by crashing them is undesirable. In those cases you may want to perform very light scanning to reduce
the risk of adverse effects. Here are a few suggestions:

[]()[]()

* Use SYN scan (`-sS`) instead of connect scan (`-sT`). User-mode applications such as web servers can rarely even detect the former because it is all handled in kernel space and thus the services have no excuse to crash.

* Version scanning (`-sV`) and some of our NSE scripts (`-sC` or `--script`) risk crashing poorly written applications. Similarly, some buggy operating systems have been reported to crash when OS fingerprinted (`-O`). Omit these options for particularly sensitive environments or where you do not need the results.

* Using `-T2`[]()or slower (`-T1`,[]()`-T0`)[]()timing modes can reduce the chances that a port scan will harm a system, though they slow your scan dramatically. Older Linux boxes had an identd daemon that would block services temporarily if they were accessed too frequently. This could happen in a port scan, as well as during legitimate high-load situations. Slower timing might help here. These slow timing modes should only be used as a last resort because they can slow scans by an order of magnitude or more.

* Limit the number of ports and machines scanned to the
  fewest that are required. Every machine scanned has a minuscule
  chance of crashing, and so cutting the number of machines down
  improves your odds. Reducing the number of ports scanned reduces the
  risks to end hosts as well as network devices. Many NAT/firewall
  devices keep a state entry for every port probe. Most of them expire
  old entries when the table fills up, but occasional (pathetic)
  implementations crash instead. Reducing the ports and hosts scanned
  reduces the number of state entries and thus might help those fragile and defective devices stay up.

[]()

### Nmap Copyright ###

[]()

While Nmap is open source, it still has a copyright license that
must be respected. As free software, Nmap also carries no
warranty. These issues are covered in much greater detail in [the section called “Legal Notices”](https://nmap.org/book/man-legal.html). Companies wishing to bundle and use Nmap within
proprietary software and appliances are especially encouraged to read
this section so they don't inadvertently violate the Nmap license. Fortunately the Nmap Project sells commercial redistribution licenses for companies which need one.

---

[<sup class="para">[6] </sup>](https://nmap.org/book/legal-issues.html#idm45818757338768)These are from the now-defunct
AmericanSushi.Com.

[<sup class="para">[7] </sup>](https://nmap.org/book/legal-issues.html#idm45818757320224)The Comcast AUP was improved after this was first published. The latest version is available at [`http://www.comcast.net/terms/use/`](http://www.comcast.net/terms/use/)

[<sup class="para">[8] </sup>](https://nmap.org/book/legal-issues.html#idm45818757306000)An excellent paper on
this topic by lawyer Ethan Preston is available at[`http://grove.ufl.edu/~techlaw/vol6/issue1/preston.html`](http://grove.ufl.edu/~techlaw/vol6/issue1/preston.html).
He has also written an excellent paper relating to the legal risks of
publishing security information and exploits at[`http://www.mcandl.com/computer-security.html`](http://www.mcandl.com/computer-security.html).

---

[Prev](https://nmap.org/book/nmap-phases.html)The Phases of an Nmap Scan

[Up](https://nmap.org/book/intro.html)Chapter 1. Getting Started with Nmap

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/history-future.html)The History and Future of Nmap