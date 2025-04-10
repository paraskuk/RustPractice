


fn average(arr: &[i32]) -> f64 {
    //let arr = vec![1, 2, 3];
    let sum = arr.iter().fold(0, |a, b| a + b);
    let count = arr.iter().fold(0, |acc, _| acc + 1);
    //let count = arr.len() as i32;
    //let sum = arr.iter().sum::<i32>();

    let average: f64 = sum as f64 / count as f64;

    println!("The sum is {}", sum);
    println!("The count is {}", count);
    println!("The average is {}", average);
    average
}

fn main() {
    println!("Hello, world!");
    let arr = vec![1, 2, 3];
    average(&arr);


}
