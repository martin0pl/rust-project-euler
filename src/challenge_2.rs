pub fn challenge() {

    let mut valeurs: Vec<i32> = Vec::new();
    valeurs.push(1);
    valeurs.push(2);

    let mut somme_globale = 0;
    let mut somme ;

    while valeurs[valeurs.len()-1] + valeurs[valeurs.len()-2] <= 4000000 {
        somme = valeurs[valeurs.len()-1] + valeurs[valeurs.len()-2];
        valeurs.push(somme);
        if somme%2==0 {
            somme_globale += somme;
        }
    }
    println!("{}",somme_globale+2);

}