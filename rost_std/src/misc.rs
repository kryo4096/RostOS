pub fn itoa(digits: &mut [u8], num: i32) -> &[u8] { 
    let mut id : u32 = 0; // "digit index"     
    let mut arr = [b'0'; 10];
    loop {
        let sig = 10i32.pow(id); // significant digit
        arr[id as usize] = b'0' + ((num / sig) % 10) as u8;
        if num / sig == 0 {
            break;
        }
        id += 1;
    }
                                    
    for i in 0..id {
        digits[i as usize] = arr[(id - (i + 1)) as usize];                                                                                                                    
    }
    
    if id > 0 {
        &digits[0..id as _]
    } else {
        b"0"
    }
}

pub fn atoi(s: &[u8]) -> i32 {
    let mut sum = 0;
    for (i, digit) in s.iter().rev().enumerate() {
        let digit = digit - b'0';

        sum += digit as i32 * 10i32.pow(i as u32);
    }
    sum
}
