/// Properties that `Grow` implementations must satisfy
macro_rules! grow {
    ($arb_cvrdt:ident, $arb_cvrdt_and_addend:ident) => {
        proptest! {
            #[test]
            fn merge_associative(x in $arb_cvrdt(), y in $arb_cvrdt(), z in $arb_cvrdt()) {
                prop_assert_eq!(
                    Grow::payload(&Grow::merge(&x, &Grow::merge(&y, &z))),
                    Grow::payload(&Grow::merge(&Grow::merge(&x, &y), &z))
                );
            }
            #[test]
            fn merge_commutative(x in $arb_cvrdt(), y in $arb_cvrdt()) {
                prop_assert_eq!(
                    Grow::payload(&Grow::merge(&x, &y)),
                    Grow::payload(&Grow::merge(&y, &x))
                );
            }
            #[test]
            fn merge_idempotent(x in $arb_cvrdt(), y in $arb_cvrdt()) {
                prop_assert_eq!(
                    Grow::payload(&Grow::merge(&Grow::merge(&x, &y), &y)),
                    Grow::payload(&Grow::merge(&x, &y))
                );
            }
            #[test]
            fn add_monotonic((x, u) in $arb_cvrdt_and_addend()) {
                let mut y = x.clone();
                Grow::add(&mut y, u);
                prop_assert!(Grow::le(&x, &y));
            }
        }
    };
    ($arb_cvrdt2:ident, $arb_cvrdt3:ident, $arb_cvrdt_and_addend:ident) => {
        proptest! {
            #[test]
            fn merge_associative((x, y, z) in $arb_cvrdt3()) {
                prop_assert_eq!(
                    Grow::payload(&Grow::merge(&x, &Grow::merge(&y, &z))),
                    Grow::payload(&Grow::merge(&Grow::merge(&x, &y), &z))
                );
            }
            #[test]
            fn merge_commutative((x, y) in $arb_cvrdt2()) {
                prop_assert_eq!(
                    Grow::payload(&Grow::merge(&x, &y)),
                    Grow::payload(&Grow::merge(&y, &x))
                );
            }
            #[test]
            fn merge_idempotent((x, y) in $arb_cvrdt2()) {
                prop_assert_eq!(
                    Grow::payload(&Grow::merge(&Grow::merge(&x, &y), &y)),
                    Grow::payload(&Grow::merge(&x, &y))
                );
            }
            #[test]
            fn add_monotonic((x, u) in $arb_cvrdt_and_addend()) {
                let mut y = x.clone();
                Grow::add(&mut y, u);
                prop_assert!(Grow::le(&x, &y));
            }
        }
    };
}

pub(crate) use grow;

/// Properties that `Shrink` implementations must satisfy
macro_rules! shrink {
    ($arb_cvrdt_and_subtrahend:ident) => {
        proptest! {
            #[test]
            fn del_monotonic((x, u) in $arb_cvrdt_and_subtrahend()) {
                let mut y = x.clone();
                Shrink::del(&mut y, u);
                prop_assert!(Grow::le(&x, &y));
            }
        }
    };
}

pub(crate) use shrink;
