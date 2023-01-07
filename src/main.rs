use std::{ //using std to get user input
    io::{
        self,
        Write,
    },
    process,
};

extern crate log; //for logging

use log::{info, warn}; //using info and warn logging for now
fn main() {
    
    env_logger::init(); //initialise logging

    info!("başlıyoruz...");

    println!("- TC No Giriniz: "); //asking user to get tc_noinput
     
    io::stdout().flush().unwrap();
 
    let mut input = String::new();
 
    io::stdin().read_line(&mut input).unwrap();
 
    let user_input = input.trim().parse::<i128>().unwrap_or_else(|_| {
        eprintln!("- Girdi bir sayı değil!"); //an exception where the input entered is not a number
        drop(input);
        process::exit(1);
    });
 
    info!(
        "- The input entered by User is {}.", //if a number have been entered, the number is logged
        user_input,
    );


    unsafe{
        let res=tr_id_check_2(user_input);
        if res == 11 {
            info!("Sayı 11 haneli, devam ediliyor")//if the input has 11 numbers, it is logged and program continiues
        }
        else {
            info!("Sayı 11 haneli değil, işlem başa alınıyor...");//if the input doesn't have 11 numbers, it is logged and program starts from begging
            warn!("Girilen rakam sayısı: {}!",res);//logging of numbers entered
        }
    }


 /* fn tr_id_check_1() -> io::Result<()> {
    let mut tc_no = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut tc_no);

    println!("input {} ", tc_no);

    let my_string = tc_no.to_string();  // `parse()` works with `&str` and `String`!
    let my_int = my_string.parse::<i32>().unwrap();

    Ok(())

}
*/
/*fn tr_id_check_1() {
let mut tc_no = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut tc_no);

    let trimmed = tc_no.trim();
    let no = tc_no;
    match trimmed.parse::<u32>() {
        Ok(i) => println!("your integer input: {}", i),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}
*/
unsafe fn tr_id_check_2(num:i128)->i128 { //counting entered numbers
    static mut COUNT:i128=0;
	if num > 0 {
		COUNT = COUNT + 1;
		tr_id_check_2(num / 10);
	}
	return COUNT;
}
}