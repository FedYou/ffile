pub fn permissions_mode_to_string(mode: u32) -> String {
    let mut s = String::new();
    let flags = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ];

    for (bit, ch) in flags {
        s.push(if mode & bit != 0 { ch } else { '-' });
    }

    s
}
