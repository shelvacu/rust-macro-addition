#![recursion_limit = "10000"]
#![feature(trace_macros)]
#![feature(log_syntax)]

macro_rules! _add{
    (
        $carry:ident
            ( )
            ( $( $r:ident )+ )
    ) => {
        _add! { $carry ( zero ) ( $( $r )+ ) }
    };
    (
        $carry:ident
            ( $( $l:ident )+ )
            ( )
    ) => {
        _add! { $carry ( $( $l )+ ) ( zero ) }
    };
    (
        one 
            ( )
            ( )
    ) => { 1 };
    (
        zero
            ( )
            ( )
    ) => { 0 };
    ( zero ( zero $( $l:ident )* ) ( zero $( $r:ident )* ) ) => { 0 + ( _add!{ zero ( $( $l )* ) ( $( $r )* ) } * 2 ) };
    ( zero ( zero $( $l:ident )* ) ( one  $( $r:ident )* ) ) => { 1 + ( _add!{ zero ( $( $l )* ) ( $( $r )* ) } * 2 ) };
    ( zero ( one  $( $l:ident )* ) ( one  $( $r:ident )* ) ) => { 0 + ( _add!{ one  ( $( $l )* ) ( $( $r )* ) } * 2 ) };
    ( zero ( one  $( $l:ident )* ) ( zero $( $r:ident )* ) ) => { 1 + ( _add!{ zero ( $( $l )* ) ( $( $r )* ) } * 2 ) };
    ( one  ( zero $( $l:ident )* ) ( one  $( $r:ident )* ) ) => { 0 + ( _add!{ one  ( $( $l )* ) ( $( $r )* ) } * 2 ) };
    ( one  ( zero $( $l:ident )* ) ( zero $( $r:ident )* ) ) => { 1 + ( _add!{ zero ( $( $l )* ) ( $( $r )* ) } * 2 ) };
    ( one  ( one  $( $l:ident )* ) ( zero $( $r:ident )* ) ) => { 0 + ( _add!{ one  ( $( $l )* ) ( $( $r )* ) } * 2 ) };
    ( one  ( one  $( $l:ident )* ) ( one  $( $r:ident )* ) ) => { 1 + ( _add!{ one  ( $( $l )* ) ( $( $r )* ) } * 2 ) };
}

macro_rules! add {
    (
        ( $( $num:ident )+ )
        plus
        ( $( $onum:ident )+ )    
    ) => {
        _add!{
            zero
                ( $( $num  )+ )
                ( $( $onum )+ )
        }
    };
}

fn main() {

    trace_macros!(true);

    //add! { ( conv_d2a!{ 1 0 1 } ) plus ( conv_d2a!{ 1 0 0 }  ) }
    let res = add!( ( one  zero one  ) plus ( zero zero one  one  one  one  ) );

    trace_macros!(false);
    println!("Result is {:}", res);
    return;
}
