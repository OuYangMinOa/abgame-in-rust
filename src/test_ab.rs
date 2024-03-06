use crate::ab_numbers;

fn test_check_is_vaild(){
    assert_eq!(true,ab_numbers::check_is_vaild(1234));
    assert_eq!(false,ab_numbers::check_is_vaild(1232));
    assert_eq!(false,ab_numbers::check_is_vaild(2556));
    assert_eq!(false,ab_numbers::check_is_vaild(9879));
    assert_eq!(false,ab_numbers::check_is_vaild(8548));

}


fn test_compare_two_value(){
    assert_eq!((4,0),ab_numbers::compare_two_value(1234,1234));
    assert_eq!((2,0),ab_numbers::compare_two_value(1234,1564));
    assert_eq!((0,4),ab_numbers::compare_two_value(1234,4321));
    assert_eq!((0,0),ab_numbers::compare_two_value(1234,5678));
    assert_eq!((2,2),ab_numbers::compare_two_value(1234,4231));
    assert_eq!((1,0),ab_numbers::compare_two_value(8174,8596));

}


pub fn _test_all(){
    test_check_is_vaild();
    test_compare_two_value();
    
    println!("[*] All test past !!")
}