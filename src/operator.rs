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

/// [`ternary`]
///
/// A macro for ternary conditional operation([wiki](https://en.wikipedia.org/wiki/Ternary_conditional_operator)) in rust.
///
/// This macro evaluates a condition and returns one of two expressions based on whether the condition
/// is true or false.
///
/// # Examples:
///
///```rust
/// use macrors::ternary;
///
/// let x = 5;
/// let y = 10;
///
/// let result = ternary!(x < y, "x is less than y", "x is not less than y");
/// assert_eq!(result, "x is less than y");
/// ```
/// ```
#[macro_export]
macro_rules! ternary {
    ($condition:expr, $value_true:expr, $value_false:expr) => {
        if $condition {
            $value_true
        } else {
            $value_false
        }
    };
}

/// [`ternary_eq`]
///
/// A macro for ternary conditional operation with equality comparison.
///
/// This macro compares two values and returns one of two expressions based on whether the values
/// are equal or not.
///
/// # Examples
///
/// ```rust
/// use macrors::ternary_eq;
///
/// let x = 5;
/// let y = 10;
///
/// let result_eq = ternary_eq!(x, y, "x is equal to y", "x is not equal to y");
/// assert_eq!(result_eq, "x is not equal to y");
/// ```
#[macro_export]
macro_rules! ternary_eq {
    ($left:expr, $right:expr, $true_expr:expr, $false_expr:expr) => {
        if $left == $right {
            $true_expr
        } else {
            $false_expr
        }
    };
}

/// [`ternary_ne`]
///
/// A macro for ternary conditional operation with equality comparison.
///
/// This macro compares two values and returns one of two expressions based on whether the values
/// are equal or not.
///
/// # Examples
///
/// ```
/// use macrors::ternary_eq;
///
/// let x = 5;
/// let y = 10;
///
/// let result_eq = ternary_eq!(x, y, "x is equal to y", "x is not equal to y");
/// assert_eq!(result_eq, "x is not equal to y");
/// ```
#[macro_export]
macro_rules! ternary_ne {
    ($left:expr, $right:expr, $true_expr:expr, $false_expr:expr) => {
        if $left != $right {
            $true_expr
        } else {
            $false_expr
        }
    };
}
