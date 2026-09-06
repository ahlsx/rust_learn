

fn main(){
 //let greater_than_zero = |x:&i32| *x > 0;
 let is_even = |x:&i32| *x % 2 == 0;
 let x = 4;
 let y = -3;
 println!("Are both true? {}", are_both_true(is_greater_than_zero,is_even,&x));
 println!("Are both false? {}", are_both_false(is_greater_than_zero,is_even,&y));
}

//function pointer dont capture environment, so they can be used as function pointers
//closures can convert to function pointers if they dont capture environment, but closures that capture environment cannot be used as function pointers
fn is_greater_than_zero(x:&i32) -> bool {
    *x > 0
}

fn are_both_true<T,U,V> (a:T,b:U,c:&V) ->bool where T:Fn(&V)->bool, U:Fn(&V)->bool {
    a(c) && b(c)
}
fn are_both_false<V> (a:fn(&V)->bool,b:fn(&V)->bool,c:&V) ->bool {
    !a(c) && !b(c)
}
