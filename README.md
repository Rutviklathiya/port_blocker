# Blackberry Technical Assessment For Rust and eBPF

This project is given as a technical assessment to determine the following

* Knowledge of eBPF programming using the C programming language
* Knowledge of Linux Networking
* Ability to learn basic Rust programming techniques

Candidates are expected to change this skeleton project to suit the requirements
given below.

## Background 

### Rust

The Rust programming language is novel, but is quickly becoming a fundamental
language in three areas of product development

* Low-level System development - Rust has performance characteristics similar
  to C, and was designed to interoperate easily with existing C projects.
  As such, Rust is expected to become an alternative programming language
  used by the Linux kernel for in-tree device drivers.[1]
* High performance Multi-threaded programs - Rust provides parallel and concurrent
  primiatives which are safer and easier to use than other non-garbage collected
  languages.  Rust Web servers have scored quite competitively on benchmarking
  tests[2]
* Web programming - Rust easily compiles to Web Assembly WASM.  WASM is showing
  great promise for developing high performance web applications.

Because of this, Blackberry has chosen Rust as a fundamental building block
in it's Cloud Workload Protection Platform.

Rust is a very new language, and at this time, Blackberry cannot expect all
skilled developers to be experts with it.  However, Blackberry does need
developers to demonstrate that they can quickly get familiar with the language.
The required Rust changes to this project are small, but should adaquately
demonstrate that a Candiate has the ability to become productive teammates.

### eBPF

eBPF is a type of virtual machine where instructions can be run within the 
Linux Kernel.  eBPF provides a much safer and more portable method of running
instructions when compared to Linux Kernel Modules.  At this time, eBPF code
which interacts with Rust can be written in C (via the libbpf-rs crate)[3], or
in Rust (via the Aya crate)[4].

Blackberry has chosen libbpf-rs to interact with eBPF for the
Cloud Workload Protection Platform, as eBPF programs written in C are familiar
with developers who have had experience on the Linux Kernel. Candidates are
expected to be very familiar with C code, but this Technical Assessment skeleton
should provide (nearly) all interactions between Rust and eBPF.

### Linux Networking

Linux Networking is a key component in all cloud workloads.  Candidates are
expected to demonstrate a high degree of knowledge about linux networking while
writing eBPF programs.

[1] - https://www.phoronix.com/news/Rust-v8-For-Linux-Kernel

[2] - https://www.techempower.com/benchmarks/#section=data-r21&hw=ph&test=fortune

[3] - https://crates.io/crates/libbpf-rs

[4] - https://crates.io/crates/aya

## Technical Assessment Requirements

When complete, this project should be able to create a Linux Binary which does
the following

- Blocks all incoming TCP and incoming UDP network packets
  except those ports given as arguments.
- Allows all other traffic

Certain errors exist in both the Rust code, and in the C/eBPF code. The
Candidate is expected to identify and fix these errors before submission.
Beyond these errors, Candidates will need to complete other sections of code
to meet project requirements.

### The Scenario

NOTE: This scenario is fictional and intended to guide Candidates towards an
acceptable solution, but limit the amount of coding effort such that a full
production ready solution is not required.  Blackberry is not expecting the code
provided by the Candidate to be immediately productizable, but it should meet 
basic requirements.

You have joined the development team at OzCorp working on the
Cloud Observation Project (COP).  The team is currently three months away
from releasing its first public beta, but the testing team is currently blocked.

The testing team requires a basic firewall to be running on their testing 
Alma Linux 9 virtual machines.  The distribution provided firewall run by 
firewalld has been determined to be not acceptable, so the development team
has been tasked to provide an eBPF based firewall running on XDP.
Norman Osborn, the CEO of OzCorp, wants this firewall to be developed entirely
in-house, so another off-the-shelf solution such as an IPTABLES script, nor
the Uncomplicated Firewall (ufw) cannot be used.

Your co-worker, Peter Parker, was originally given the task, but he has had an
emergency to attend to and is not available to finish the work.  Your manager,
Mary Jane Watson, has asked you to complete Peter's work.  Mary Jane has told
you that the firewall doesn't need to be completely bullet proof, but it needs
to function well enough to unblock the testing team.

