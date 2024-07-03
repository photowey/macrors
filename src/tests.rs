/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// ----------------------------------------------------------------

use std::ops::{Add, Sub};
use std::time::{Duration, SystemTime};

use super::*;

#[test]
fn test_ternary_bool() {
    let seed = 10;

    let is_even = ternary!(seed % 2 == 0, true, false);
    assert!(is_even);
}

#[test]
fn test_ternary_i32() {
    let seed = 10;

    let pos = ternary!(seed % 2 == 0, 1, -1);
    let neg = ternary!((seed - 1) % 2 == 0, 1, -1);
    assert_eq!(1, pos);
    assert_eq!(-1, neg);
}

#[test]
fn test_ternary_str() {
    let seed = 10;

    let result = ternary!(seed > 5, "greater than 5", "less than or equal to 5");
    assert_eq!("greater than 5", result);
}

#[test]
fn test_ternary_string() {
    let seed = 10;

    let result = ternary!(
        seed > 5,
        String::from("greater than 5"),
        String::from("less than or equal to 5")
    );
    assert_eq!(String::from("greater than 5"), result);
}

// ----------------------------------------------------------------

#[test]
fn test_ternary_eq() {
    let eq = ternary_eq!(2, 2, 1, -1);
    let ne = ternary_eq!(1, 2, 1, -1);

    assert_eq!(1, eq);
    assert_eq!(-1, ne);
}

#[test]
fn test_ternary_eq_float() {
    let seed = 2.0;

    let eq = ternary_eq!(2.0, 2.0, 1.0, -1.0);
    let ne = ternary_eq!(2.0, 2.1, 1.0, -1.0);

    assert_eq!(1.0, eq);
    assert_eq!(-1.0, ne);
}

#[test]
fn test_ternary_eq_date_time() {
    let now = SystemTime::now();
    let now_add = now.add(Duration::from_millis(1_000));
    let now_sub = now.sub(Duration::from_millis(1_000));

    let eq = ternary_eq!(now, now, 1, -1);
    let ne = ternary_eq!(now, now.add(Duration::from_millis(1_000)), now_add, now_sub);

    assert_eq!(1, eq);
    assert_eq!(now_sub, ne);
}

// ----------------------------------------------------------------

#[test]
fn test_ternary_ne() {
    let ne = ternary_ne!(3, 2, 1, -1);
    let eq = ternary_ne!(2, 2, 1, -1);

    assert_eq!(1, ne);
    assert_eq!(-1, eq);
}

#[test]
fn test_ternary_ne_date_time() {
    let now = SystemTime::now();
    let now_add = now.add(Duration::from_millis(1_000));
    let now_sub = now.sub(Duration::from_millis(1_000));

    let ne = ternary_ne!(now, now.add(Duration::from_millis(1_000)), 1, -1);
    let eq = ternary_ne!(now, now, now_add, now_sub);

    assert_eq!(1, ne);
    assert_eq!(now_sub, eq);
}

// ---------------------------------------------------------------- repeat

#[test]
fn test_repeat() {
    let repeat_1 = repeat!("A", 5);
    assert_eq!(vec!["A", "A", "A", "A", "A"], repeat_1);

    let repeat_2 = repeat!(101, 5);
    assert_eq!(vec![101, 101, 101, 101, 101], repeat_2);
}

#[test]
fn test_repeat_str() {
    let repeat_str_1 = repeat_str!("A", 5);
    assert_eq!("AAAAA", repeat_str_1);

    let repeat_str_2 = repeat_str!("A", 5, ",");
    assert_eq!("A,A,A,A,A", repeat_str_2);

    let repeat_str_3 = repeat_str!(101, 5, ",");
    assert_eq!("101,101,101,101,101", repeat_str_3);
}
