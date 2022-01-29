extern crate proc_macro;

#[macro_export]
macro_rules! run_health_check {
    ( $x:expr ) => {
        if $x.borrow().is_fainted() {
            continue;
        }
    };
}

#[macro_export]
macro_rules! if_healthy {
    ( $x:expr, $code:block ) => {{
        if $x.borrow().has_health() $code
    }};
}

#[macro_export]
macro_rules! if_ko {
    ( $x:expr, $code:block ) => {{
        if $x.borrow().is_fainted() $code
    }};
}

#[macro_export]
macro_rules! end_if_ko {
    ( $x:expr ) => {{
        if $x.borrow().is_fainted() {
            return;
        }
    }};
    ( $x: expr, $ret: expr) => {{
        if $x.borrow().is_fainted() {
            return $ret;
        }
    }};
}