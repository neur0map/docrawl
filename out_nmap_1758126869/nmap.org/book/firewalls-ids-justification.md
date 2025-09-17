---
title: "Why Would Ethical Professionals (White-hats) Ever Do This? | Nmap Network Scanning"
source_url: https://nmap.org/book/firewalls-ids-justification.html
fetched_at: 2025-09-17T16:44:17.846679+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 10. Detecting and Subverting Firewalls and Intrusion Detection Systems](https://nmap.org/book/firewalls.html)
* Why Would Ethical Professionals (White-hats) Ever Do This?

[Prev](https://nmap.org/book/firewalls.html)

[Next](https://nmap.org/book/determining-firewall-rules.html)

Why Would Ethical Professionals (White-hats) Ever Do This?
----------

[]()

Some of you white-hat readers may be tempted to skip this
chapter. For authorized use against your own networks, why would you
ever want to evade your own security systems? Because it helps in
understanding the danger of real attackers. If you can sneak around a
blocked portmapper port using Nmap direct
RPC scanning,[]()then so can
the bad guys. It is easy to make a mistake in configuring complex
firewalls and other devices. Many of them even come with glaring
security holes which conscientious users must find and close. Regular
network scanning can help find dangerous implicit rules (for example, in your
Checkpoint Firewall-1 or Windows IPsec filters) before attackers
do.

There are good reasons for evading IDSs as well. Product
evaluation is one of the most common. If attackers can slide under
the radar by simply adding an Nmap flag or two, the system is not
offering much protection. It may still catch the script
kiddies[]() and
worms, but they are usually blazingly obvious anyway.

Occasionally people suggest that Nmap should not offer features
for evading firewall rules or sneaking past IDSs. They argue
that these features are just as likely to be misused by attackers as
used by administrators to enhance security. The problem with this
logic is that these methods would still be used by attackers, who
would just find other tools or patch the functionality into Nmap.
Meanwhile, administrators would find it that much harder to do their
jobs. Deploying only modern, patched FTP servers is a far more
powerful defense than trying to prevent the distribution of tools
implementing the FTP bounce attack.

---

[Prev](https://nmap.org/book/firewalls.html)Chapter 10. Detecting and Subverting Firewalls and Intrusion Detection Systems

[Up](https://nmap.org/book/firewalls.html)Chapter 10. Detecting and Subverting Firewalls and Intrusion Detection Systems

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/determining-firewall-rules.html)Determining Firewall Rules