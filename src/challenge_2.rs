pub fn challenge() {

    let mut valeurs: Vec<i32> = Vec::new();
    valeurs.push(1);
    valeurs.push(2);

    while valeurs[valeurs.len()-1] + valeurs[valeurs.len()-2] <= 4000000 {
        valeurs.push(valeurs[valeurs.len()-1] + valeurs[valeurs.len()-2]);
    }

    println!("{}",valeurs.len());

}