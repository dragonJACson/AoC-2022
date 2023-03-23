fn main() {
    println!("All codes are in corresponding dir, run `cargo test` for testing.");
}


#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;
    use test_binary::*;

    #[test]
    fn day_one() -> Result<(), Box<dyn std::error::Error>> {
        let test_bin_path = build_test_binary("day-1", "").expect("error building test binary");
        let mut test_bin = Command::new(test_bin_path);

        test_bin.assert()
                .success()
                .stdout(predicate::str::diff("70374\n204610\n"));

        Ok(())
    }

    #[test]
    fn day_two() -> Result<(), Box<dyn std::error::Error>> {
        let test_bin_path = build_test_binary("day-2", "").expect("error building test binary");
        let mut test_bin = Command::new(test_bin_path);

        test_bin.assert()
                .success()
                .stdout(predicate::str::diff("14163\n12091\n"));

        Ok(())
    }

    #[test]
    fn day_three() -> Result<(), Box<dyn std::error::Error>> {
        let test_bin_path = build_test_binary("day-3", "").expect("error building test binary");
        let mut test_bin = Command::new(test_bin_path);

        test_bin.assert()
                .success()
                .stdout(predicate::str::diff("7917\n2585\n"));

        Ok(())
    }

    #[test]
    fn day_four() -> Result<(), Box<dyn std::error::Error>> {
        let test_bin_path = build_test_binary("day-4", "").expect("error building test binary");
        let mut test_bin = Command::new(test_bin_path);

        test_bin.assert()
                .success()
                .stdout(predicate::str::diff("576\n905\n"));

        Ok(())
    }

    #[test]
    fn day_five() -> Result<(), Box<dyn std::error::Error>> {
        let test_bin_path = build_test_binary("day-5", "").expect("error building test binary");
        let mut test_bin = Command::new(test_bin_path);

        test_bin.assert()
                .success()
                .stdout(predicate::str::diff("RFFFWBPNS\nCQQBBJFCS\n"));

        Ok(())
    }

    #[test]
    fn day_six() -> Result<(), Box<dyn std::error::Error>> {
        let test_bin_path = build_test_binary("day-6", "").expect("error building test binary");
        let mut test_bin = Command::new(test_bin_path);

        test_bin.assert()
                .success()
                .stdout(predicate::str::diff("1804\n2508\n"));

        Ok(())
    }

    #[test]
    fn day_seven() -> Result<(), Box<dyn std::error::Error>> {
        let test_bin_path = build_test_binary("day-7", "").expect("error building test binary");
        let mut test_bin = Command::new(test_bin_path);

        test_bin.assert()
                .success()
                .stdout(predicate::str::diff("1297159\n3866390\n"));

        Ok(())
    }

    #[test]
    fn day_eight() -> Result<(), Box<dyn std::error::Error>> {
        let test_bin_path = build_test_binary("day-8", "").expect("error building test binary");
        let mut test_bin = Command::new(test_bin_path);

        test_bin.assert()
                .success()
                .stdout(predicate::str::diff("1662\n537600\n"));

        Ok(())
    }

    #[test]
    fn day_nine() -> Result<(), Box<dyn std::error::Error>> {
        let test_bin_path = build_test_binary("day-9", "").expect("error building test binary");
        let mut test_bin = Command::new(test_bin_path);

        test_bin.assert()
                .success()
                .stdout(predicate::str::diff("5878\n2405\n"));

        Ok(())
    }

    #[test]
    fn day_ten() -> Result<(), Box<dyn std::error::Error>> {
        let test_bin_path = build_test_binary("day-10", "").expect("error building test binary");
        let mut test_bin = Command::new(test_bin_path);

        test_bin.assert()
                .success()
                .stdout(predicate::path::eq_file("./day-10/result.txt").utf8().unwrap());

        Ok(())
    }

    #[test]
    fn day_eleven() -> Result<(), Box<dyn std::error::Error>> {
        let test_bin_path = build_test_binary("day-11", "").expect("error building test binary");
        let mut test_bin = Command::new(test_bin_path);

        test_bin.assert()
                .success()
                .stdout(predicate::str::diff("121450\n28244037010\n"));

        Ok(())
    }
}
