macro_rules! cond {
    {
        $c:expr => $d:expr
    } => {{
        if $c {
            $d
        }
    }};
    {
        $c:expr => $d:expr, 
        $($c2:expr => $d2:expr),+
    } => {{
        if $c {
            $d
        } else {
            cond!{$($c2 => $d2),+}
        }
    }};
}

fn main() {
    let a = 2;
    cond! {
        a == 1 => {
            println!("a");
        },
        a == 2 => {
            println!("b");
        },
        a == 3 => {
            println!("c");
        }
    };
}

