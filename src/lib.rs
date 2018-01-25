mod simple {
    #[derive(Eq, PartialEq, Debug)]
    struct Element(i32);

    #[derive(Default, Eq, PartialEq, Debug)]
    struct Collection {
        elements: Vec<Element>
    }

    impl Collection {
        fn add<I: IntoIterator<Item=Element>>(&mut self, elements: I) {
            self.elements.extend(elements.into_iter())
        }
    }

    impl IntoIterator for Element {
        type Item = Element;
        type IntoIter = ::std::iter::Once<Element>;

        fn into_iter(self) -> Self::IntoIter {
            ::std::iter::once(self)
        }
    }


    #[test]
    fn should_add_a_vec_of_elements() {
        let mut c = Collection::default();

        c.add(vec![Element(1), Element(2)]);

        assert_eq!(c, Collection { elements: vec![Element(1), Element(2)] })
    }

    #[test]
    fn should_add_a_element() {
        let mut c = Collection::default();

        c.add(Element(42));

        assert_eq!(c, Collection { elements: vec![Element(42)] })
    }
}

mod generic {
    use ::std::fmt::Debug;

    #[derive(Default, PartialEq, Debug)]
    struct Collection<E>
        where E: PartialEq + Debug
    {
        elements: Vec<E>
    }

    impl<E> Collection<E>
        where E: PartialEq + Debug
    {
        fn add<I: IntoIterator<Item=E>>(&mut self, elements: I) {
            self.elements.extend(elements.into_iter())
        }
    }

    impl IntoIterator for E
        where E: PartialEq + Debug
    {
        type Item = E;
        type IntoIter = ::std::iter::Once<E>;

        fn into_iter(self) -> Self::IntoIter {
            ::std::iter::once(self)
        }
    }

    #[derive(Default, Debug, PartialEq)]
    struct E(f32);

    #[test]
    fn should_add_a_vec_of_elements() {
        let mut c = Collection::default();

        c.add(vec![E(1.0), E(2.0)]);

        assert_eq!(c, Collection { elements: vec![E(1.0), E(2.0)] })
    }

    #[test]
    fn should_add_a_element() {
        let mut c = Collection::default();

        c.add(E(42.0));

        assert_eq!(c, Collection { elements: vec![E(42.0)] })
    }
}
