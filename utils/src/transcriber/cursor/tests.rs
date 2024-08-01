use crate::cursor::CursorNavigation;

use super::*;

#[test]
fn test_cursor() {
    let source = vec![1, 2, 3, 4, 5];
    let mut cursor = TranscriberCursor::new(&source);

    assert_eq!(cursor.pos, 0, "cursor position should be 0");
    assert_eq!(cursor.peek(), Some(&1), "cursor peek should be 1");
    assert_eq!(cursor.consume(), Some(&1), "cursor consume should be 1");
    assert_eq!(cursor.pos, 1, "cursor position should be 1");
    assert_eq!(cursor.peek(), Some(&2), "cursor peek should be 2");
    assert_eq!(cursor.consume(), Some(&2), "cursor consume should be 2");
    assert_eq!(cursor.pos, 2, "cursor position should be 2");
    assert_eq!(cursor.peek(), Some(&3), "cursor peek should be 3");

    cursor.back();
    assert_eq!(cursor.pos, 1, "cursor position should be 1");
    assert_eq!(cursor.peek(), Some(&2), "cursor peek should be 2");

    assert!(!cursor.is_overflow(), "cursor should not be overflow");
    cursor.pos = 5;
    assert!(cursor.is_overflow(), "cursor should be overflow");
}