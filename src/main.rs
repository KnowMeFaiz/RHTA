fn cek_voltase(volt: i32) -> bool {
    volt > 200
}

fn main() {
    let kapasitas: usize = 4;
    let mut head: usize = 0;
    let mut buffer = [0, 0, 0, 0];

    let stream = [400, 800, 1200, 1600];

    for i in 0..stream.len() {
        let v = stream[i];
        if cek_voltase(v) {
            buffer[head] = v;
            head = (head + 1) % kapasitas;
        }
    }
}
println!("Head akhir: {}, Buffer slot 0: {}", head, buffer[0]);