You have received an email from Peter:
----
From: Peter Parker <pparker@ozcorp.com>

To: Employee <yourname@ozcorp.com>

Subject: Sorry, Here is what I did for the firewall ticket

Hey Employee,

Sorry to dump this on you last minute, but something came up.

There are some issues I haven't yet solved with the skeleton I sent you.

Rust Issues:
1) Some weird move error on `if !args.unload_firewall` in main.rs.  The Rust
compiler is yelling that it can't copy the Args struct.  I tried
`#[derive(Copy)]` but then the compiler yelled at me saying that
allowed_ports doesn't implement Copy, so that didn't work.
I think there should be a way to just share args, but this Rust stuff is hard!

2) I couldn't figure out a way to use libbpf-rs to unpin a previous existing
firewall in the `unload_firewall()` function, so I just removed the
/sys/fs/bpf/my_firewall file.  I think this is fine, but I am sure
Mary Jane will demand a reason for not using libbpf-rs.  Could you figure out
why we can't just `libbpf_rsLink::open()?`  Make a code comment, or maybe keep 
a document called "answers.txt" with the answer for her?
Either method is fine.

3) in the `load_firewall()` function, the call to `attach_xdp()`
only seemed to work with the value of 2.  I'm worried this might cause problems
in the test env. Do you know what this 2 means?  Do we need to do something?

eBPF Issues:
1) When I run the `make bpf` I'm getting a warning.  That needs to be fixed
somehow.  Im using `MAX_PORTS` to communicate to the Rust code the amount of 
allowed ports.

2) Mary Jane wants to keep the `allow_xdp()` function small, so I've broken
out a `get_ports()` function to get the source and destination port from the
incoming packet.  I wasn't able to get to this, but I found some example code
from the libbpf-rs project that we can probably adapt.  That code was for "tc"
and we need to work with "xdp."  Since that code is licenced GPL we can use it
but I am sure Mary Jane will have lots and lots of questions, so it might be
wise to "over document" this code if you use it and explain very thoroughly
whats going on here.

3) That example code I found in for #2 is looking at both source and destination
ports for the check:
```
if (is_port_allowed(ports.dst) || is_port_allowed(ports.src)) {
```
I don't think we need to fix this to unblock the test team, but shouldn't
we only need to look at the destination port for an incoming packet?
Why does the source packet matter?

I got some info from the test team on how they run their virtual machine
They say that they are Running Alma Linux 9 and that the result of doing and
`ip link` is the following
```
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN mode DEFAULT group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
2: ens4: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP mode DEFAULT group default qlen 1000
    link/ether aa:bb:c0:a8:00:00 brd ff:ff:ff:ff:ff:ff
    altname enp0s4
```

They've run a `sudo systemctl disable firewalld` and a
`sudo systemctl stop firewalld` on the machine.  So there is no other firewall
running on their test environment.

The result of running `uname -a` is
```
Linux alma9 5.14.0-70.17.1.el9_0.x86_64 #1 SMP PREEMPT Tue Jun 28 14:55:40 EDT 2022 x86_64 x86_64 x86_64 GNU/Linux
```

The test team says that they regularly SSH into the VM, they use something
called "cockpit" to manage the machine, and sometimes they run netcat on port
9000.  They want to be able to block and unblock these by re-running the tool.
They need to be able to ping the box, even when the firewall is running.

I've included a Makefile for quick compilation and testing
the `make load` target will compile the program and load the firewall
the `make unload` will compile the program and unload the firewall
the `make build` will compile the program
the `make clean` does a cargo clean, but also gets rid of the
  generated bpf skeleton files
the `make bpf` does a C based compilation used for testing if the bpf code
  compiles properly, the objects generated are not used anywhere
the `make chk` does a pre-submission check. It makes sure that there aren't
any warnings in C/Rust or formatting issues in Rust and it runs the rust linter.

Make sure you have a clean `make chk` before submission for Mary Jane.

I hope I've given you enough information to get this document

Yours Truly
Peter Parker

----

## Notes Regarding this Assessment

src/bpf/block.bpf.c is GPL-3.0 licenced code, all other files are Copyright
Blackberry, ltd.

Follow up questions about Candidate solutions may be requested after
submission of the solved program.
