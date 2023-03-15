Extra unused symbols in rust library
====================================

If we include `lib1` as a dep, and `use` something from it, like a data structure, we find that the resulting cdylib contains the function `foo` from `lib1` even though  it is not used.

    $ cd lib2/
    $ cargo build -r
    $ nm -D target/release/liblib2.so
    0000000000001100 T bar
                     w __cxa_finalize@GLIBC_2.2.5
    0000000000001100 T foo
                     w __gmon_start__
                     w _ITM_deregisterTMCloneTable
                     w _ITM_registerTMCloneTable

If we comment out the `use` line in `lib2/src/lib.rs`, we find that `foo` is not exported.

    $ set -i 's/use/\/\/use/' src/lib.rs
    $ cargo build -r
    $ nm -D target/release/liblib2.so
    0000000000001100 T bar
                     w __cxa_finalize@GLIBC_2.2.5
                     w __gmon_start__
                     w _ITM_deregisterTMCloneTable
                     w _ITM_registerTMCloneTable

Why does this happen?
