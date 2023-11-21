use std::io;

fn main() {
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    loop {  
    println!(" РЕШИТЬ КВАДРАТНОЕ УРОВНЕНИЕ");

    println!(" ВВЕДИТЕ a ");
    match io::stdin().read_line(&mut a_str) {
        Ok(_) => {},
        Err(e) => println!(" ошибка ввода - {}", e)
    } 
    println!(" ВВЕДИТЕ b ");
    match io::stdin().read_line(&mut b_str) {
        Ok(_) => {},
        Err(e) => println!(" ошибка ввода - {}", e)
    }
    println!(" ВВЕДИТЕ c ");
    match io::stdin().read_line(&mut c_str) {
        Ok(_) => {},
        Err(e) => println!(" ошибка ввода - {}", e)
    }
    
    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();
    
    let d: f64 = (b*b) - 4.0 * (a * c);
    
    if d > 0.0 {
        let x1: f64 = ((-b) + d.sqrt()) / (2.0 * a);
        let x2: f64 = ((-b) - d.sqrt()) / (2.0 * a);

        println!(" Решено\n Есть 2 корня\n D ={}\n Корень 1 = {}\n кКорень 2 = {}", d, x1, x2);
}
    if d == 0.0 {
        let x: f64  = (-b) / (2.0 * a);

        println!(" Решено\n Есть 1 корень\n D =0\n Корень  = {}", x);
    }
    if d < 0.0 {
        println!(" Нет корней!\n D<0\n D = {}", d);
    }
}
}