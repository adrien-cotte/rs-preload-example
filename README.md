Example of LD_PRELOAD usage with RUST
=====================================

BUILD
-----

```
$ make
```

USAGE
-----

```
$ export LD_PRELOAD=target/release/librs_preload_example.so
$ <cmd>
```

EXAMPLE
-------

```
$ export LD_PRELOAD=target/release/librs_preload_example.so
$ hostname
== PRELOAD - INIT
== Ok(
    Statm {
        size: 4767,
        resident: 350,
        share: 316,
        text: 3,
        data: 92,
    },
)
==

localhost

== Ok(
    Statm {
        size: 4767,
        resident: 350,
        share: 316,
        text: 3,
        data: 92,
    },
)
==
== PRELOAD - FINI
```

AUTHORS
-------

* Adrien Cotte <adrien@cotte.com>
