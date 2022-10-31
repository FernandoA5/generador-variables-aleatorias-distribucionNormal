use rand_distr::{Normal, Distribution};
fn main() {
//https://rust-random.github.io/rand/rand_distr/struct.Normal.html
    let mut llegada: Vec<f64> = Vec::new();
    let mut entrada: Vec<f64> = Vec::new();
    let mut cobro: Vec<f64> = Vec::new();
    let mut enjuague: Vec<f64> = Vec::new();
    let mut lavado: Vec<f64> = Vec::new();
    let mut post_pro: Vec<f64> = Vec::new();
    
    for _i in 0..10{
        llegada.push(generador(8.0, 3.3));
        entrada.push(generador(1.75, 0.71));
        cobro.push(generador(2.13, 0.83));
        enjuague.push(generador(3.25, 0.71));
        lavado.push(generador(3.13, 0.99));
        post_pro.push(generador(8.38, 1.92));
    }

}
fn generador(media: f64, str_dev: f64)->f64{
    let normal = Normal::new(media, str_dev).unwrap();
    let var = normal.sample(&mut rand::thread_rng());
    var
}