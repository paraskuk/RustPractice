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
            //println!("end is {}" ,end);
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
    println!("Please enter a number");
    io::stdin().read_line(&mut buffer).unwrap();
    //let number = buffer.trim().parse::<i32>().unwrap();
    println!("You entered: {}", custom_trim(buffer.as_str()));

}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn new(width: i32, height: i32) -> Rectangle {
        Rectangle { width, height }
    }
    fn print(&self) {
        println!("width is {} and height is {}", self.width, self.height);
    }
    fn get_width(&self) -> i32 {
        self.width
    }
    fn get_height(&self) -> i32 {
        self.height
    }
}

// Function to add two boxes of type T that implement the Add trait
use std::ops::Add;

fn add_boxes<T>(a: Box<T>, b: Box<T>) -> Box<T>
where
    T: Add<Output = T>,
{
    Box::new(*a + *b)
}

//build a function that compares and prints two types ...use traits use display and partiaEQ and Copy
fn compare_and_print<T, U>(a: T, b: U)
where
    T: std::fmt::Display + std::cmp::PartialEq<U> + Copy,
    U: std::fmt::Display + Copy,
{
    if a == b {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is not equal to {}", a, b);
    }
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
    //command_line();
    let rect = Rectangle::new(10, 20);
    rect.print();
    //println!("The area is {}", rect.area());

    let area = rect.area();
    let width = rect.get_width();
    let height = rect.get_height();
    println!("The area of the rectangle is {} square pixels.", area);
    println!("The width of the rectangle is {} pixels.", width);
    println!("The height of the rectangle is {} pixels.", height);

    // Create two boxes of type i32
    let box1 = Box::new(5);
    let box2 = Box::new(10);
    // Add the two boxes together
    let result = add_boxes(box1, box2);
    // Print the result
    println!("The result of adding the two boxes is: {}", *result);


    // Compare and print two integers
    let a = 5;
    let b = 10;
    compare_and_print(a, b);
    // Compare and print two strings
    let str1 = "Hello";
    let str2 = "World";
    compare_and_print(str1, str2);


}