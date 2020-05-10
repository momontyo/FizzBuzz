fn main() {

    for num in (1..100)  {
        let stri: String = num.to_string();

        let mut hasSeven = false;
        for c in stri.chars() {
            if c == '7' {
                hasSeven = true;
                break;
            }
        }

        if hasSeven {
            println!("Github");
        } else if num % 15 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", num);
        }
    }

}
