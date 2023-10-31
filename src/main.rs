mod utils;
use snarkvm::{prelude::{narwhal::BatchCertificate, Testnet3}, utilities::{TestRng, ToBytes, FromBytes}};
use utils::sample_batch_certificate;
fn main() {
    let mut rng = TestRng::default();
    let certificate: BatchCertificate<Testnet3> = sample_batch_certificate(&mut rng);
    // Print the size of certificate in MB
    let data_certificate = certificate.to_bytes_le().unwrap();
    println!("Size of certificate: {} MB", data_certificate.len() as f64 / 1024.0 / 1024.0);
    let start = time::Instant::now();
    let certificate = BatchCertificate::<Testnet3>::from_bytes_le(&data_certificate).unwrap();
    println!("Time to deserialize: {:?}", start.elapsed());
}

// Case1:
// Signatures_num: 100_000
// Size of certificate: 12.970909118652344 MB
// Time to deserialize: Duration { seconds: 100, nanoseconds: 605376522 }

// Case2: 
// signatures_num: 10_000
// Size of certificate: 1.2979354858398438 MB
// Time to deserialize: Duration { seconds: 10, nanoseconds: 37171050 }
