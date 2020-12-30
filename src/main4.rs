fn main() {
    /*
     * string
     */
    // 1
    let s1 = "Hello1";
    println!("{}", s1);

    // 2
    let s2: String = String::from("Hello2");
    println!("{}", s2);

    /*
     * tuple
     */
    let mut t1 = (1, "2");
    println!("{}", t1.0);
    println!("{}", t1.1);
    t1.1 = "3";
    println!("{}", t1.1);

    /*
     * array
     */
    let a1 = [0, 1, 2];
    println!("{}", a1[0]);
    let mut a2: [i32; 3] = [3, 4, 5];
    a2[1] = 44;
    println!("{}", a2[1]);

    /*
     * Result
     */
    let result: Result<i32, String> = Ok(200);
    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("err: {}", err),
    }

    /*
     * Vector
     */
    let mut v1 = vec![1, 2, 3];
    v1.push(4);
    println!("{}", v1[3]);

    /*
     * if
     */
    let num = 1;
    if 0 < num {
        println!("0 < num");
    } else if num < 0 {
        println!("num < 0");
    } else {
        println!("0 == num");
    }

    /*
     * loop
     */
    let mut count = 0;
    loop {
        println!("count: {}", count);
        count += 1;
        if count == 10 {
            break;
        }
    }
}
