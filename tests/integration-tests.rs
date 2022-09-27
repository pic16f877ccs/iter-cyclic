#[cfg(test)]
mod range_skip {
    use iter_cyclic::range_skip;

    #[test]
    fn zero_zero_skip_zero() {
        assert_eq!(
            range_skip(u8::MIN..u8::MIN, usize::MIN).collect::<Vec<_>>(),
            [0]
        );
    }

    #[test]
    fn zero_one_skip_zero() {
        assert_eq!(
            range_skip(u8::MIN..1, usize::MIN).collect::<Vec<_>>(),
            [0, 1]
        );
    }

    #[test]
    fn zero_ten_skip_zero() {
        assert_eq!(
            range_skip(0..10, 0).collect::<Vec<_>>(),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
    }

    #[test]
    fn zero_maxu8_zero() {
        assert_eq!(
            range_skip(0..u8::MAX, 0).collect::<Vec<_>>(),
            (0..=255).collect::<Vec<u8>>()
        );
    }

    #[test]
    fn mini8_maxi8_zero() {
        assert_eq!(
            range_skip(i8::MIN..i8::MAX, 0).collect::<Vec<_>>(),
            (i8::MIN..=i8::MAX).collect::<Vec<_>>()
        );
    }

    #[test]
    fn mini8_zero_zero() {
        assert_eq!(
            range_skip(i8::MIN..0, 0).collect::<Vec<_>>(),
            (i8::MIN..=0).collect::<Vec<_>>()
        );
    }

    #[test]
    fn zero_maxi8_zero() {
        assert_eq!(
            range_skip(0..i8::MAX, 0).collect::<Vec<_>>(),
            (0..=i8::MAX).collect::<Vec<_>>()
        );
    }

    #[test]
    fn zero_zero_skip_one() {
        assert_eq!(
            range_skip(0..0, 1).take(10).collect::<Vec<u8>>(),
            [0, 2, 4, 6, 8, 10, 12, 14, 16, 18]
        );
    }

    #[test]
    fn zero_one_skip_one() {
        assert_eq!(
            range_skip(0..1, 1).take(10).collect::<Vec<u8>>(),
            [0, 1, 3, 4, 6, 7, 9, 10, 12, 13]
        );
    }

    #[test]
    fn zero_one_skip_twohundred() {
        assert_eq!(range_skip(0..1, 200).collect::<Vec<u8>>(), [0, 1, 202, 203]);
    }

    #[test]
    #[should_panic(expected = "expected addition to succeed")]
    fn output_value_overflow() {
        range_skip(0_u8..127, 1);
    }

    #[test]
    #[should_panic(expected = "expected addition to succeed")]
    fn zero_maxu8_one() {
        range_skip(0..u8::MAX, 1);
    }

    #[test]
    #[should_panic(expected = "expected subtraction to succeed")]
    fn zero_mini8_maxi8() {
        range_skip(i8::MIN..i8::MAX, 1);
    }

    #[test]
    #[should_panic(expected = "start 10 > end 0")]
    fn input_value_overflow() {
        range_skip(10..0, 0);
    }

    #[test]
    #[should_panic(expected = "expected conversion to succeed")]
    fn input_value_conversion() {
        range_skip(1_u8..10, 256).collect::<Vec<_>>();
    }
}

#[cfg(test)]
mod range_step {
    use iter_cyclic::range_step;
    #[test]
    fn range_step_test_i8_a() {
        assert_eq!(
            range_step(i8::MIN, i8::MAX, 256).collect::<Vec<_>>(),
            (i8::MIN..=i8::MAX).collect::<Vec<_>>()
        );
    }

    #[test]
    fn range_step_test_i8_b() {
        assert_eq!(
            range_step(i8::MIN, -124, 51).collect::<Vec<_>>(),
            [
                -128, -127, -126, -125, -124, -77, -76, -75, -74, -73, -26, -25, -24, -23, -22, 25,
                26, 27, 28, 29, 76, 77, 78, 79, 80
            ]
        );
    }

    #[test]
    fn range_step_test_i8_c() {
        assert_eq!(
            range_step(i8::MIN, i8::MIN, 51).collect::<Vec<_>>(),
            [-128, -77, -26, 25, 76]
        );
    }

    #[test]
    fn range_step_test_i8_d() {
        assert_eq!(
            range_step(i8::MIN, i8::MIN + 1, 85).collect::<Vec<_>>(),
            [-128, -127, -43, -42, 42, 43]
        );
    }

    #[test]
    fn range_step_test_i8_g() {
        assert_eq!(
            range_step(i8::MIN + 1, i8::MAX - 1, u8::MAX as usize).collect::<Vec<_>>(),
            (i8::MIN + 1..i8::MAX).collect::<Vec<_>>()
        );
    }

    #[test]
    fn range_step_test_i8_e() {
        assert_eq!(
            range_step(i8::MIN, i8::MIN, 86).collect::<Vec<_>>(),
            [-128, -42]
        );
    }

    #[test]
    fn range_step_test_u8_a() {
        assert_eq!(
            range_step(u8::MIN, u8::MAX, 256).collect::<Vec<_>>(),
            (u8::MIN..=u8::MAX).collect::<Vec<_>>()
        );
    }

    #[test]
    fn range_step_test_u8_b() {
        assert_eq!(
            range_step(u8::MIN, 50, 51).collect::<Vec<_>>(),
            (u8::MIN..u8::MAX).collect::<Vec<_>>()
        );
    }

    #[test]
    fn range_step_test_u8_c() {
        assert_eq!(
            range_step(1, u8::MAX - 1, u8::MAX as usize).collect::<Vec<_>>(),
            (1..u8::MAX).collect::<Vec<_>>()
        );
    }

}
