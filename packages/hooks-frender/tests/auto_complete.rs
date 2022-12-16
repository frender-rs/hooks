use hooks_frender::{build, def_props};

def_props! {
    struct MyProps {
        optional_field: String = Default::default(),
        required_field: i32,
    }
}

/// Please manually move the cursor up of `^  SUGGEST HERE`
/// and type `Ctrl + Space` to auto suggest completion.
pub fn struct_style() {
    let _ = build!(MyProps {});
    //                      ^  SUGGEST HERE (NOT WORKING)

    let _ = build!(MyProps {
        //                  ^  SUGGEST HERE
        optional_field: "".into()
    });

    let _ = build!(MyProps {
        //                  ^  SUGGEST HERE
        optional_field: "".into(),
    });

    let _ = build!(MyProps {
        //                  ^  SUGGEST HERE
        optional_field: "".into(),
        ..MyProps()
    });

    // CURRENTLY NOT WORKING
    let _ = build!(MyProps {
        //                  ^  SUGGEST HERE (NOT WORKING)
        ..MyProps()
    });

    let _ = build!(MyProps {
        optional_field: "".into(),
        //                        ^  SUGGEST HERE (NOT WORKING)
    });

    let _ = build!(MyProps {
        optional_field: "".into(),
        //                        ^  SUGGEST HERE
        required_field: 1
    });

    let _ = build!(MyProps {
        optional_field: "".into(),
        //                        ^  SUGGEST HERE
        required_field: 1,
    });

    let _ = build!(MyProps {
        optional_field: "".into(),
        //                        ^  SUGGEST HERE (NOT WORKING)
        ..MyProps()
    });
}

/// Please manually move the cursor up of `^  SUGGEST HERE`
/// and type `.` to auto suggest completion.
pub fn fn_style() {
    let _ = build!(MyProps());
    //                      ^  SUGGEST HERE (NOT WORKING)

    let _ = build!(MyProps().optional_field("".into()));
    //                      ^  SUGGEST HERE

    let _ = build!(MyProps(MyProps()).optional_field("".into()));
    //                               ^  SUGGEST HERE

    let _ = build!(MyProps(MyProps()));
    //                               ^  SUGGEST HERE

    let _ = build!(MyProps().optional_field("".into()));
    //                                                ^  SUGGEST HERE (NOT WORKING)

    let _ = build!(MyProps().optional_field("".into()).required_field(1));
    //                                                ^  SUGGEST HERE

    let _ = build!(MyProps(MyProps()).optional_field("".into()));
    //                                                         ^  SUGGEST HERE
}
