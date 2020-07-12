use num_traits::pow;


// Como el fractal de Mandelbrot de exponente 2 es una Funcion
// exponencial, si |Zn| > 2 entonces la funcion diverge
fn diverge(zn: f64) -> bool{
    return zn > 2.0;
}

// Funcion que recibe la cantidad de iteraciones y el valor C y
// aplica la serie de Mandelbrot para calcular si converge o diverge
fn mandelbrot(c: f64) {
    //let zi = 0;
    let mut mondelbrot_set = vec![];
    let mut zf: f64 = 0.0;
    let mut znf: f64 = 0.0;
    //let exp: usize = 2;

    for _ in 1..10 {
        mondelbrot_set.push(zf);
        znf = (( zf * zf ) as f64 + c) as f64;

        if diverge(znf){
            break;
        }
        zf = znf;
    }

    println!("{:?}", mondelbrot_set);
}

fn main(){
    let c: f64 = 5.0;

    mandelbrot(c);
}
