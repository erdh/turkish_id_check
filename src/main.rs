use std::io::{self};
use goto::gpoint;

extern crate log; //for logging
extern crate goto; //for goto

use log::{info, warn}; //using info and warn logging for now


fn main() {

    env_logger::init(); //initialise logging

    info!("başlıyoruz...");

    gpoint!['begin:

    println!("- TC No Giriniz: "); //asking user to get tc_no input

    let tc_no = get_input().trim().parse::<i64>().unwrap(); //getting input as tc_no with i64

    //let rakamlar: Vec<_> = tc_no.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect(); //dividing tc_no entered

    info!("- The input entered by User is {}.",tc_no); //if a number have been entered, the number is logged

        let digits = x(tc_no as usize); //converting the input to a vector

        let oddnumbers:usize = digits[0..10].iter().step_by(2).sum();

        let evennumbers:usize = digits[1..7].iter().step_by(2).sum();

        let muthistoplam:usize = digits[0..10].iter().sum();

        let sayim = digits.len(); //counting vector


        if sayim == 11 {
            info!("Sayı 11 haneli, devam ediliyor");//if the input has 11 numbers, it is logged and program continues
            println!("{:?}", digits); //for debugging purposes
            //let oddnumbers = partition_by_parity();
            println!("{:?}", digits[2]);
            //let oddtotal = digits;
            //println!("{}", oddtotal);
            //stupid_total(digits);
            println!("{}, {}", oddnumbers, evennumbers);
            let tc_hesap = oddnumbers * 7 - evennumbers;
            tc_check(tc_hesap, digits, muthistoplam);
            //divider(digits); //dividing digits and printing them
        }
        else {
            warn!("Sayı 11 haneli değil, girilen rakam sayısı {}! İşlem başa alınıyor...", sayim);//if the input doesn't have 11 numbers, it is logged and program starts from 'begin point
            continue 'begin
        }

        ];


}

fn tc_check(mut tc_hesap: usize, digits: Vec<usize>, muthistoplam: usize) {

        tc_hesap += 10;

        info!("the new hesap is {}", tc_hesap);

        if digits[9] == tc_hesap % 10 {

            info!("muthistoplam: {}", muthistoplam);

            if digits[10] == muthistoplam % 10 {
                println!("Becerdin mal!");
            }
        }
        else {
            warn!("Beceremedin mal! al sana hata! {}, {}", digits[9], tc_hesap / 10);
        }
    }


 /*pub fn partition_by_parity(nums: &mut [i64]) {
    let (even, odd): (Vec<_>, Vec<_>) = nums.iter().partition(|&x| x % 2 == 0);
    nums[..odd.len()].copy_from_slice(&odd);
    nums[odd.len()..].copy_from_slice(&even);
}
  */

fn x(n: usize) -> Vec<usize> {
    fn x_inner(n: usize, xs: &mut Vec<usize>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    xs
}

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}
/*fn sum() {
    let a = [1, 2, 3, 4, 5];
    let sum: u8 = a.iter().sum();
    println!("the total sum is: {}", sum);
}
 */