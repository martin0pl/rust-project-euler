pub fn _crible_eratosthene (mut nb_max:usize) {

    // Calcul du plus grand diviseur possible pour limiter les calculs
    nb_max = ((nb_max as f32).sqrt() as i32) as usize;

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

}