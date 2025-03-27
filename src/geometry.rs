pub fn magnitude(coordenadas:&[f64; 3]) -> f64 {
    let mut result: f64 = 0.0;
    for element in coordenadas {
        result += element*element;
    }
    return result.sqrt();
}

pub fn normalize(coordenadas : & mut [f64; 3])  {
   let magnitud : f64 = magnitude(coordenadas);
   for i in 0..=2 {
       coordenadas[i] = coordenadas[i]/ magnitud;
   }
}
