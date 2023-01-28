use std::io::{self};
use std::process::exit;
use goto::gpoint;

extern crate log; //for logging
extern crate goto; //for goto

use log::{info, warn}; //using info and warn logging for now


fn main() {
    env_logger::init(); //initialise logging

    info!("here we go...");

    gpoint!['begin:

    println!("TC Kimlik No Giriniz: "); //asking user to get tc_no input

    let tc_no = get_input().trim().parse::<u64>().unwrap(); //getting input as tc_no as u64

    info!("The input entered by User is {}.", tc_no); //if a number have been entered, the number is logged

        let digits: Vec<u32> = tc_no.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect(); //converting the input to a vector

        let sayim = digits.len(); //counting vector

        if sayim == 11 {

            info!("Sayı 11 haneli, devam ediliyor");//if the input has 11 numbers, it is logged and program continues

            let oddnumbers:u32 = digits[0..10].iter().step_by(2).sum(); //getting sum of odd digits
            let evennumbers:u32 = digits[1..7].iter().step_by(2).sum(); //getting sum of even digits
            let marveloustotal:u32 = digits[0..10].iter().sum();

            let tc_calc = oddnumbers * 7 - evennumbers; //calculating as for the Turkish ID Number Algorithm

            tc_check(tc_calc, digits, marveloustotal, tc_no);
        }

        else {
            warn!("Sayı 11 haneli değil, girilen rakam sayısı {}! İşlem başa alınıyor...", sayim);//if the input doesn't have 11 numbers, it is logged and program starts from 'begin point
            continue 'begin
        }
    ];


    fn tc_check(mut tc_calc: u32, digits: Vec<u32>, marveloustotal: u32, tc_no: u64) { //identifying integers as original to ease of understanding about the algorithm

        tc_calc += 10;

        info!("The new calculation is {}", tc_calc); //for debugging purposes

        if digits[9] == tc_calc % 10 { //as for Turkish ID Number Algorithm, 10th digit must be equal to mod of tc_calc

            info!("Marvelous total is: {}", marveloustotal); //for debugging purposes

            if digits[10] == marveloustotal % 10 { //as for Turkish ID Number Algorithm, 11th digit must be equal to mod of marveloustotal
                println!("{} geçerli bir TC Kimlik Numarası!", tc_no);
                exit(0);
            }

            else {
                println!("{} geçerli bir TC Kimlik Numarası Değil!", tc_no); //if the number entered is not a valid ID number, it is informed to user
                warn!("The number is rejected because of mismatch between {} and {}", digits[10], marveloustotal % 10); //for troubleshooting purposes
                drop(main);
                exit(1);
            }
        }

        else {
            println!("{} geçerli bir TC Kimlik Numarası Değil!", tc_no); //if the number entered is not a valid ID number, it is informed to user
            warn!("The number is rejected because of mismatch between {} and {}", digits[9], tc_calc / 10); //for troubleshooting purposes
            drop(main);
            exit(1);
        }
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}