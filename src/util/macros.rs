#![allow(unused_macros)]
macro_rules! chmin { ($a: expr, $b: expr) => { if $a > $b { $a = $b; true } else { false } }; }
macro_rules! chmax { ($a: expr, $b: expr) => { if $a < $b { $a = $b; true } else { false } }; }
