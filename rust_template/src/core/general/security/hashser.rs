use pwhash::bcrypt;

/*
    Basic password hashing. A random salt should be added in a per password basis
    and be stored in a secured location, but this security concerns fall outside of the scope of
    this template and must be handled by the development team, evaluating the pros and cons of every
    approach.
*/
pub fn hash_password(password: &str) ->  String {
    let hash = bcrypt::hash(password).unwrap();
    hash
}