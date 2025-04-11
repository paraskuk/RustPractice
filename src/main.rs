use std::io;

/// Calculates the average of the elements in the given array slice.
///
/// # Arguments
///
/// * `arr` - A slice of 32-bit integers for which the average is to be calculated.
///
/// # Returns
///
/// A `f64` value representing the average of the elements in the array slice.
///
/// # Panics
///
/// This function will panic if the slice is empty, as dividing by zero is not handled.
///
/// # Example
///
/// ```
/// let arr = vec![1, 2, 3, 4, 5];
/// let avg = average(&arr);
/// println!("The average is {}", avg);
/// ```
fn average(arr: &[i32]) -> f64 {
    // Iterate over the array and print each element with its index
    for (item, index) in arr.iter().enumerate() {
        println!("{} : {}", item, index);
    }

    // Get the count of elements in the array
    let count = arr.len() as i32;

    // Calculate the sum of the elements in the array
    let sum = arr.iter().sum::<i32>();

    // Calculate the average as a floating-point value
    let average: f64 = sum as f64 / count as f64;

    // Print the sum, count, and average
    println!("The sum is {}", sum);
    println!("The count is {}", count);
    println!("The average is {}", average);

    // Return the calculated average
    average
}

/// Finds the minimum value in the given array slice.
///
/// # Arguments
///
/// * `arr` - A slice of 32-bit integers from which the minimum value is to be found.
///
/// # Returns
///
/// A `i32` value representing the minimum value in the array slice.
///
/// # Panics
///
/// This function will panic if the slice is empty.
///
/// # Example
///
/// ```
/// let arr = vec![1, 2, 3, 4, 5];
/// let min = min_array(&arr);
/// println!("The minimum value is {}", min);
/// ```
fn min_array(arr: &[i32]) -> i32 {
    let mut min = arr[0];
    for &item in arr.iter() {
        if item < min {
            min = item;
        }
    }
    min
}

/// Finds the maximum value in the given array slice.
///
/// # Arguments
///
/// * `arr` - A slice of 32-bit integers from which the maximum value is to be found.
///
/// # Returns
///
/// A `i32` value representing the maximum value in the array slice.
///
/// # Panics
///
/// This function will panic if the slice is empty.
///
/// # Example
///
/// ```
/// let arr = vec![1, 2, 3, 4, 5];
/// let max = max_array(&arr);
/// println!("The maximum value is {}", max);
/// ```
fn max_array(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for &item in arr.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

/// Trims whitespace characters from both ends of a string.
///
/// # Arguments
///
/// * `s` - A string slice to be trimmed.
///
/// # Returns
///
/// A string slice with leading and trailing whitespace removed.
///
/// # Examples
///
/// ```
/// let s = "  Hello, world!  ";
/// let trimmed = custom_trim(s);
/// assert_eq!(trimmed, "Hello, world!");
/// ```
fn custom_trim(s: &str) -> &str {
    if s.is_empty() {
        return s;
    }

    // Find the first non-whitespace character
    let mut start = 0;
    for (i, c) in s.char_indices() {
        if !c.is_whitespace() {
            start = i;
            break;
        }

        // If we reach the end and all characters are whitespace
        if i == s.len() - 1 {
            return "";
        }
    }

    // Find the last non-whitespace character
    let mut end = s.len();
    for (i, c) in s.char_indices().rev() {
        if !c.is_whitespace() {
            end = i + c.len_utf8();
            println!("end is {}" ,end);
            break;
        }
    }

    // If end <= start, the string is all whitespace
    if end <= start {
        ""
    } else {
        &s[start..end]
    }
}

fn command_line() {
    let mut buffer = String::new();
    println!("Please enter a command");
    io::stdin().read_line(&mut buffer).unwrap();
    println!("You entered: {}", custom_trim(buffer.as_str()));

}

fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6];

    // Calculate and print the average of the array
    average(&arr);

    // Calculate and print the minimum value in the array
    println!("The min is {}", min_array(&arr));

    // Calculate and print the maximum value in the array
    println!("The max is {}", max_array(&arr));

    let s = "  Hello, world!  ";
    let trimmed = custom_trim(s);
    assert_eq!(trimmed, "Hello, world!");
    println!("trimmed is : {}", trimmed);
    command_line();
}