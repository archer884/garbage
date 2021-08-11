use rand::RngCore;
use squirrel_rng::SquirrelRng;

macro_rules! time {
    ($ex:expr) => {{
        let mut time = stopwatch::Stopwatch::start_new();
        $ex;
        time.stop();
        time
    }};
}

fn main() {
    let mut rng = SquirrelRng::new();
    let mut buf = vec![0u8; 0x10_0000 * 10].into_boxed_slice();

    let elapsed = time! {
        for _ in 0..10 {
            rng.fill_bytes(&mut buf);
        }
    };

    println!("{}", elapsed);
}
