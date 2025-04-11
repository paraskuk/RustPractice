


fn average(arr: &[i32]) -> f64 {
    //let arr = vec![1, 2, 3];
    //let sum = arr.iter().fold(0, |a, b| a + b);
    //let count = arr.iter().fold(0, |acc, _| acc + 1);
    for (item, index) in arr.iter().enumerate() {
        println!("{} : {}", item, index);
    }

    let count = arr.len() as i32;
    let sum = arr.iter().sum::<i32>();
    let average: f64 = sum as f64 / count as f64;

    println!("The sum is {}", sum);
    println!("The count is {}", count);
    println!("The average is {}", average);
    average
}

fn min_array(arr: &[i32]) -> i32 {
    let mut min = arr[0];
    for &item in arr.iter() {
        if item < min {
            min = item;
        }
    }
    min
}

fn max_array(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for &item in arr.iter() {
        if item > max {
            max = item;
        }
    }
    max
}



fn main() {
    //println!("Hello, world!");
    let arr = vec![1, 2, 3,4,5,6];
    average(&arr);
    //min_array(&arr);
    println!("The min is {}", min_array(&arr));
    println!("The max is {}", max_array(&arr));


}
