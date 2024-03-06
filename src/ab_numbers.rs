use rand::{thread_rng, Rng};

pub fn check_is_vaild(number : u32) -> bool {
    // check if the  number is 4 digit between 123~9876 and no repeat
    match number{
        num @ 1023..=9876 => {
            let str_num = num.to_string();
            for each in str_num.chars() {
                if str_num.matches(each).count() > 1 {
                    return false;
                }
            }
            return true;
        },
        _ => return false,
    };
}

pub fn compare_two_value(num1:u32, num2:u32) -> (u32,u32){
    let s_num1: String = num1.to_string();
    let s_num2 = num2.to_string();
    let mut a:u32 = 0;
    let mut b:u32 = 0; 
    for (i, each) in s_num1.chars().enumerate() {
        if  each == s_num2.chars().nth(i).unwrap() {
            a += 1
        }
    }

    for (i1, e1) in s_num1.chars().enumerate() {
        for (i2, e2) in s_num2.chars().enumerate() {
            if i1 != i2 && e1 == e2 {
                b += 1;
            }
        }
    }
    (a, b)
}

pub fn get_all_vaild_numbers() -> Vec<u32>{
    let mut vec_numbers: Vec<u32> = Vec::new();
    for i in 1023..=9876 {
        if check_is_vaild(i) {
            vec_numbers.push(i);
        }
    }
    vec_numbers
}

pub fn random_generate_vaild_number() -> u32{
    let mut answer:u32 = thread_rng().gen_range(1023..9876);
    while !check_is_vaild(answer){
        answer = thread_rng().gen_range(1023..9876);
    }
    answer
}

