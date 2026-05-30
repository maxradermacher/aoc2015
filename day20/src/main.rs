use std::fs;

fn main() {
    let input: usize = fs::read_to_string("20.txt").unwrap().trim_end().parse().unwrap();
    let mut deliveries = vec![0usize; input];
    for elf in 1..=(input/10) {
        for house_number in (elf..=(input/10)).step_by(elf) {
            deliveries[house_number - 1] += 10*elf;
        }
    }
    for (house_index, deliveries) in deliveries.iter().enumerate() {
        if *deliveries >= input {
            println!("{}", house_index + 1);
            break;
        }
    }
    // {
    //     let mut house_number = 1u64;
    //     loop {
    //         let mut deliveries = 0;
    //         for n in 1..=house_number {
    //             if n*n == house_number {
    //                 deliveries += n;
    //             }
    //             if n*n >= house_number {
    //                 break;
    //             }
    //             if house_number % n == 0 {
    //                 deliveries += n;
    //                 deliveries += house_number / n;
    //             }
    //         }
    //         if 10*deliveries >= input {
    //             println!("{}", house_number);
    //             break;
    //         }
    //         house_number += 1;
    //     }
    // }
    // {
    //     let mut house_number = 1u64;
    //     loop {
    //         let mut deliveries = 0;
    //         for n in 1..=house_number {
    //             if n*n == house_number {
    //                 deliveries += n;
    //             }
    //             if n*n >= house_number {
    //                 break;
    //             }
    //             if house_number % n == 0 {
    //                 if house_number <= 50*n {
    //                     deliveries += n;
    //                 }
    //                 if house_number <= house_number / n * 50 {
    //                     deliveries += house_number / n;
    //                 }
    //             }
    //         }
    //         if 11*deliveries >= input {
    //             println!("{}", house_number);
    //             break;
    //         }
    //         house_number += 1;
    //     }
    // }
}
