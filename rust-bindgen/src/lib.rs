use node_bindgen::derive::node_bindgen;

/// add two integer
#[node_bindgen]
fn sum(first: i32, second: i32) -> i32 {
    first + second
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
