pub fn challenge() {
    let valeur_max: usize = 600851475143;
    let valeur_max_racine = (valeur_max as f64).sqrt() as usize;

    // Récupération des nombres premiers jusqu'à la racine
    let diviseurs_possibles = crible_eratosthene(valeur_max_racine);

    // On parcourt la liste en partant de la fin (du plus grand au plus petit)
    for &p in diviseurs_possibles.iter().rev() {
        if valeur_max % (p as usize) == 0 {
            println!("{}", p);
            break;
        }
    }
}

pub fn crible_eratosthene (nb_max:usize) -> Vec<i32> {

    // Réservation de la place
    let mut crible: Vec<bool> = Vec::with_capacity(nb_max);

    // Passage de toutes les valeurs à true
    for _ in 0..2 {crible.push(false);}
    for _ in 0..(nb_max-1) {crible.push(true);}

    // Application du crible
    for i in 0..crible.len() {

        // Si il est pas premier, on le saute
        if crible[i] == false {
            continue;
        }

        // On prend tous les indices divisibles par i
        let mut j = i+i;

        // On annule les indices divisibles par i
        while j < crible.len() {
            crible[j] = false;
            j += i;
        }
    }

    // Vecteur pour tous les nombres premiers
    let mut nombres_premiers: Vec<i32> = Vec::new();

    // On récupère tous les nombres premiers
    for i in 0..crible.len() {
        if crible[i] == true {
            nombres_premiers.push(i as i32);
        }
    }

    // On retourne la liste des nombres premiers
    nombres_premiers

}