
macro_rules! lets {
    ($($name:ident = $e:expr),*) => {{
        let mut tmp;
        $(
            let $name = $e;
            tmp = $name;
         )*
        tmp
    }};
}

macro_rules! lets2 {
    ($name:ident = $e:expr)  => {{
        let $name = $e;
        $name
    }};
    ($name:ident = $e:expr, $($name2:ident = $e2:expr),+) => {{
        let $name = $e;
        lets2!($($name2 = $e2),+)
    }};
}

fn main() {
    println!("{:?}", lets!(a = 1 + 2 + 3, b = a + 1, c = b + 2));
    println!("{:?}", lets2!(a = 1 + 2 + 3, b = a + 1, c = b + 2));
}
