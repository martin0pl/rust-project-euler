pub fn challenge() {
    let mut est_le_bon = false;

    let mut i = 20;

    loop {
        if est_le_bon {break;}

        i += 2;

        for j in 2..=20 {
            est_le_bon = i%j == 0;
            if !est_le_bon {break;}
        }
    }

    println!("{}",i);
}