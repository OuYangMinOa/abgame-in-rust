mod test_ab;
mod ab_numbers;//::check_is_vaild;
mod game_start;

fn main() {
    // testing
    test_ab::_test_all();

    // game_start::guess_computer();
    loop{
        let mut user_input = String::new();
        println!("[1] Player guess computer\n\r[2] Computer guess player");
        std::io::stdin().read_line(&mut user_input).unwrap();

        if user_input.trim() == "1" {
            game_start::guess_computer();
        }else if user_input.trim() == "2"{
            game_start::guess_player();
        }
    }
    // 

}
