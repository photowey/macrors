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

// ----------------------------------------------------------------

#[test]
fn test_ternary_ne() {
    let ne = ternary_ne!(3, 2, 1, -1);
    let eq = ternary_ne!(2, 2, 1, -1);

    assert_eq!(1, ne);
    assert_eq!(-1, eq);
}
