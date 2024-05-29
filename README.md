# dark-star-controller

This is a controller/monitor system designed for linux with hard coded thresholds for reactions.
The reactions can be done based on crossing thresholds and then not done again until "healed" and a second event crosses the threshold, etc.

The reactions are coded in `reactions.rs`, by default mostly they are collecting the kernel message buffer and running other commands like ps and df, but also doing some disk space cleaning and some other example.
The output is simply going to STDOUT, which can be used adhoc and interactively on the CLI or daemonized if run within a container or systemd, which will then have the STDOUT data be collected within the logging system.
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
in a vector of i32s. The i32s used for health have a floor of -2 billion and a ceiling of 100. Values above 100 will instead be 100, and values below -2 billion will be -2 billion.

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
$ 
```



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

The reactions take place regardless of whether '-w' or '-d' or other value is passed. The reactions can use bayesian inference or MDP-like algorithms because of the markers and thresholds. We can compute a new probability of an action being the right one across same metric or all metric data, and act specifically based on those calculations, too. This secondary or more developed logic is not included in the template, but the template is set up to enable that. 

The main logic section can be perhaps understood by examining this function, which takes probe data (string output) and converts it to an `f64` to compare it to the threshold value.
The result is an unary float that is later checked to ultimately tip the health score, either -1 or +2000000100 back to 100.

```
fn compare_strings(str1: &str, str2: &f64) -> f64 {
    let mbo = { *str2 };
    if let (Ok(num1), num2) = (str1.parse::<f64>(), mbo) {
        if num1 == num2 {
            0.0
        } else if num1 > num2 {
            1.0
        } else {
            2.0
        }
    } else {
        255.0
    }
}

```

In this way, we also have preconfigured delays before reaction if we lower the threshold values. It takes interations as each negative metric results in -1 health for that metric position.

```
        let disk = dskroot(); // collect the probe sample
        let de = compare_strings(&disk, &90.0); // compare sample to coded threshold using the function "compare_strings"

        if de == 1.0 {
            sim[2] -= 1; // drop the score by 1 if comparison result is 1.0
            let newval = lowend(sim[2]); // keep max depth from going out of i32 range, min of -2 billion
            sim[2] = newval

        }

        if de == 0.0 {
            sim[2] -= 1; // drop the score by 1 if the comparison result is 0.0
            let newval = lowend(sim[2]);
            sim[2] = newval

        }

        if de == 2.0 {  // heal to 100 if the result is 2.0
            if sim[2] < 100 {
                sim[2] += 2000000100
            }
            if sim[2] > 100 {
                sim[2] = 100
            }

        }

```

Then further logic acts when the score dips below a threshold and sets its marker:

```

        if sim[2] < 0 && marker1 < 1 {

            let dskcont = thread::spawn(|| {
                reactions::diskcleaner1();
            });

            dskcont.join().unwrap();
            marker1 = 1;
        }
```

And the marker is reset if the score reaches 100 again.

```
        if sim[2] == 100 {
            marker1 = 0;
        } else if marker1 == 2 {
             _ = 0;
        } else if mode == "-w" {
            let nim: DateTime<Utc> = Utc::now();
            println!("[{} WARN] - {:?} {:?} -> disk over threshold: {:?}", nim, &host, &sim, &disk);
        } else if mode == "-d" {
            let nim: DateTime<Utc> = Utc::now();
            println!("[{} WARN] - {:?} {:?} -> disk over threshold: / {:?} /var {:?}", nim, &host, &sim, &disk, &diskt);
        }
```

The default template has 1 marker per metric, and 1 threshold per metric for reaction, with metrics below 0 also triggering "prolonged negative state" events, that have a WARN print, if -w or -d are used.

Additional thresholds and marker logic is programmed (by the person doing the implentation) to decide on the actions and thresholds, develop the flows and actions or other algorithms within the looping or ahead of the looping.

An example logic flow could be as follows if we wanted to make version entirely focused on disk metrics and disk space management. Of course we could also just increase the vector size and add rather than replce metrics, too.

```
(replace all health scores with disk partition (slice) metrics for use %, as well as inode count, count of open files, bits written, and bits read, for example)

react to 80% slice use with gathering open files on that slice and files over 200MB in size, set marker to not trigger again until after healthy
react to 85% slice use with sending slice data to admins, set marker to not trigger again until after healthy
react to 90% slice use with preconfigured log rotation actions, set marker to not trigger again until after healthy
react to use with score below -2000 to send an escalated alert to admins, set marker to not trigger again until after healthy
react to use with score below -9000 to send a reminder alert to admins and take more extreme measures to attempt to free up disk space

```

We can create low thresholds that only occur if a previous threshold was crossed and previous action taken, and an amount of time passed as the negative counters are only -1 per sample iteration.
So a score threshold of -200000 would take over 4 days to reach, unless of course additional health penalizations are included per round on the same metric. Because we can go down to -2000000000,
we could have 32 years continuous runtime of negative metrics before reaching the end floor, or longer if the iteration sleep value is increased. Of course the floor could be lowered by using f64s or BigInt values, but
we are using i32 to reduce RAM and because we don't need scores below -2 billion in this design. We could reduce resource utilization by not putting the i32s in a vector at all perhaps, but the vector data structure is useful, too.

## Notes about changing the sleep value

The sleep value is set to 1999 milliseconds by default, just under 2 seconds. Increasing it can lighten the load but also provides greater window to miss important samples. The sleep can be safey increased to 30 seconds, and even upwards to 59 seconds. It isn't as strong of a load average metric if we sleep for 60 seconds or more, but it could be done, however slows the rate of reaction below what is intended. Shortening the sleep value to less than 1999ms is fine, but does increase the rate of action and the load on the system. Some systems might have enough spare capacity and desire to run 500ms sleep, for example.


## Example systemd daemonization

Here is a simple systemd unit file for dark-star-controller (dstrc). The binary is deployed to /usr/local/bin/dstrc and set as executable.
This unit file then can be placed in `/etc/systemd/system/dstrc.service`, and then the `sudo systemctl daemon-reload && sudo systemctl enable dstrc && sudo systemctl start dstrc`.

```
[Unit]
Description=dstrc
After=network.target

[Service]
Type=simple
ExecStart=nice /usr/local/bin/dstrc -w
TimeoutStartSec=0

[Install]
WantedBy=multi-user.target
```

The print messages will by default go to the systemd journal files, such as can be accessed with `journalctl -xe`.


## Resource utilization

This program isn't exactly light weight as it samples the system every 1999ms, constantly recalculating and rechecking values. Continued reduction in resource consumption may be made.
Interestingly the disk calculation is less efficient here than it would be to simply Command call df, so that is a target for optimization.



## More

Also see https://github.com/jpegleg/frontline-controller/ as a similar project that uses persistent storage in redis and a policy file instead of all hard-coded and ephemeral.

