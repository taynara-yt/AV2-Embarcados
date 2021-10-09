fn main(){    
    let mut a = Vec::<bool>::new();
    a.push(false);
    a.push(false);
    a.push(true);
    a.push(true);
    
    let mut b = Vec::<bool>::new();
    b.push(false);
    b.push(true);
    b.push(false);
    b.push(true);
    
    let mut x = Vec::<bool>::new();
    let mut xn = Vec::<bool>::new();
    
    println!("A & B = X = !X = !X");
    
    for i in 0..4 {
              // A && B        (A and B)
        x.push(a[i] && b[i]); 
              // !(A && B)    not(A and B)
        xn.push(!(x[i]));     
        println!("{}   {}   {}   {}    {}", match a[i] {true => 1,false => 0}, 
                                            match b[i] {true => 1,false => 0}, 
                                            match x[i] {true => 1,false => 0}, 
                                            match xn[i] {true => 1,false => 0},
                                            match !xn[i] {true => 1,false => 0}); // !(!(A && B))    not(not(A and B))
    }
}