use std::fs;

fn main() {
    let input = fs::read_to_string("11.txt").unwrap().trim().to_string();
    let part1 = next_password(input);
    println!("{}", part1);
    let part2 = next_password(part1);
    println!("{}", part2);
}

fn next_password(current_password: String) -> String {
    let mut password = current_password.into_bytes();
    assert!(password.len() >= 5);
    password.reverse();
    password[0] += 1;
    carry(&mut password);
    loop {
        remove_iol(&mut password);
        if ensure_pairs(&mut password) { continue; }
        if ensure_run(&mut password) { continue; }
        break;
    }
    password.reverse();
    String::from_utf8(password).unwrap()
}

fn remove_iol(password: &mut Vec<u8>) -> bool {
    let mut should_reset = false;
    for character in password.iter_mut().rev() {
        if should_reset {
            *character = b'a';
            continue;
        }
        match *character {
            b'i' | b'o' | b'l' => {
                *character += 1;
                should_reset = true;
            }
            _ => {},
        }
    }
    should_reset
}

fn ensure_pairs(password: &mut Vec<u8>) -> bool {
    assert!(password.len() >= 4);
    // We need a pair in [2, ...) or we can't possibly have two pairs.
    if count_pairs(&password[2..]) == 0 {
        if password[2] < password[3] {
            password[2] = password[3];
        } else {
            password[3] += 1;
            carry(password);
            password[2] = b'a';
        }
        password[1] = b'a';
        password[0] = b'a';
        return true;
    }
    // Otherwise, put one at the end if we have an earlier pair but not a later pair.
    if count_pairs(password) < 2 {
        if password[0] < password[1] {
            password[0] = password[1];
        } else {
            password[1] += 1;
            carry(password);
            password[0] = b'a';
        }
        return true;
    }
    false
}

fn count_pairs(password: &[u8]) -> i32 {
    let mut pair_count = 0;
    let mut skip_count = 0;
    for pair in password.windows(2) {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }
        if pair[0] == pair[1] {
            pair_count += 1;
            skip_count = 1;
        }
    }
    return pair_count;
}

fn ensure_run(password: &mut Vec<u8>) -> bool {
    assert!(password.len() >= 3);
    if has_run(password) {
        return false;
    }
    if password[1] == password[2] + 1 {
        password[0] = password[1] + 1;
    } else if password[1] < password[2] {
        password[1] = password[2] + 1;
        password[0] = b'a';
    } else {
        password[2] += 1;
        password[1] = b'a';
        password[0] = b'a';
    }
    carry(password);
    true
}

fn has_run(password: &[u8]) -> bool {
    for window in password.windows(3) {
        if window[0] == window[1] + 1 && window[1] == window[2] + 1 {
            return true;
        }
    }
    false
}

fn carry(password: &mut Vec<u8>) {
    let mut has_carry = false;
    for character in password.iter_mut() {
        if has_carry { *character += 1; }
        has_carry = *character > b'z';
        if has_carry { *character = b'a'; }
    }
    assert!(!has_carry);
}
