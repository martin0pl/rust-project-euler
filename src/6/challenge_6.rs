pub fn challenge() {
    const MAX: i32 = 100;

    // Somme des carrés
    let mut somme_carre: i32 = 0;
    for i in 1..=MAX {
        somme_carre += i*i;
    }

    // Carré de la somme
    let mut carre_somme: i32 = 0;
    for i in 1..=MAX {
        carre_somme += i;
    }
    carre_somme = carre_somme * carre_somme;

    println!("{}",carre_somme - somme_carre);
}