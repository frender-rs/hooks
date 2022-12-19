use bg::{build, def_props};

def_props! {
    pub struct MyBaseProps {
        id[borrow? str],
    }
}

def_props! {
    struct MyProps {
        required_field: Vec<u8>,
        field_with_default: std::borrow::Cow<'static,str> = "".into(),
        inherited_props[inherit MyBaseProps],
    }
}

/// Test for ide support manually.
/// Currently this works with rust-analyzer.
/// The following variables should have fully deduced type.
pub fn type_deduce() {
    let _v = build!(MyProps {});
    let v = build!(MyProps {
        required_field: [1].into()
    });
    let v = build!(MyProps { ..v });
    let _v = build!(MyProps {
        required_field: vec![],
        ..v
    });
    let _v = build!(MyProps { id: "" });
    let v = build!(MyProps {
        required_field: vec![],
        id: ""
    });
    let v = build!(MyProps {
        field_with_default: String::new().into(),
        ..v
    });
    let _v = build!(MyProps {
        id: String::new(),
        ..v
    });
}
