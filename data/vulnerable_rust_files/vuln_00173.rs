// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0015
// CVE: CVE-2021-26951
// CWE: N/A
// Crate: calamine
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/calamine
// Extraction Method: repo_search
// GitHub Repo: tafia/calamine
// Commit: 38ef0e3
// File: src/xlsx/mod.rs


Ok(merge_cells)
 }
 
struct CellReference {
    row: u32,
    col: u32,
    absolute_row: bool,
    absolute_col: bool,
 }
 
// Parse a cell reference as it appears in formulas (e.g., "A1", "$A1", "A$1", "$A$1").
fn parse_cell_reference(name: &[u8]) -> Result<CellReference, XlsxError> {
    let mut cell = CellReference {
        row: 0,
        col: 0,
        absolute_row: false,
        absolute_col: false,
    };

    for &c in name.iter() {
        match c {
            b'$' => {
                if cell.col == 0 {
                    cell.absolute_col = true;
                } else if cell.row == 0 {
                    cell.absolute_row = true;
                } else {
                    return Err(XlsxError::Alphanumeric(c));
                 }
            }
            c @ b'A'..=b'Z' | c @ b'a'..=b'z' => {
                if cell.row != 0 {
                    return Err(XlsxError::Alphanumeric(c));
                 }
                cell.col = cell
                    .col
                    .wrapping_mul(26)
                    .wrapping_add((c.to_ascii_uppercase() - b'A') as u32 + 1);
            }
            c @ b'0'..=b'9' => {
                if cell.col == 0 {
                    return Err(XlsxError::RangeWithoutColumnComponent);
                 }
                cell.row = cell.row.wrapping_mul(10).wrapping_add((c - b'0') as u32);
             }
            _ => return Err(XlsxError::Alphanumeric(c)),
         }
     }
 
    cell.row = cell
        .row
        .checked_sub(1)
        .ok_or(XlsxError::RangeWithoutRowComponent)?;
    cell.col = cell
        .col
        .checked_sub(1)
        .ok_or(XlsxError::RangeWithoutColumnComponent)?;
 
    if cell.col >= MAX_COLUMNS {
        return Err(XlsxError::ColumnNumberOverflow);
    }
    if cell.row >= MAX_ROWS {
        return Err(XlsxError::RowNumberOverflow);
    }
 
    Ok(cell)
}
 
// Format a cell reference (e.g., "$A$1", "A1", etc.).
fn format_cell_reference(cell: &CellReference, buf: &mut Vec<u8>) -> Result<(), XlsxError> {
    if cell.col >= MAX_COLUMNS {
        return Err(XlsxError::ColumnNumberOverflow);
    }
    if cell.row >= MAX_ROWS {
        return Err(XlsxError::RowNumberOverflow);
     }
    if cell.absolute_col {
        buf.push(b'$');
     }
    column_number_to_name(cell.col, buf)?;
    if cell.absolute_row {
        buf.push(b'$');
     }
    buf.extend((cell.row + 1).to_string().into_bytes());
    Ok(())
 }
 
