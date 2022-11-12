macro_rules! with_unsized_new{
    ($(#[$($m:tt)*])* struct $name:ident { $($f:ident: $t:ty ,)* ~ $sl_f:ident: [$sl_t:ty] $(,)?}) => {
        // the struct itself
        $(#[$($m)*])*
        #[repr(C)]
        pub struct $name{
            $($f: $t,)*
            $sl_f: [$sl_t]
        }

        // version with the slice replaced with a type parameter
        ::paste::paste!{
            #[repr(C)]
            struct [< $name _generic >] <Gen: ?Sized> {
                $($f: $t,)*
                $sl_f: Gen
            }

            impl $name {
                unsafe fn create_unchecked<const S: usize>($($f: $t,)* $sl_f: [$sl_t; S]) -> Box<$name>{
                    let similar: [< $name _generic >] <[$sl_t; S]> = [< $name _generic >] {
                        $($f,)* $sl_f
                    };
                    let boxed = Box::new(similar);
                    let boxed_unsized: Box<[< $name _generic >] <[$sl_t]>> = boxed;
                    return ::std::mem::transmute::<Box<[< $name _generic >] <[$sl_t]>>, Box<$name>>(boxed_unsized);
                }
            }
        }
    };
}

#[cfg(test)]
mod tests{

    with_unsized_new!{
        #[derive(Debug)]
        struct S{
            a: u16,
            b: u32,
            ~c: [u64]
        }
    }

    #[test]
    fn test_basic(){
        let usized: Box<S> = unsafe{
            S::create_unchecked(1, 2, [33, 44, 55, 66])
        };
        println!("{usized:?}");
    }
}