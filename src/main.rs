use std::io;

fn fibonacci(a:i32) -> Vec<u32>{
      let mut sequence: Vec<u32> = Vec::new();
    sequence.push(1);
    sequence.push(2);
    let mut index:usize = 1;

    for _ in 0..(a-1){
        sequence.push({&sequence[&index-1] + &sequence[index]});
        index += 1;
    }

    /* for a in &sequence{
        println!("{a}");
    }  */ 
    return sequence;
}


fn main() {
    //F(n) = F(n-1) + F(n-2)
    let mut input_text: String = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read the line");

    let input_number: i32 = match input_text.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("error parsing input number, input num is now 0"); 0},
    };

    let resulting_sequence = fibonacci(input_number);
    let result:u32 = *resulting_sequence.last().unwrap();
    println!("{result}");
}
