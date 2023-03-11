mod calculator;
use calculator::calculator::calculator::{sum, subtraction, multiplication, division};
use std::io;

fn main() {
    println!("Informe qual operacao deseja");
    println!("1 - Sum");
    println!("2 - Subtraction");
    println!("3 - Multiplication");
    println!("4 - Division");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Falha ao ler entrada");

    if option.trim() == "1" {
        let mut x = String::new();
        let mut y = String::new();

        println!("Informe o primeiro valor");
        io::stdin().read_line(&mut x).expect("Falha ao informar valor");

        println!("Informe o segundo valor");
        io::stdin().read_line(&mut y).expect("Falha ao informar valor");

        let x: i32 = x.trim().parse().unwrap();
        let y: i32 = y.trim().parse().unwrap();

        let sum = sum(x, y);
        println!("{}", sum);
        
    } else if option.trim() == "2" {
        let mut x = String::new();
        let mut y = String::new();

        println!("Informe o primeiro valor");
        io::stdin().read_line(&mut x).expect("Falha ao informar valor");

        println!("Informe o segundo valor");
        io::stdin().read_line(&mut y).expect("Falha ao informar valor");

        let x: i32 = x.trim().parse().unwrap();
        let y: i32 = y.trim().parse().unwrap();

        let sub = subtraction(x, y);
        println!("{}", sub);

    } else if option.trim() == "3"{
        let mut x = String::new();
        let mut y = String::new();

        println!("Informe o primeiro valor");
        io::stdin().read_line(&mut x).expect("Falha ao informar valor");

        println!("Informe o segundo valor");
        io::stdin().read_line(&mut y).expect("Falha ao informar valor");

        let x: i32 = x.trim().parse().unwrap();
        let y: i32 = y.trim().parse().unwrap();

        let mult = multiplication(x, y);
        println!("{}", mult);

    } else {
        let mut x = String::new();
        let mut y = String::new();

        println!("Informe o primeiro valor");
        io::stdin().read_line(&mut x).expect("Falha ao informar valor");

        println!("Informe o segundo valor");
        io::stdin().read_line(&mut y).expect("Falha ao informar valor");

        let x: i32 = x.trim().parse().unwrap();
        let y: i32 = y.trim().parse().unwrap();

        let div = division(x, y);
        println!("{}", div)
    }

}
