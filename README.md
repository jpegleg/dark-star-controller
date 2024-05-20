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
```
90% disk usage in /
90% disk usage in /var
1000 ESTABLISHED TCP connections
2000 running system processes/tasks
load average over # cores (this one isn't hard coded and the threshold is assigned to the number of CPUs/vCPUs on the system, so 4 cores will make a load average of 4 the threshold)
50% memory usage
```

## Event "cascading" - no need to repeat actions

As samples are collected (using multiple collection mechanisms in rust) and the results impact the "health" value for that metric. All of the health metrics are stored
in a vector of i32s. The i32s used for health have a floor of -2 billion and a ceiling of 100. Values above 100 will instead be 100, and values below 2 billion will be 2 billion.

Thresholds of health per score (or with combined scores) can trigger reactions. If a reaction uses a _marker_, then the marker prevents the same action from being taken until
that health metric is restored to full. Restoration comes from values above threshold during probes, which returns the health score to 100 and removes the marker.


## Ad-hoc use

Rather than daemonizing the dark-star-controller (the binary is "dstrc"), it can also be used as command line utility.

```
$ dstrc -d
[2024-05-20 02:35:31.246380061 UTC INFO] - "dragon" - Started dark-star-controller
[2024-05-20 02:35:31.346880399 UTC DEBUG] - "dragon" [100, 100, 100, 100, 100] -> disk: "68.00961561035817" net: "5" proc: "852" load: "1.3100586" mem: "0.2073705227757457"
[2024-05-20 02:35:33.439508087 UTC DEBUG] - "dragon" [100, 100, 100, 100, 100] -> disk: "68.00961561035817" net: "5" proc: "851" load: "1.3657227" mem: "0.20740493483707487"
[2024-05-20 02:35:35.529249340 UTC DEBUG] - "dragon" [100, 100, 100, 100, 100] -> disk: "68.00961561035817" net: "5" proc: "850" load: "1.3657227" mem: "0.20742951488088143"
[2024-05-20 02:35:37.617374858 UTC DEBUG] - "dragon" [100, 100, 100, 100, 100] -> disk: "68.00961561035817" net: "5" proc: "849" load: "1.3364258" mem: "0.20747572536323775"
^C

```

In this way, we also have preconfigured delays before reaction if we lower the threshold values. It takes interations as each negative metric results in -1 health for that metric position.


This next example is WARN logging (-w mode), and in a daemonized dstrc:

```
May 19 18:27:42 dragon nice[784]: [2024-05-19 22:27:42.890663844 UTC WARN] - "dragon" [100, 98, 100, 100, 100] -> load average over threshold: "4.2280273"
May 19 18:27:45 dragon nice[784]: [2024-05-19 22:27:45.066054471 UTC WARN] - "dragon" [100, 97, 100, 100, 100] -> load average over threshold: "4.0493164"
May 19 18:27:47 dragon nice[784]: [2024-05-19 22:27:47.260031774 UTC WARN] - "dragon" [100, 96, 100, 100, 100] -> load average over threshold: "4.0493164"
May 19 18:27:49 dragon nice[784]: [2024-05-19 22:27:49.482792624 UTC WARN] - "dragon" [100, 95, 100, 100, 100] -> load average over threshold: "5.166504"
May 19 18:27:51 dragon nice[784]: [2024-05-19 22:27:51.663664204 UTC WARN] - "dragon" [100, 94, 100, 100, 100] -> load average over threshold: "5.166504"
May 19 18:27:53 dragon nice[784]: [2024-05-19 22:27:53.853270458 UTC WARN] - "dragon" [100, 93, 100, 100, 100] -> load average over threshold: "5.166504"
May 19 18:27:56 dragon nice[784]: [2024-05-19 22:27:56.088487741 UTC WARN] - "dragon" [100, 92, 100, 100, 100] -> load average over threshold: "5.072754"
May 19 18:27:58 dragon nice[784]: [2024-05-19 22:27:58.257818617 UTC WARN] - "dragon" [100, 91, 100, 100, 100] -> load average over threshold: "5.072754"
May 19 18:28:00 dragon nice[784]: [2024-05-19 22:28:00.435438773 UTC WARN] - "dragon" [100, 90, 100, 100, 100] -> load average over threshold: "5.0668945"
```

In '-w' mode, we don't print healthy iterations, only iterations with negative samples.

In '-d' mode, we print both healthy DEBUG values and WARN negative samples.

If any other arguments are used, then only the start and reaction values will print. The reactions can of course be set to not print, and on a per reaction basis, too.


## Resource utilization

This program isn't exactly light weight as it samples the system every 1999ms, constantly recalculating and rechecking values. Continued reduction in resource consumption may be made.
Interestingly the disk calculation is less efficient here than it would be to simply Command call df, so that is a target for optimziation.




## More

Also see https://github.com/jpegleg/frontline-controller/ as a similar project that uses persistent storage in redis and a policy file instead of all hard-coded and ephemeral.

