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

/// A macro for generates a vector containing the repetition of the given expression `item` for `times` times.
///
/// This macro utilizes a loop to repeatedly push the evaluated `item` into a vector,
/// effectively creating a collection with `times` instances of `item`.
///
/// # Arguments
///
/// - `item`: The expression to be repeated.
/// - `times`: The number of times `$item` should be repeated.
///
/// # Returns
///
/// A vector containing the repeated expressions.
///
/// # Example
///
/// ```rust
/// use macrors::repeat;
///
/// let repeat_1 = repeat!("A", 5);
///  assert_eq!(vec!["A", "A", "A", "A", "A"], repeat_1);
///
/// let repeat_2 = repeat!(101, 5);
/// assert_eq!(vec![101, 101, 101, 101, 101], repeat_2);
/// ```
///
/// @since 0.2.0
///
#[macro_export]
macro_rules! repeat {
    ($item:expr, $times:expr) => {{
        let mut v = Vec::new();
        for _ in 0..$times {
            v.push($item);
        }

        v
    }};
}

// ----------------------------------------------------------------

/// A macro for concatenates a string expression `item` repeated `times` with an optional separator `separator`.
///
/// The macro provides two forms of invocation. The first form is a shorthand that defaults the separator to an empty string.
///
/// # Arguments
///
/// * `item`: The string expression to be repeated.
/// * `times`: The number of times to repeat the expression.
/// * `separator`: An optional separator to insert between repetitions. Defaults to an empty string if not provided.
///
/// # Returns
///
/// A `String` consisting of `item` repeated `times` times, with `separator` inserted between each repetition if provided.
///
/// # Example
///
/// ```rust
/// use macrors::repeat_str;
///
/// let repeat_str_1 = repeat_str!("A", 5);
/// assert_eq!("AAAAA", repeat_str_1);
///
/// let repeat_str_2 = repeat_str!("A", 5, ",");
/// assert_eq!("A,A,A,A,A", repeat_str_2);
///
/// let repeat_str_3 = repeat_str!(101, 5, ",");
/// assert_eq!("101,101,101,101,101", repeat_str_3);
/// ```
///
/// @since 0.2.0
///
#[macro_export]
macro_rules! repeat_str {
    ($item:expr, $times:expr) => {
        repeat_str!($item, $times, "")
    };

    ($item:expr, $times:expr, $separator:expr) => {{
        let mut rvt = String::new();
        for i in 0..$times {
            if i > 0 {
                rvt.push_str($separator);
            }
            rvt.push_str(&format!("{}", $item));
        }

        rvt
    }};
}
