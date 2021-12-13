use rpncalc::rpn_calculator::RpnCalculator;

#[test]
fn test_ok() {
    let calc = RpnCalculator::new(false);
    assert_eq!(calc.eval("5").unwrap(), 5);
    assert_eq!(calc.eval("50").unwrap(), 50);
    assert_eq!(calc.eval("-50").unwrap(), -50);

    assert_eq!(calc.eval("2 3 +").unwrap(), 5);
    assert_eq!(calc.eval("2 3 *").unwrap(), 6);
    assert_eq!(calc.eval("2 3 -").unwrap(), -1);
    assert_eq!(calc.eval("2 3 /").unwrap(), 0);
    assert_eq!(calc.eval("2 3 %").unwrap(), 2);
}

#[test]
fn test_ng() {
    let calc = RpnCalculator::new(false);
    assert!(calc.eval("").is_err());
    assert!(calc.eval("1 1 1 +").is_err());
    assert!(calc.eval("+ 1 1").is_err());
    assert!(calc.eval("1 1 ^").is_err());
}