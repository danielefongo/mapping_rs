mod unit_variants {
    #[derive(Debug, PartialEq, Eq)]
    pub enum InternalEnum {
        A,
        B,
    }

    #[mapping::map_enum_to(InternalEnum)]
    #[mapping::map_enum_from(InternalEnum)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum ExternalEnum {
        A,
        B,
    }

    #[test]
    fn map_enum_from() {
        assert_eq!(InternalEnum::from(ExternalEnum::A), InternalEnum::A);
        assert_eq!(InternalEnum::from(ExternalEnum::B), InternalEnum::B);
    }

    #[test]
    fn map_enum_to() {
        assert_eq!(ExternalEnum::from(InternalEnum::A), ExternalEnum::A);
        assert_eq!(ExternalEnum::from(InternalEnum::B), ExternalEnum::B);
    }
}

mod unnamed_variants {
    #[derive(Debug, PartialEq, Eq)]
    pub enum InternalEnum {
        A(String),
        B(String),
    }

    #[mapping::map_enum_to(InternalEnum)]
    #[mapping::map_enum_from(InternalEnum)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum ExternalEnum {
        A(String),
        B(String),
    }

    #[test]
    fn map_enum_to() {
        assert_eq!(
            InternalEnum::from(ExternalEnum::A("A".to_owned())),
            InternalEnum::A("A".to_owned())
        );
        assert_eq!(
            InternalEnum::from(ExternalEnum::B("B".to_owned())),
            InternalEnum::B("B".to_owned())
        );
    }

    #[test]
    fn map_enum_from() {
        assert_eq!(
            ExternalEnum::from(InternalEnum::A("A".to_owned())),
            ExternalEnum::A("A".to_owned())
        );
        assert_eq!(
            ExternalEnum::from(InternalEnum::B("B".to_owned())),
            ExternalEnum::B("B".to_owned())
        );
    }
}

mod options {
    #[derive(Debug, PartialEq, Eq)]
    pub struct InternalStringWrapper(String);
    impl From<ExternalStringWrapper> for InternalStringWrapper {
        fn from(value: ExternalStringWrapper) -> Self {
            Self(value.0)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum InternalEnum {
        A(Option<InternalStringWrapper>),
        B(String),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct ExternalStringWrapper(String);
    impl From<InternalStringWrapper> for ExternalStringWrapper {
        fn from(value: InternalStringWrapper) -> Self {
            Self(value.0)
        }
    }

    #[mapping::map_enum_to(InternalEnum)]
    #[mapping::map_enum_from(InternalEnum)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum ExternalEnum {
        A(Option<ExternalStringWrapper>),
        B(String),
    }

    #[test]
    fn map_enum_to() {
        assert_eq!(
            InternalEnum::from(ExternalEnum::A(Some(ExternalStringWrapper("A".to_owned())))),
            InternalEnum::A(Some(InternalStringWrapper("A".to_owned())))
        );
        assert_eq!(
            InternalEnum::from(ExternalEnum::B("B".to_owned())),
            InternalEnum::B("B".to_owned())
        );
    }

    #[test]
    fn map_enum_from() {
        assert_eq!(
            ExternalEnum::from(InternalEnum::A(Some(InternalStringWrapper("A".to_owned())))),
            ExternalEnum::A(Some(ExternalStringWrapper("A".to_owned())))
        );
        assert_eq!(
            ExternalEnum::from(InternalEnum::B("B".to_owned())),
            ExternalEnum::B("B".to_owned())
        );
    }
}

mod vectors {
    #[derive(Debug, PartialEq, Eq)]
    pub struct InternalStringWrapper(String);
    impl From<ExternalStringWrapper> for InternalStringWrapper {
        fn from(value: ExternalStringWrapper) -> Self {
            Self(value.0)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum InternalEnum {
        A(Vec<InternalStringWrapper>),
        B(String),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct ExternalStringWrapper(String);
    impl From<InternalStringWrapper> for ExternalStringWrapper {
        fn from(value: InternalStringWrapper) -> Self {
            Self(value.0)
        }
    }

    #[mapping::map_enum_to(InternalEnum)]
    #[mapping::map_enum_from(InternalEnum)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum ExternalEnum {
        A(Vec<ExternalStringWrapper>),
        B(String),
    }

    #[test]
    fn map_enum_to() {
        assert_eq!(
            InternalEnum::from(ExternalEnum::A(vec![ExternalStringWrapper("A".to_owned())])),
            InternalEnum::A(vec![InternalStringWrapper("A".to_owned())])
        );
        assert_eq!(
            InternalEnum::from(ExternalEnum::B("B".to_owned())),
            InternalEnum::B("B".to_owned())
        );
    }

    #[test]
    fn map_enum_from() {
        assert_eq!(
            ExternalEnum::from(InternalEnum::A(vec![InternalStringWrapper("A".to_owned())])),
            ExternalEnum::A(vec![ExternalStringWrapper("A".to_owned())])
        );
        assert_eq!(
            ExternalEnum::from(InternalEnum::B("B".to_owned())),
            ExternalEnum::B("B".to_owned())
        );
    }
}
