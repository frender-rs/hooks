use bg::{build, builder};

builder! {
    struct MyProps {
        optional_field: String = Default::default(),
        required_field: i32,
    }
}

/// Please manually move the cursor up of `^  SUGGEST HERE`
/// and type `Ctrl + Space` to auto suggest completion.
#[test]
pub fn struct_style() {
    let _ = build!(MyProps {});
    //                      ^  SUGGEST HERE

    let _ = build!(MyProps {
        //                  ^  SUGGEST HERE (NOT WORKING)
        optional_field: "".into()
    });

    let _ = build!(MyProps {
        //                  ^  SUGGEST HERE (NOT WORKING)
        optional_field: "".into(),
    });

    let _ = build!(MyProps {
        //                  ^  SUGGEST HERE (NOT WORKING)
        optional_field: "".into(),
        ..build!(MyProps {})
    });

    let _ = build!(MyProps {
        //                  ^  SUGGEST HERE (NOT WORKING)
        ..build!(MyProps {})
    });

    let _ = build!(MyProps {
        optional_field: "".into(),
        //                        ^  SUGGEST HERE
    });

    let _ = build!(MyProps {
        optional_field: "".into(),
        //                        ^  SUGGEST HERE (NOT WORKING)
        required_field: 1
    });

    let _ = build!(MyProps {
        optional_field: "".into(),
        //                        ^  SUGGEST HERE (NOT WORKING)
        required_field: 1,
    });

    let _ = build!(MyProps {
        optional_field: "".into(),
        //                        ^  SUGGEST HERE (NOT WORKING)
        ..build!(MyProps {})
    });
}

/// Please manually move the cursor up of `^  SUGGEST HERE`
/// and type `.` to auto suggest completion.
pub fn fn_style() {
    let _ = build!(MyProps());
    //                      ^  SUGGEST HERE

    let _ = build!(MyProps().optional_field("".into()));
    //                      ^  SUGGEST HERE

    let _ = build!(MyProps(build!(MyProps())).optional_field("".into()));
    //                                       ^  SUGGEST HERE

    let _ = build!(MyProps(build!(MyProps())));
    //                                       ^  SUGGEST HERE

    let _ = build!(MyProps().optional_field("".into()));
    //                                                ^  SUGGEST HERE (NOT WORKING)

    let _ = build!(MyProps().optional_field("".into()).required_field(1));
    //                                                ^  SUGGEST HERE

    let _ = build!(MyProps(build!(MyProps())).optional_field("".into()));
    //                                                                 ^  SUGGEST HERE
}
