use rand::{thread_rng, Rng};

use crate::ab_numbers;

pub fn guess_player(){
    let mut _all_answer = ab_numbers::get_all_vaild_numbers();
    let mut rand_index ;
    let mut user_input ;
    let mut error_msg = String::new();
    let mut user_split;
    let mut user_a ;
    let mut user_b ;
    let mut thisa ;
    let mut thisb ;
    

    loop {
        rand_index = thread_rng().gen_range(0..(_all_answer.len()));
        let com_guess = _all_answer[rand_index];
        println!("{}",error_msg);
        println!("[*] Computer guess : {}", com_guess);
        println!("[*] Please input your A B:");
        user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();
        user_split = user_input.split_whitespace();
        let user_as = user_split.next().unwrap();
        let user_bs = user_split.next().unwrap();
        let user_ar = user_as.trim().parse();
        let user_br = user_bs.trim().parse();
        match user_ar {
            Ok(num1) => {user_a = num1;}
            Err(e) => {error_msg = format!("Error : Please input vaild number (A {user_as}) ({e})!"); continue }
        }
        match user_br {
            Ok(num2) => {user_b = num2;}
            Err(e) => {error_msg = format!("Error : Please input vaild number (B {user_bs}) ({e})!"); continue }
        }

        if user_a ==4{
            break;
        }
        if user_b >4 || user_a>4{
            error_msg = format!("Error : Please input vaild number (0~4)");
            continue;
        }

        println!("[*] (A,B) : {} {}", user_a, user_b);
        {
            let mut new_vec = vec![];
        
            for each in _all_answer.into_iter(){
                (thisa, thisb) = ab_numbers::compare_two_value(each, com_guess);
                if thisa == user_a && thisb == user_b{
                    // println!("{com_guess} {each} ({thisa},{thisb}) ({user_a},{user_b})");
                    new_vec.push(each);
                }
            }

            _all_answer = new_vec.clone();
            
        }
    }

    println!("[*] I win !!");
}



pub fn guess_computer(){
    let     answer = ab_numbers::random_generate_vaild_number();

    let mut user_input = String::new();
    let mut error_msg = String::new();
    let mut memory = String::new();

    let mut guess_time:u32 = 0;
    let mut user_num : u32 = 0;

    let mut thisa ;
    let mut thisb ;

    dbg!(answer);
    while user_num != answer{
        user_input.clear();
        println!("{}","\r\n".repeat(2));  // clear
        
        println!("{}", memory);
        println!("{}", error_msg);
        println!("[*] Guess my number (1023 ~ 9876 with no repeat)");

        // user input 
        let stdinput = std::io::stdin().read_line(&mut user_input);
        match stdinput {
            Ok(_) => {}
            Err(_) => { error_msg = "Error : Please input vaild number".to_string(); continue }
        }
        dbg!(&user_input);


        // parse userinput to u32
        match user_input.trim().parse::<u32>() {
            Ok(num) => { 
                if ab_numbers::check_is_vaild(num){
                    println!("{}",num);
                    user_num = num; 
                }else{
                    error_msg = "Error : Please input vaild number".to_string();
                    continue;
                }
            }
            Err(e) => { error_msg = format!("Error : Please input vaild number ({e})"); continue }
        }
        
        (thisa, thisb) = ab_numbers::compare_two_value(user_num, answer);


        guess_time += 1;
        memory.push_str(format!("{guess_time}. | {user_num} | A:{thisa} B:{thisb} |\n").as_str());
        error_msg = String::new();
    }
    println!("[*] Congratulation !!!! you got the right number");


}