fn foo() {
    || ();
    || -> i32 { 92 };
    |x| x;
    move |x: i32,| x;
    async || {};
    move || {};
    async move || {};
}
