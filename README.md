# dark-star-controller

This is a controller/monitor system designed for linux with hard coded thresholds for reactions.
The reactions can be done based on crossing thresholds and then not done again until "healed" and a second event crosses the threshold, etc.

The reactions are coded in `reactions.rs`, by default mostly they are collecting the kernel message buffer and running other commands like ps and df, but also doing some disk space cleaning and some other example.
The output is simply going to STDOUT, which can be used adhoc and interactively on the CLI or daemonized if run within a container or systemd, which will then jave the STDOUT data be collected within the logging system.
The logging may then be further used by other systems for further event correlation and data analysis.

<b>Adjust the reactions to what is needed before running on a real system!</b>

The thresholds are also hard coded intentionally (to reduce tampering potential and reliance on disk files).
<b>Adjust the thresholds to what is needed on a case-by-case basis! This means tuning the binary to fit the intended system use-case!</b>

Default thresholds:
90% disk usage in /
90% disk usage in /var
1000 ESTABLISHED TCP connections
2000 running system processes/tasks
load average over # cores (this one isn't hard coded and the threshold is assigned to the number of CPUs/vCPUs on the system, so 4 cores will make a load average of 4 the threshold)

## Resource utilizatoin

This program isn't exactly light weight as it samples the system every 1999ms, constantly recalculating and rechecking values. Continued reduction in resource consumption may be made.
Interestingly the disk calculation is less efficient here than it would be to simply Command call df, so that is a target for optimziation.


## More

Also see https://github.com/jpegleg/frontline-controller/ as a similar project that uses persistent storage in redis and a policy file instead of all hard-coded and ephemeral.

