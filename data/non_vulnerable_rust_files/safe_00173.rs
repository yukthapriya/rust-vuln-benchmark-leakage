// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0015 (FIXED)
// Crate: calamine
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: tafia/calamine
// Commit: 38ef0e3
// File: src/xlsx/mod.rs


Ok(merge_cells)
 }
 
#[derive(Debug, Copy, Clone)]
enum Reference {
    Cell {
        row: u32,
        col: u32,
        absolute_row: bool,
        absolute_col: bool,
    },
    Row {
        row: u32,
        absolute: bool,
    },
    Column {
        col: u32,
        absolute: bool,
    },
 }
 
impl Reference {
    // Create a cell reference with validation.
    fn cell(row: u32, col: u32, absolute_row: bool, absolute_col: bool) -> Result<Self, XlsxError> {
        let reference = Reference::Cell {
            row,
            col,
            absolute_row,
            absolute_col,
        };
        reference.validate()?;
        Ok(reference)
    }

    // Create a column reference with validation.
    fn column(col: u32, absolute: bool) -> Result<Self, XlsxError> {
        let reference = Reference::Column { col, absolute };
        reference.validate()?;
        Ok(reference)
    }

    // Create a row reference with validation.
    fn row(row: u32, absolute: bool) -> Result<Self, XlsxError> {
        let reference = Reference::Row { row, absolute };
        reference.validate()?;
        Ok(reference)
    }

    // Parse a reference (e.g., "A1", "$A1", "A$1", "$A$1", "E", "$E", "5", "$5").
    fn parse(name: &[u8]) -> Result<Self, XlsxError> {
        let mut iter = name.iter().peekable();
        let mut col: u32 = 0;
        let mut row: u32 = 0;
        let mut absolute_col = false;
        let mut absolute_row = false;

        while let Some(&c) = iter.next() {
            match (c, iter.peek()) {
                (b'$', Some(b'A'..=b'Z' | b'a'..=b'z')) => {
                    if row > 0 || col > 0 {
                        return Err(XlsxError::Alphanumeric(c));
                    }
                    absolute_col = true;
                 }
                (b'$', Some(b'0'..=b'9')) => {
                    if row > 0 {
                        return Err(XlsxError::Alphanumeric(c));
                    }
                    absolute_row = true;
                 }
                (b'$', _) => return Err(XlsxError::Alphanumeric(c)),
                (c @ (b'A'..=b'Z' | b'a'..=b'z'), _) => {
                    if row > 0 {
                        return Err(XlsxError::Alphanumeric(c));
                    }
                    col = col
                        .wrapping_mul(26)
                        .wrapping_add((c.to_ascii_uppercase() - b'A') as u32 + 1);
                }
                (c @ b'0'..=b'9', _) => {
                    row = row.wrapping_mul(10).wrapping_add((c - b'0') as u32);
                 }
                _ => return Err(XlsxError::Alphanumeric(c)),
             }
        }

        match (col.checked_sub(1), row.checked_sub(1)) {
            (Some(col), Some(row)) => Reference::cell(row, col, absolute_row, absolute_col),
            (Some(col), None) => Reference::column(col, absolute_col),
            (None, Some(row)) => Reference::row(row, absolute_row),
            (None, None) => Err(XlsxError::Unexpected("Empty reference")),
         }
     }
 
    // Apply offset to create a new reference with validation.
    fn offset(self, offset: (i64, i64)) -> Result<Self, XlsxError> {
        let result = match self {
            Reference::Cell {
                row,
                col,
                absolute_row,
                absolute_col,
            } => {
                let new_col = if absolute_col {
                    col
                } else {
                    (col as i64 + offset.1) as u32
                };
                let new_row = if absolute_row {
                    row
                } else {
                    (row as i64 + offset.0) as u32
                };
 
                Reference::Cell {
                    row: new_row,
                    col: new_col,
                    absolute_row,
                    absolute_col,
                }
            }
            Reference::Column { col, absolute } => {
                let new_col = if absolute {
                    col
                } else {
                    (col as i64 + offset.1) as u32
                };
 
                Reference::Column {
                    col: new_col,
                    absolute,
                }
            }
            Reference::Row { row, absolute } => {
                let new_row = if absolute {
                    row
                } else {
                    (row as i64 + offset.0) as u32
                };
 
                Reference::Row {
                    row: new_row,
                    absolute,
                }
            }
        };

        result.validate()?;
        Ok(result)
     }

