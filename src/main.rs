use  std::io;
use rand::Rng;
use std::cmp::Ordering;



fn main(){
    //Jeu plus ou moins
    println!("Devinez le nombre!");
    let nombre_secret = rand::rng()
    			.random_range(1..101);
    
    //println!("Le nombre secret est: {}", nombre_secret);
    
    loop {
    
	    println!("Veuillez entrer un nombre: ");
    
	    //Le jeu va tirer au sort un nombre entre 1 et 100
	    let mut supposition  = String::new(); 
	    //User doit saisir un nombre
	    io::stdin()
		.read_line(&mut supposition)
		.expect("Echec de la lecture de l'entrée utilisateur");
		
	    let supposition: u32 = match supposition.trim().parse() {
	    	Ok(nombre) => nombre,
	    	Err(_) => continue,
	    };

	    println!("Votre nombre : {}", supposition);
	    
	    //Afficher si le nombre saisi par l'utilisateur est petit ou grand
	    match supposition.cmp(&nombre_secret) {
	    	Ordering::Less => println!("Le nombre secret est plus que: {}!", supposition),
	    	Ordering::Greater => println!("Le nombre secret est moins que: {}!", supposition),
	    	Ordering::Equal => {
	    		println!("Bravo! Vous avez gagné !");
	    		break;
	    	}	
    }
    
    
    }
    //Si le nombre est bon, le jeu afficheraun message de felicitation
    //Le jeu se termine
}
