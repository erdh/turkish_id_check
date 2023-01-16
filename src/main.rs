use std::{ //using std to get user input
    io::{
        self,
        Write,
    },
};
use goto::gpoint;

extern crate log; //for logging
extern crate goto; //for goto

use log::{info, warn}; //using info and warn logging for now
fn main() {
    env_logger::init(); //initialise logging

    info!("başlıyoruz...");

    gpoint!['begin:

    println!("- TC No Giriniz: "); //asking user to get tc_no input

    io::stdout().flush().unwrap();

    let tc_no = get_input().trim().parse::<i64>().unwrap();
    let digits: Vec<_> = tc_no.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let sayi:i64 = tc_no;
    info!("- The input entered by User is {}.",sayi); //if a number have been entered, the number is logged

        let sayim = digits.iter().count();
        if sayim == 11 {
            info!("Sayı 11 haneli, devam ediliyor");//if the input has 11 numbers, it is logged and program continues
            let digits = x(sayi as usize);
                println!("{:?}", digits);
        }
        else {
            warn!("Sayı 11 haneli değil, girilen rakam sayısı {}! İşlem başa alınıyor...",sayim);//if the input doesn't have 11 numbers, it is logged and program starts from begging
            continue 'begin
        }

        ];

}
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