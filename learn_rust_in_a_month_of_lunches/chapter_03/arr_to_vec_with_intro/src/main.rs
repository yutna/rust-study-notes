// NOTE: Use Vec<_> for type inference

fn main() {
    let _my_vec: Vec<u8> = [1, 2, 3].into();
    let _my_vec2: Vec<_> = [9, 0, 10].into();
    let _my_vec3 = Vec::from([0, 1, 2, 4]);
}