// Advance the cell name by the offset.
fn offset_cell_name(name: &[u8], offset: (i64, i64), buf: &mut Vec<u8>) -> Result<(), XlsxError> {
    let cell = parse_cell_reference(name)?;

    let new_row = if cell.absolute_row {
        cell.row
    } else {
        (cell.row as i64 + offset.0) as u32
    };
    let new_col = if cell.absolute_col {
        cell.col
    } else {
        (cell.col as i64 + offset.1) as u32
    };

    format_cell_reference(
        &CellReference {
            row: new_row,
            col: new_col,
            absolute_row: cell.absolute_row,
            absolute_col: cell.absolute_col,
        },
        buf,
    )
 }
 
 // Advance all valid cell names in the string by the offset.
 fn replace_cell_names(s: &str, offset: (i64, i64)) -> Result<String, XlsxError> {
     let bytes = s.as_bytes();
     let mut res: Vec<u8> = Vec::new();
    let mut cell_start = 0;
    let mut cell_end = 0;
    let mut is_cell_row = false;
     let mut in_quote = false;
     for (i, &c) in bytes.iter().enumerate() {
        if c == b'"' {
            in_quote = !in_quote;
        }
        if in_quote {
            res.push(c);
            continue;
        }
        if c.is_ascii_alphabetic() || c == b'$' {
            if is_cell_row {
                // two cell not possible stick together in formula
                res.extend(&bytes[cell_start..cell_end]);
                cell_start = i;
                is_cell_row = false;
            }
            cell_end = i + 1;
        } else if c.is_ascii_digit() {
            is_cell_row = true;
            cell_end = i + 1;
         } else {
            let cell = &bytes[cell_start..cell_end];
            if offset_cell_name(cell, offset, &mut res).is_err() {
                res.extend(cell);
             }
            cell_start = i + 1;
            cell_end = i + 1;
            is_cell_row = false;
             res.push(c);
         }
     }
    if cell_start < cell_end {
        let cell = &bytes[cell_start..cell_end];
        if offset_cell_name(cell, offset, &mut res).is_err() {
            res.extend(cell);
        }
     }
     match String::from_utf8(res) {
         Ok(s) => Ok(s),
         Err(_) => Err(XlsxError::Unexpected("fail to convert cell name")),
     }
 
     #[test]
    fn test_parse_cell_reference() {
        let check = |input: &[u8], row, col, abs_row, abs_col| {
            let cell = parse_cell_reference(input).unwrap();
            assert_eq!(
                (cell.row, cell.col, cell.absolute_row, cell.absolute_col),
                (row, col, abs_row, abs_col),
            );
         };
 
        check(b"A1", 0, 0, false, false);
        check(b"$A1", 0, 0, false, true);
        check(b"A$1", 0, 0, true, false);
        check(b"$A$1", 0, 0, true, true);
        check(b"XFD1048576", MAX_ROWS - 1, MAX_COLUMNS - 1, false, false);
     }
 
     #[test]
    fn test_format_cell_reference() {
        let check = |row, col, abs_row, abs_col, expected: &[u8]| {
             let mut buf = Vec::new();
            format_cell_reference(
                &CellReference {
                    row,
                    col,
                    absolute_row: abs_row,
                    absolute_col: abs_col,
                },
                &mut buf,
            )
             .unwrap();
             assert_eq!(buf, expected);
         };
 
        check(0, 0, false, false, b"A1");
        check(0, 0, false, true, b"$A1");
        check(0, 0, true, false, b"A$1");
        check(0, 0, true, true, b"$A$1");
        check(MAX_ROWS - 1, MAX_COLUMNS - 1, false, false, b"XFD1048576");
    }

    #[test]
    fn test_format_cell_reference_overflow() {
        let check_col_err = |row, col, abs_row, abs_col| {
             let mut buf = Vec::new();
            assert!(matches!(
                format_cell_reference(
                    &CellReference {
                        row,
                        col,
                        absolute_row: abs_row,
                        absolute_col: abs_col,
                    },
                    &mut buf
                ),
                Err(XlsxError::ColumnNumberOverflow)
            ));
            assert!(buf.is_empty(), "buffer should not be modified on error");
         };
        let check_row_err = |row, col, abs_row, abs_col| {
             let mut buf = Vec::new();
            assert!(matches!(
                format_cell_reference(
                    &CellReference {
                        row,
                        col,
                        absolute_row: abs_row,
                        absolute_col: abs_col,
                    },
                    &mut buf
                 ),
                Err(XlsxError::RowNumberOverflow)
            ));
            assert!(buf.is_empty(), "buffer should not be modified on error");
         };
 
        check_col_err(0, MAX_COLUMNS, false, false);
        check_col_err(0, MAX_COLUMNS, false, true);
        check_col_err(0, MAX_COLUMNS, true, false);
        check_col_err(0, MAX_COLUMNS, true, true);
 
        check_row_err(MAX_ROWS, 0, false, false);
        check_row_err(MAX_ROWS, 0, true, false);
        check_row_err(MAX_ROWS, 0, false, true);
        check_row_err(MAX_ROWS, 0, true, true);
     }
 
     #[test]
    fn test_offset_cell_name() {
         let check = |input: &[u8], offset, expected: &[u8]| {
             let mut buf = Vec::new();
            offset_cell_name(input, offset, &mut buf).unwrap();
             assert_eq!(buf, expected);
         };
 
         check(b"A1", (1, 1), b"B2");
         check(b"$A1", (1, 1), b"$A2");
         check(b"A$1", (1, 1), b"B$1");
         check(b"$A$1", (1, 1), b"$A$1");
     }
 
     #[test]
    fn test_parse_cell_reference_overflow() {
         let check_col_err = |input: &[u8]| {
             assert!(matches!(
                parse_cell_reference(input),
                 Err(XlsxError::ColumnNumberOverflow)
             ));
         };
         let check_row_err = |input: &[u8]| {
             assert!(matches!(
                parse_cell_reference(input),
                 Err(XlsxError::RowNumberOverflow)
             ));
         };
 
         check_col_err(b"XFE1");
         check_col_err(b"AAAA1");
         check_row_err(b"A1048577");
         check_row_err(b"A99999999999999999999");
         check_col_err(b"$XFE$1");
     }
 
     #[test]
    fn test_offset_cell_name_overflow() {
         let check_col_err = |input: &[u8], offset| {
             let mut buf = Vec::new();
             assert!(matches!(
                offset_cell_name(input, offset, &mut buf),
                 Err(XlsxError::ColumnNumberOverflow)
             ));
            assert!(buf.is_empty(), "buffer should not be modified on error");
         };
         let check_row_err = |input: &[u8], offset| {
             let mut buf = Vec::new();
             assert!(matches!(
                offset_cell_name(input, offset, &mut buf),
                 Err(XlsxError::RowNumberOverflow)
             ));
            assert!(buf.is_empty(), "buffer should not be modified on error");
         };
 
        // Original cell reference is out of bounds
         check_col_err(b"XFE1", (0, 0));
         check_col_err(b"$XFE$1", (0, 0));
         check_row_err(b"A1048577", (0, 0));
         check_row_err(b"$A$1048577", (0, 0));
 
         // Offset pushes valid cell out of bounds
         check_col_err(b"XFD1", (0, 1));
         check_row_err(b"A1048576", (1, 0));
         check_row_err(b"XFD1048576", (1, 0));
         check_col_err(b"XFD1048576", (0, 1));
     }
 
     #[test]
             replace_cell_names("한글 A1 テスト", (0, 1)).unwrap(),
             "한글 B1 テスト".to_owned()
         );
     }
 
     #[test]