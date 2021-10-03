extern crate csv;

use ::lib::report_utilities::process_file_contents;
use ::lib::report_utilities::read_report_csv;
use ::lib::report_utilities::find_sum_candidates;

mod lib;


// TODO: Add support for n-length candidate vectors 
// TODO: Refactor default values into arguements 

fn main() {
    let default_file_name = "report.csv";
    let default_target: i32 = 2020;
    let report_contents = read_report_csv(&default_file_name).expect("No file found");
    let report_values = process_file_contents(&report_contents).expect("No values found in report contents");
    let candidate_values = find_sum_candidates(&report_values, default_target).expect("No cadidate values detected");   
    println!("{:?}", candidate_values);
    let mut candidate_iterator = candidate_values.into_iter();
    let (candidate_value1, candidate_value2) = candidate_iterator.next().unwrap();
    println!("{:?} - {:?}", candidate_value1, candidate_value2);
    let result_value = candidate_value1 * candidate_value2;
    println!("value 1: {:?}", result_value);
}