    // Validate that row/column values are in bounds.
    fn validate(&self) -> Result<(), XlsxError> {
        match self {
            Reference::Cell { row, col, .. } => {
                if *col >= MAX_COLUMNS {
                    return Err(XlsxError::ColumnNumberOverflow);
                }
                if *row >= MAX_ROWS {
                    return Err(XlsxError::RowNumberOverflow);
                }
                Ok(())
            }
            Reference::Column { col, .. } => {
                if *col >= MAX_COLUMNS {
                    return Err(XlsxError::ColumnNumberOverflow);
                }
                Ok(())
            }
            Reference::Row { row, .. } => {
                if *row >= MAX_ROWS {
                    return Err(XlsxError::RowNumberOverflow);
                }
                Ok(())
            }
        }
     }

    // Format a reference to bytes.
    fn format(&self, buf: &mut Vec<u8>) -> Result<(), XlsxError> {
        match self {
            Reference::Cell {
                row,
                col,
                absolute_row,
                absolute_col,
            } => {
                if *absolute_col {
                    buf.push(b'$');
                }
                column_number_to_name(*col, buf)?;
                if *absolute_row {
                    buf.push(b'$');
                }
                buf.extend((row + 1).to_string().into_bytes());
                Ok(())
            }
            Reference::Column { col, absolute } => {
                if *absolute {
                    buf.push(b'$');
                }
                column_number_to_name(*col, buf)
            }
            Reference::Row { row, absolute } => {
                if *absolute {
                    buf.push(b'$');
                }
                buf.extend((row + 1).to_string().into_bytes());
                Ok(())
            }
        }
     }
 }
 
