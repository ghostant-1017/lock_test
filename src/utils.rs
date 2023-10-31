use snarkvm::{prelude::{*, narwhal::{BatchCertificate, batch_header::test_helpers::sample_batch_header_for_round_with_previous_certificate_ids}}};
use snarkvm::prelude::Testnet3 as CurrentNetwork;
use indexmap::{IndexSet, IndexMap};

const PRIVATE_KEY_NUMS: usize = 10000;

/// Returns a sample batch certificate, sampled at random.
pub fn sample_batch_certificate(rng: &mut TestRng) -> BatchCertificate<CurrentNetwork> {
    sample_batch_certificate_for_round(rng.gen(), rng)
}

/// Returns a sample batch certificate with a given round; the rest is sampled at random.
pub fn sample_batch_certificate_for_round(round: u64, rng: &mut TestRng) -> BatchCertificate<CurrentNetwork> {
    // Sample certificate IDs.
    let certificate_ids = (0..10).map(|_| Field::<CurrentNetwork>::rand(rng)).collect::<IndexSet<_>>();
    // Return the batch certificate.
    sample_batch_certificate_for_round_with_previous_certificate_ids(round, certificate_ids, rng)
}

/// Returns a sample batch certificate with a given round; the rest is sampled at random.
pub fn sample_batch_certificate_for_round_with_previous_certificate_ids(
    round: u64,
    previous_certificate_ids: IndexSet<Field<CurrentNetwork>>,
    rng: &mut TestRng,
) -> BatchCertificate<CurrentNetwork> {
    // Sample a batch header.
    let batch_header =
        sample_batch_header_for_round_with_previous_certificate_ids(
            round,
            previous_certificate_ids,
            rng,
        );
    // Sample a list of signatures.
    let mut signatures = IndexMap::with_capacity(5);
    for _ in 0..PRIVATE_KEY_NUMS {
        let private_key = PrivateKey::new(rng).unwrap();
        let timestamp = time::OffsetDateTime::now_utc().unix_timestamp();
        let timestamp_field = Field::from_u64(timestamp as u64);
        signatures.insert(private_key.sign(&[batch_header.batch_id(), timestamp_field], rng).unwrap(), timestamp);
    }
    // Return the batch certificate.
    BatchCertificate::new(batch_header, signatures).unwrap()
}
