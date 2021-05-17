/*
    Given:

```
    variant!(
        ShapeDef: Clone {
            I32(i32),
            Vec(Vec<i32>)
        }
    );
```

    generates

```
    #[derive(Debug, Clone)]
    pub enum ShapeDef {
        I32(i32),
        Vec(Vec<i32>),
    }

    impl From<i32> for ShapeDef {
        fn from(value: i32) -> ShapeDef {
            ShapeDef::I32(value)
        }
    }

    impl From<Vec<i32>> for ShapeDef {
        fn from(value: Vec<i32>) -> ShapeDef {
            ShapeDef::Vec(value)
        }
    }

```

*/
#[macro_export]
macro_rules! define_variant {
    (
        $variant_name:ident
        $(: $( $der:ident ),*)?
        {
            $( $ctor:ident ($typ:ty) ),+
        }
    ) => {
        #[derive(
            Debug,
            $( $($der),* )?
        )]
        pub enum $variant_name {
            $( $ctor($typ) ),+
        }

        $(
            impl From<$typ> for $variant_name {
                fn from(value: $typ) -> $variant_name {
                    $variant_name::$ctor(value)
                }
            }
        )+
    }
}