// Advance a reference by the offset (e.g., "A1", "E:F", "5:6", "A1:B5").
fn offset_range(range: &[u8], offset: (i64, i64), buf: &mut Vec<u8>) -> Result<(), XlsxError> {
    let colon_pos = range.iter().position(|&b| b == b':');

    match colon_pos {
        None => {
            let reference = Reference::parse(range)?;
            if !matches!(reference, Reference::Cell { .. }) {
                return Err(XlsxError::Unexpected("Single reference type must be cell"));
            }
            let offset_ref = reference.offset(offset)?;
            offset_ref.format(buf)
        }
        Some(idx) => {
            let start = &range[..idx];
            let end = &range[idx + 1..];

            let start_ref = Reference::parse(start)?;
            let end_ref = Reference::parse(end)?;

            if std::mem::discriminant(&start_ref) != std::mem::discriminant(&end_ref) {
                return Err(XlsxError::Unexpected("Range type mismatch"));
            }

            let start_offset = start_ref.offset(offset)?;
            let end_offset = end_ref.offset(offset)?;

            start_offset.format(buf)?;
            buf.push(b':');
            end_offset.format(buf)
        }
    }
 }
 
 // Advance all valid cell names in the string by the offset.
 fn replace_cell_names(s: &str, offset: (i64, i64)) -> Result<String, XlsxError> {
     let bytes = s.as_bytes();
     let mut res: Vec<u8> = Vec::new();
     let mut in_quote = false;

    let mut token_start = 0;
    let mut token_end = 0;

     for (i, &c) in bytes.iter().enumerate() {
        if !in_quote && (c.is_ascii_alphanumeric() || c == b'$' || c == b':') {
            token_end = i + 1;
         } else {
            if token_start < token_end
                && offset_range(&bytes[token_start..token_end], offset, &mut res).is_err()
            {
                res.extend(&bytes[token_start..token_end]);
             }
             res.push(c);
            token_start = i + 1;
            token_end = i + 1;

            if c == b'"' {
                in_quote = !in_quote;
            }
         }
     }

    if token_start < token_end
        && offset_range(&bytes[token_start..token_end], offset, &mut res).is_err()
    {
        res.extend(&bytes[token_start..token_end]);
     }

     match String::from_utf8(res) {
         Ok(s) => Ok(s),
         Err(_) => Err(XlsxError::Unexpected("fail to convert cell name")),
     }
 
     #[test]
    fn test_parse_reference() {
        let check_cell =
            |input: &[u8], row, col, abs_row, abs_col| match Reference::parse(input).unwrap() {
                Reference::Cell {
                    row: r,
                    col: c,
                    absolute_row: ar,
                    absolute_col: ac,
                } => {
                    assert_eq!((r, c, ar, ac), (row, col, abs_row, abs_col));
                }
                _ => panic!("Expected Cell reference"),
            };

        let check_column = |input: &[u8], col, abs| match Reference::parse(input).unwrap() {
            Reference::Column {
                col: c,
                absolute: a,
            } => {
                assert_eq!((c, a), (col, abs));
            }
            _ => panic!("Expected Column reference"),
        };

        let check_row = |input: &[u8], row, abs| match Reference::parse(input).unwrap() {
            Reference::Row {
                row: r,
                absolute: a,
            } => {
                assert_eq!((r, a), (row, abs));
            }
            _ => panic!("Expected Row reference"),
         };
 
        // Cell references
        check_cell(b"A1", 0, 0, false, false);
        check_cell(b"$A1", 0, 0, false, true);
        check_cell(b"A$1", 0, 0, true, false);
        check_cell(b"$A$1", 0, 0, true, true);
        check_cell(b"XFD1048576", MAX_ROWS - 1, MAX_COLUMNS - 1, false, false);

        // Column references
        check_column(b"A", 0, false);
        check_column(b"$A", 0, true);
        check_column(b"XFD", MAX_COLUMNS - 1, false);

        // Row references
        check_row(b"1", 0, false);
        check_row(b"$1", 0, true);
        check_row(b"1048576", MAX_ROWS - 1, false);
     }
 
     #[test]
    fn test_format_reference() {
        let check_cell = |row, col, abs_row, abs_col, expected: &[u8]| {
             let mut buf = Vec::new();
            Reference::Cell {
                row,
                col,
                absolute_row: abs_row,
                absolute_col: abs_col,
            }
            .format(&mut buf)
             .unwrap();
             assert_eq!(buf, expected);
         };
 
        let check_column = |col, absolute, expected: &[u8]| {
             let mut buf = Vec::new();
            Reference::Column { col, absolute }
                .format(&mut buf)
                .unwrap();
            assert_eq!(buf, expected);
         };

        let check_row = |row, absolute, expected: &[u8]| {
             let mut buf = Vec::new();
            Reference::Row { row, absolute }.format(&mut buf).unwrap();
            assert_eq!(buf, expected);
        };

        // Cell references
        check_cell(0, 0, false, false, b"A1");
        check_cell(0, 0, false, true, b"$A1");
        check_cell(0, 0, true, false, b"A$1");
        check_cell(0, 0, true, true, b"$A$1");
        check_cell(MAX_ROWS - 1, MAX_COLUMNS - 1, false, false, b"XFD1048576");

        // Column references
        check_column(0, false, b"A");
        check_column(0, true, b"$A");
        check_column(MAX_COLUMNS - 1, false, b"XFD");

        // Row references
        check_row(0, false, b"1");
        check_row(0, true, b"$1");
        check_row(MAX_ROWS - 1, false, b"1048576");
    }

    #[test]
    fn test_format_reference_overflow() {
        let check_err = |reference: Reference, offset| {
            let result = reference.offset(offset);
            assert!(
                matches!(
                    result,
                    Err(XlsxError::ColumnNumberOverflow) | Err(XlsxError::RowNumberOverflow)
                 ),
                "expected overflow error, got {:?}",
                result
            );
         };
 
        // Cell reference offset pushes column out of bounds
        check_err(
            Reference::Cell {
                row: 0,
                col: MAX_COLUMNS - 1,
                absolute_row: false,
                absolute_col: false,
            },
            (0, 1),
        );

        // Cell reference offset pushes row out of bounds
        check_err(
            Reference::Cell {
                row: MAX_ROWS - 1,
                col: 0,
                absolute_row: false,
                absolute_col: false,
            },
            (1, 0),
        );

        // Column reference offset pushes out of bounds
        check_err(
            Reference::Column {
                col: MAX_COLUMNS - 1,
                absolute: false,
            },
            (0, 1),
        );
 
        // Row reference offset pushes out of bounds
        check_err(
            Reference::Row {
                row: MAX_ROWS - 1,
                absolute: false,
            },
            (1, 0),
        );
     }
 
     #[test]
    fn test_offset_range() {
         let check = |input: &[u8], offset, expected: &[u8]| {
             let mut buf = Vec::new();
            offset_range(input, offset, &mut buf).unwrap();
             assert_eq!(buf, expected);
         };
 
        let check_err = |input: &[u8], offset| {
            let mut buf = Vec::new();
            let res = offset_range(input, offset, &mut buf);
            assert!(res.is_err());
            assert_eq!(buf.len(), 0)
        };

        // Cell references
         check(b"A1", (1, 1), b"B2");
         check(b"$A1", (1, 1), b"$A2");
         check(b"A$1", (1, 1), b"B$1");
         check(b"$A$1", (1, 1), b"$A$1");

        // Column references
        check_err(b"E", (0, 1));
        check_err(b"$E", (0, 1));

        // Row references
        check_err(b"5", (1, 0));
        check_err(b"$5", (1, 0));

        // Cell ranges
        check(b"A1:B2", (1, 1), b"B2:C3");
        check(b"$A$1:$B$2", (1, 1), b"$A$1:$B$2");

        // Column ranges
        check(b"E:F", (0, 1), b"F:G");
        check(b"$E:$F", (0, 1), b"$E:$F");
        check(b"E:F", (1, 0), b"E:F");

        // Row ranges
        check(b"5:6", (1, 0), b"6:7");
        check(b"$5:$6", (1, 0), b"$5:$6");
        check(b"5:6", (0, 1), b"5:6");
     }
 
     #[test]
    fn test_parse_reference_overflow() {
         let check_col_err = |input: &[u8]| {
             assert!(matches!(
                Reference::parse(input),
                 Err(XlsxError::ColumnNumberOverflow)
             ));
         };
         let check_row_err = |input: &[u8]| {
             assert!(matches!(
                Reference::parse(input),
                 Err(XlsxError::RowNumberOverflow)
             ));
         };
        let check_syntax_err = |input: &[u8]| {
            assert!(matches!(
                Reference::parse(input),
                Err(XlsxError::Alphanumeric(_))
            ));
        };
 
        // Invalid syntax
        check_syntax_err(b"A$A1");
        check_syntax_err(b"A1$2");
        check_syntax_err(b"$$A1");
        check_syntax_err(b"$A$$1");
        check_syntax_err(b"A$$1");
        check_syntax_err(b"1A");
        check_syntax_err(b"1A1");
        check_syntax_err(b"A1B2");

        // Cell references
         check_col_err(b"XFE1");
         check_col_err(b"AAAA1");
         check_row_err(b"A1048577");
         check_row_err(b"A99999999999999999999");
         check_col_err(b"$XFE$1");

        // Column references
        check_col_err(b"XFE");
        check_col_err(b"$XFE");

        // Row references
        check_row_err(b"1048577");
        check_row_err(b"$1048577");
     }
 
     #[test]
    fn test_offset_range_overflow() {
         let check_col_err = |input: &[u8], offset| {
             let mut buf = Vec::new();
             assert!(matches!(
                offset_range(input, offset, &mut buf),
                 Err(XlsxError::ColumnNumberOverflow)
             ));
         };
         let check_row_err = |input: &[u8], offset| {
             let mut buf = Vec::new();
             assert!(matches!(
                offset_range(input, offset, &mut buf),
                 Err(XlsxError::RowNumberOverflow)
             ));
         };
 
        // Original reference is out of bounds
         check_col_err(b"XFE1", (0, 0));
         check_col_err(b"$XFE$1", (0, 0));
         check_row_err(b"A1048577", (0, 0));
         check_row_err(b"$A$1048577", (0, 0));
        check_col_err(b"XFE:XFE", (0, 0));
        check_row_err(b"1048577:1048577", (0, 0));
 
         // Offset pushes valid cell out of bounds
         check_col_err(b"XFD1", (0, 1));
         check_row_err(b"A1048576", (1, 0));
         check_row_err(b"XFD1048576", (1, 0));
         check_col_err(b"XFD1048576", (0, 1));

        // Offset pushes valid range out of bounds
        check_col_err(b"XFD:XFD", (0, 1));
        check_row_err(b"1048576:1048576", (1, 0));
     }
 
     #[test]
             replace_cell_names("한글 A1 テスト", (0, 1)).unwrap(),
             "한글 B1 テスト".to_owned()
         );

        assert_eq!(
            replace_cell_names("ABC\"asd\"123", (1, 0)).unwrap(),
            "ABC\"asd\"123".to_owned()
        );

        // Column ranges
        assert_eq!(
            replace_cell_names("SUM(E:F)", (0, 1)).unwrap(),
            "SUM(F:G)".to_owned()
        );
        assert_eq!(
            replace_cell_names("SUM($E:$F)", (0, 1)).unwrap(),
            "SUM($E:$F)".to_owned()
        );
        assert_eq!(
            replace_cell_names("SUM($E:F)", (0, 1)).unwrap(),
            "SUM($E:G)".to_owned()
        );

        // Row ranges
        assert_eq!(
            replace_cell_names("SUM(5:6)", (1, 0)).unwrap(),
            "SUM(6:7)".to_owned()
        );
        assert_eq!(
            replace_cell_names("SUM($5:$6)", (1, 0)).unwrap(),
            "SUM($5:$6)".to_owned()
        );
        assert_eq!(
            replace_cell_names("SUM($5:6)", (1, 0)).unwrap(),
            "SUM($5:7)".to_owned()
        );

        // Mixed with cell references
        assert_eq!(
            replace_cell_names("SUM(A1:A5,E:F)", (0, 1)).unwrap(),
            "SUM(B1:B5,F:G)".to_owned()
        );

        // Invalid syntax
        assert_eq!(
            replace_cell_names(
                "Valid: A1 Invalid: A1B1 A1$ $$A1 $A$$1 A$$1 A:1 1:A 1 A A1:1 A1:B A$A1 A1$2 $1 $A Valid: C1:D1",
                (1, 1)
            )
            .unwrap(),
            "Valid: B2 Invalid: A1B1 A1$ $$A1 $A$$1 A$$1 A:1 1:A 1 A A1:1 A1:B A$A1 A1$2 $1 $A Valid: D2:E2"
                .to_owned()
        );
     }
 
     #[test]