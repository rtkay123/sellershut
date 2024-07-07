use crate::common::id_gen::{generate_id, ID_LENGTH};

#[test]
fn check_id() {
    let id = generate_id();

    assert_eq!(id.len(), ID_LENGTH);
}
