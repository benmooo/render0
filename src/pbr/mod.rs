#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let a = ["😤", "😡", "😠", "😣", "😢", "😡", "😤", "😠", "😣", "😢"]
            .into_iter()
            .for_each(|e| {
                println!("{}", e);
            });

        assert_eq!(2 + 2, 4);
    }
}
