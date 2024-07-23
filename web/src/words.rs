use rand::seq::IteratorRandom;
pub fn words(amount: usize) -> Vec<String> {
    Vec::from([
        "casa", "libro", "gato", "escuela", "amigo", "playa", "mesa", "coche", "flor", "sol",
    ])
    .iter()
    .choose_multiple(&mut rand::thread_rng(), amount)
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}
