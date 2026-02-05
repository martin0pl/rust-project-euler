pub fn challenge () {
    let mut plus_grand = 0;

    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            let multi = i * j;

            if est_palyndrome(multi) {
                if multi > plus_grand {plus_grand = multi;}
            }
        }
    }

    println!("{}",plus_grand);
}

fn est_palyndrome (nombre: i32) -> bool {

    let mut est_palyndrome = true;

    let nombre = nombre.to_string();

    for i in 0..(nombre.len()/2) {
        if nombre.chars().nth(i) != nombre.chars().nth(nombre.len()-1-i) {
            est_palyndrome = false;
            break;
        }
    }

    est_palyndrome
}