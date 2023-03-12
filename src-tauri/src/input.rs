
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl TryFrom<&str> for Key {
    type Error = ();

    fn try_from(key_str: &str) -> Result<Self, ()> {
        use Key::*;

        match key_str {
            "a" => Ok(A),
            "b" => Ok(B),
            "c" => Ok(C),
            "d" => Ok(D),
            "e" => Ok(E),
            "f" => Ok(F),
            "g" => Ok(G),
            "h" => Ok(H),
            "i" => Ok(I),
            "j" => Ok(J),
            "k" => Ok(K),
            "l" => Ok(L),
            "m" => Ok(M),
            "n" => Ok(N),
            "o" => Ok(O),
            "p" => Ok(P),
            "q" => Ok(Q),
            "r" => Ok(R),
            "s" => Ok(S),
            "t" => Ok(T),
            "u" => Ok(U),
            "v" => Ok(V),
            "w" => Ok(W),
            "x" => Ok(X),
            "y" => Ok(Y),
            "z" => Ok(Z),
            _ => Err(()),
        }
    }
}
