extern crate csv;

use std::io::{File};

fn main() {
    // Open CSV file with statistics
    let file = File::open(&Path::new("test.csv"));
    let mut rdr = csv::Decoder::from_reader(file);
    rdr.separator(';');
    rdr.has_headers(true);
    rdr.enforce_same_length(false);
    // Ignore comment lines generated by benchmark suite
    for i in range(0u,3) {
      let ignored = rdr.iter().next();
      println!("{} length", ignored.unwrap().len());
    }

    // Vectors for storage of data items
    let mut data : Vec<Vec<int>> = Vec::with_capacity(3);
    data.push(Vec::with_capacity(16384)); // event latency
    data.push(Vec::with_capacity(16384)); // dispatch latency + event latency
    data.push(Vec::with_capacity(16384)); // execution time

    for (event_latency, dispatch_latency, exec_time) in rdr.decode_iter::<(int, int, int)>() {
        // println!("{}, {}, {}", event_latency, dispatch_latency, exec_time);
        data.get_mut(0).push(event_latency);
        data.get_mut(1).push(dispatch_latency);
        data.get_mut(2).push(exec_time);
    }

    // Sort vectors
    data.get_mut(0).sort_by(|a, b| compare_ints(a, b)); // |a, b| a.cmp(b)
    data.get_mut(1).sort_by(|a, b| compare_ints(a, b));
    data.get_mut(2).sort_by(|a, b| compare_ints(a, b));

    for item in data[0].iter() {
        println!("{}", item);
    }
    println!("First: {}, Last: {}", get_first(&data[0]), get_last(&data[0]));
    println!("Avg: {}", get_avg(&data[0]));

    let (x, y) = get_quantiles(&data[0]);
    println!("Quantiles: {} {}", x, y);

}

fn get_first(input : &Vec<int>) -> int {
    input[0]
}

fn get_last(input : &Vec<int>) -> int {
    input[input.len() - 1]
}

fn get_avg(input : &Vec<int>) -> f32 {
    let mut sum = 0f32;

    for item in input.iter() {
        sum += *item as f32;
    }
    // Result
    return sum / (input.len() as f32);
}

/// Calculate 5% and 95% quantiles
fn get_quantiles(input : &Vec<int>) -> (int, int) {
    let low_index = ((input.len()-1) * 5u) / 100u;
    let high_index = ((input.len()-1) * 95u) / 100u;
    // Result
    (input[low_index], input[high_index])
}

fn compare_ints(left : &int, right : &int) -> Ordering {
    if left < right {
      Less
    } else if left > right {
      Greater
    } else {
      Equal
    }
}
