use bg::{build, builder};

builder! {
    struct MyProps {
        id[borrow? str],
    }
}

#[test]
fn sealed() {
    let v = build!(MyProps());
    let v = build!(MyProps(v).id("hello"));

    assert_eq!(v.id, "hello");
}

builder! {
    #![inheritable]
    struct BaseProps {
        id[borrow? str],
    }
}

builder! {
    struct MyInheritedProps {
        base_props[inherit BaseProps],
        class[borrow? str],
    }
}

#[test]
fn sealed_and_inherit_from_another() {
    let v = build!(MyInheritedProps());
    let v = build!(MyInheritedProps(v).id("app").class("bg-blue".to_string()));
    assert_eq!(v.base_props.id, "app");
    assert_eq!(v.class, "bg-blue");
}
