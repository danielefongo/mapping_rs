mod simple_fields {
    #[derive(Debug, PartialEq, Eq)]
    pub struct InternalStruct {
        pub field1: bool,
        pub field2: i32,
    }

    #[mapping::map_struct_to(InternalStruct)]
    #[mapping::map_struct_from(InternalStruct)]
    #[derive(Debug, PartialEq, Eq)]
    pub struct ExternalStruct {
        pub field1: bool,
        pub field2: i32,
    }

    #[test]
    fn map_enum_to() {
        assert_eq!(
            InternalStruct::from(ExternalStruct {
                field1: true,
                field2: 42
            }),
            InternalStruct {
                field1: true,
                field2: 42
            }
        );
    }

    #[test]
    fn map_enum_from() {
        assert_eq!(
            ExternalStruct::from(InternalStruct {
                field1: true,
                field2: 42
            }),
            ExternalStruct {
                field1: true,
                field2: 42
            }
        );
    }
}

mod options {
    #[derive(Debug, PartialEq, Eq)]
    pub struct InternalStruct {
        pub field1: Option<InternalInnerStruct>,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct InternalInnerStruct {
        pub field1: bool,
    }

    #[mapping::map_struct_to(InternalStruct)]
    #[mapping::map_struct_from(InternalStruct)]
    #[derive(Debug, PartialEq, Eq)]
    pub struct ExternalStruct {
        pub field1: Option<ExternalInnerStruct>,
    }

    #[mapping::map_struct_to(InternalInnerStruct)]
    #[mapping::map_struct_from(InternalInnerStruct)]
    #[derive(Debug, PartialEq, Eq)]
    pub struct ExternalInnerStruct {
        pub field1: bool,
    }

    #[test]
    fn map_enum_to() {
        assert_eq!(
            InternalStruct::from(ExternalStruct {
                field1: Some(ExternalInnerStruct { field1: true }),
            }),
            InternalStruct {
                field1: Some(InternalInnerStruct { field1: true }),
            }
        );
    }

    #[test]
    fn map_enum_from() {
        assert_eq!(
            ExternalStruct::from(InternalStruct {
                field1: Some(InternalInnerStruct { field1: true }),
            }),
            ExternalStruct {
                field1: Some(ExternalInnerStruct { field1: true }),
            }
        );
    }
}

mod vectors {
    #[derive(Debug, PartialEq, Eq)]
    pub struct InternalStruct {
        pub field1: Vec<InternalInnerStruct>,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct InternalInnerStruct {
        pub field1: bool,
    }

    #[mapping::map_struct_to(InternalStruct)]
    #[mapping::map_struct_from(InternalStruct)]
    #[derive(Debug, PartialEq, Eq)]
    pub struct ExternalStruct {
        pub field1: Vec<ExternalInnerStruct>,
    }

    #[mapping::map_struct_to(InternalInnerStruct)]
    #[mapping::map_struct_from(InternalInnerStruct)]
    #[derive(Debug, PartialEq, Eq)]
    pub struct ExternalInnerStruct {
        pub field1: bool,
    }

    #[test]
    fn map_enum_to() {
        assert_eq!(
            InternalStruct::from(ExternalStruct {
                field1: vec![ExternalInnerStruct { field1: false }],
            }),
            InternalStruct {
                field1: vec![InternalInnerStruct { field1: false }],
            }
        );
    }

    #[test]
    fn map_enum_from() {
        assert_eq!(
            ExternalStruct::from(InternalStruct {
                field1: vec![InternalInnerStruct { field1: false }],
            }),
            ExternalStruct {
                field1: vec![ExternalInnerStruct { field1: false }],
            }
        );
    }
}
