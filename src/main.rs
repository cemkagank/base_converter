use std::env;

fn main() {
    let m1  = env::args().nth(1).unwrap();
    let m2  = env::args().nth(2).unwrap();
    match m2.as_str() {
        "hex" => println!("hex"),
        "dec" => {  
                    to_x(&m1,2);
                    to_x(&m1, 8);
                    to_x(&m1,16);
                },
        "oct" => println!("oct"),
        "bin" => to_x(&m1,8),
        _ => println!("unvalid base")
    }

}

fn to_x(s : &String,x : usize) {
    let mut f : f64 = s.parse::<f64>().unwrap();
    let mut i : usize = f.floor() as usize;
    let mut reverse = String::from("");
    while i > 0 {
        let half = i/x;
        let reminder = i % x;
        match reminder {
            0..=9 => {
                let reminder = reminder.to_string();
                reverse = reverse + &reminder.to_owned();
                
            },
            10 => {
                reverse = reverse + "A"
            },
            11 => {
                reverse = reverse + "B"
            },
            12 => {
                reverse = reverse + "C"
            },
            13 => {
                reverse = reverse + "D"
            },
            14 => {
                reverse = reverse + "E"
            },
            15 => {
                reverse = reverse + "F"
            },
            _ => println!("unexpected eror {}",reminder)

        }
        
        i = half; 
    }
    let correct = reverse.chars().rev().collect::<String>();
    
    // Now the fraction part
    let mut fra = String::from("");
    let mut ctr = 8;
    while ctr > 0 {
        f = f - f.floor();
        if f == 0.0 {
            break;
        }
        f = f * (x as f64);
        match f.floor() as i64 {
           0..=9=> fra = fra + &f.floor().to_string().to_owned(),
           10 => fra = fra + "A",
           11 => fra = fra + "B",
           12 => fra = fra + "C",
           13 => fra = fra + "D",
           14 => fra = fra + "E",
           15 => fra = fra + "F",
           _ => println!("Unexpected error")
        }
        
        ctr = ctr - 1;
    }
    match x {
        2 => println!("Binary ==> {}.{}",correct,fra),
        8 => println!("Octal ==> {}.{}",correct,fra),
        16 => println!("Hexadicaml ==> {}.{}",correct,fra),
        _ => println!("unexpected problem")
    }
}
