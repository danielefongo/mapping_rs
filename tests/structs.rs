mod named_fields {
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
