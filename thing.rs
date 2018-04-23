#![recursion_limit = "10000"]
#![feature(trace_macros)]
#![feature(log_syntax)]

macro_rules! _add_truncate {
    //( $lhs:ident $rhs:ident $carry:ident )
    ( _a _a _a ) => { _a };
    ( _a _a _b ) => { _b };
    ( _a _b _a ) => { _b };
    ( _a _b _b ) => { _a };
    ( _b _a _a ) => { _b };
    ( _b _a _b ) => { _a };
    ( _b _b _a ) => { _a };
    ( _b _b _b ) => { _b };
}

macro_rules! _add_carry {
    //( $lhs:ident $rhs:ident $carry:ident )
    ( _a _a _a ) => { _a };
    ( _a _a _b ) => { _a };
    ( _a _b _a ) => { _a };
    ( _a _b _b ) => { _b };
    ( _b _a _a ) => { _a };
    ( _b _a _b ) => { _b };
    ( _b _b _a ) => { _b };
    ( _b _b _b ) => { _b };
}

macro_rules! __add{
    (
        $carry:ident
            ( )
            ( $( $rhs_rest:ident )+ )
    ) => {
        __add! { $carry ( _a ) ( $( $rhs_rest )+ ) }
    };
    (
        $carry:ident
            ( $( $lhs_rest:ident )+ )
            ( )
    ) => {
        __add! { $carry ( $( $lhs_rest )+ ) ( _a ) }
    };
    (
        _b
            ( )
            ( )
    ) => { let _b = 1; };
    (
        _a
            ( )
            ( )
    ) => { };
    /*(
        $carry:ident
        ( $lhs:ident $( $lhs_rest:ident )* )
        ( $rhs:ident $( $rhs_rest:ident )* )
    ) => {
        _add_truncate!{$carry $lhs $rhs} __add!{ __add_carry!{$carry $lhs $rhs} ( $( $lhs_rest )+ ) ( $( $rhs_rest )+ ) }
};*/
    ( _a ( _a $( $l:ident )* ) ( _a $( $r:ident )* ) ) => { let _a = { __add!{ _a ( $( $l )* ) ( $( $r )* ) } }; };
    ( _a ( _a $( $l:ident )* ) ( _b $( $r:ident )* ) ) => { let _b = { __add!{ _a ( $( $l )* ) ( $( $r )* ) } }; };
    ( _a ( _b $( $l:ident )* ) ( _b $( $r:ident )* ) ) => { let _a = { __add!{ _b ( $( $l )* ) ( $( $r )* ) } }; };
    ( _a ( _b $( $l:ident )* ) ( _a $( $r:ident )* ) ) => { let _b = { __add!{ _a ( $( $l )* ) ( $( $r )* ) } }; };
    ( _b ( _a $( $l:ident )* ) ( _b $( $r:ident )* ) ) => { let _a = { __add!{ _b ( $( $l )* ) ( $( $r )* ) } }; };
    ( _b ( _a $( $l:ident )* ) ( _a $( $r:ident )* ) ) => { let _b = { __add!{ _a ( $( $l )* ) ( $( $r )* ) } }; };
    ( _b ( _b $( $l:ident )* ) ( _a $( $r:ident )* ) ) => { let _a = { __add!{ _b ( $( $l )* ) ( $( $r )* ) } }; };
    ( _b ( _b $( $l:ident )* ) ( _b $( $r:ident )* ) ) => { let _b = { __add!{ _b ( $( $l )* ) ( $( $r )* ) } }; };
}

macro_rules! add {
    (
        ( $( $num:ident )+ )
        plus
        ( $( $onum:ident )+ )    
    ) => {
        __add!{
            _a
                ( $( $num  )+ )
                ( $( $onum )+ )
        }
    };
}

macro_rules! conv_d2a {
    ( 0 ) => { _a };
    ( 1 ) => { _b };
    ( $first_thing:tt $( $thing:tt )+ ) => {
        $( conv_d2a!{$thing} )+ conv_d2a!{$first_thing} 
    }
}

macro_rules! conv_a2d {
    ( _a ) => { 0 };
    ( _b ) => { 1 };
    ( $first_thing:ident $( $thing:ident )+ ) => {
        conv_a2d!{ $( $thing )+ } conv_a2d!{$first_thing}
    }
}

macro_rules! print_idents {
    ( $( $arg:ident )+ ) => {
        log_syntax! { $arg }
    }
}

fn main() {

    trace_macros!(true);

    //add! { ( conv_d2a!{ 1 0 1 } ) plus ( conv_d2a!{ 1 0 0 }  ) }
    add! { ( _b _a _b ) plus ( _a _a _b _b _b _b ) };

    trace_macros!(false);
    return;
}
