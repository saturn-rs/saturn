/// Imports
use crate::buffer::{Buffer, row::Row};

/// Asserts row
fn assert_row(got: &Row, expected: &Row) {
    assert_eq!(got.idx(), expected.idx());
    assert_eq!(got.text(), expected.text());
}

/// Asserts rows`
fn assert_rows(got: &[Row], expected: &[Row]) {
    assert_eq!(got.len(), expected.len());
    got.iter()
        .zip(expected)
        .for_each(|(got, expected)| assert_row(got, expected));
}

/// Tests drop first
#[test]
fn row_drop_first_test() {
    assert_row(
        &Row::new(0, "Hhhello, world!".into()).drop_first(2),
        &Row::new(0, "hello, world!".into()),
    );
}

/// Tests drop last
#[test]
fn row_drop_last_test() {
    assert_row(
        &Row::new(0, "hello, world!!!!".into()).drop_last(3),
        &Row::new(0, "hello, world!".into()),
    );
}

/// Tests row range
#[test]
fn row_range_test() {
    assert_row(
        &Row::new(0, "Hhhello, world!!!!".into()).range(2, 15),
        &Row::new(0, "hello, world!".into()),
    );
}

/// Tests basic buffer
#[test]
fn basic_buffer_test() {
    let buf = Buffer::new(
        r#"abc
def
ghi"#
            .into(),
        "test".into(),
    );
    assert_rows(
        &buf.rows((1, 1), 3, 3),
        &[Row::new(1, "ef".into()), Row::new(2, "hi".into())],
    );
}

/// Tests basic buffer
#[test]
fn buffer_less_size_test() {
    let buf = Buffer::new(
        r#"abcd
efgh
ijkl"#
            .into(),
        "test".into(),
    );
    assert_rows(
        &buf.rows((1, 1), 2, 2),
        &[Row::new(1, "fg".into()), Row::new(2, "jk".into())],
    );
}
