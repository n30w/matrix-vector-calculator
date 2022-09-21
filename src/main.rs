use std::{io, vec};

enum Calculation {
    UnitVector(Vec<f32>),
    Length(f32),
    DotProduct(f32),
    Angle(f32)
}

impl Calculation {

    fn unit_vec(v: &Vec<f32>) -> Vec<f32> {
        let len = Self::vec_len(v);
        let mut unit_vec: Vec<f32>;
        for i in 0..v.len() {
            unit_vec.push(v[i] / len);
        }
        unit_vec
    }

    fn vec_len(v: &Vec<f32>) -> f32 {
        Self::dot_prod(v, None).sqrt()
    }

    fn dot_prod(v1: &Vec<f32>, v2: Option<&Vec<f32>>) -> f32 {
        let mut dot_product: f32 = 0.0;
        let len = v1.len();

        // Optional number of arguments in function: https://stackoverflow.com/questions/28951503/how-can-i-create-a-function-with-a-variable-number-of-arguments

        match v2 {
            Some(v2) => { // Case where there is a second unique vector
                for i in 0..len {
                    dot_product += v1[i] * v2[i];
               }
               dot_product 
            }
            None => { // Case where there is only one unique vector
                for i in 0..len {

                    // How to num to power: https://stackoverflow.com/questions/51208703/how-to-raise-a-number-to-a-power
                    
                    dot_product += f32::powf(v1[i], 2.0);
                }
                dot_product
            }
        }
    }
    
    fn angle(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
        let angle: f32 = Self::dot_prod(v1, Some(v2)) / (Self::vec_len(v1) * Self::vec_len(v2));
        angle.acos()
    }
}

fn evaluate(c: Calculation){
    match c {
        Calculation::UnitVector(v) => println!("{:?}", Calculation::unit_vec(&v)),
        Calculation::Length(l) => println!("{}", Calculation::vec_len(&v)),
        Calculation::DotProduct(p) => println!("{}", Calculation::dot_prod(v1, v2)),
        Calculation::Angle(a) => println!("{}", Calculation::angle(v1, v2)),
    }
}

fn main() {
    let vec_count = 2;

    let mut input_string = String::new();
    let mut v1: Vec<f32> = Vec::new();
    let mut v2: Vec<f32> = Vec::new();
    

    println!("Please input size of vector: ");
    
    // Define vector size:
    input_string.clear();
    io::stdin()
        .read_line(&mut input_string)
        .unwrap();
    
    let v_size: i32 = input_string.trim().parse().unwrap(); 
    
    println!("Please input vector components: ");
    
    for i in 0..vec_count {
        println!("Components for vector{}: ", i);
        for j in 0..v_size {
            input_string.clear();
            print!("{}: ", j);
            io::stdin()
                .read_line(&mut input_string)
                .unwrap();
            
            let n: f32 = input_string.trim().parse().unwrap();
            if i != 0 { // Push to vector 1 or 2
                v2.push(n);
            } else {
                v1.push(n);
            }
            println!();
        }
    }

    println!("Select Calculation: ");
    println!("1. Unit Vector");
    println!("2. Length");
    println!("3. Dot Product");
    println!("4. Angle between x and y");
    print!("Select with number: ");

    let mut user_select = String::new();

    io::stdin()
        .read_line(&mut user_select)
        .unwrap();

    let n: i32 = user_select.trim().parse().unwrap();

    match n {
       1 => {
        Calculation::UnitVector(v1);
        
       }
       2 => ,
       3 => ,
       _ => println!("wrong")
    }
    



}

