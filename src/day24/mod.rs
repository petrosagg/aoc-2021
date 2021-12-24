pub fn part1() {
    let largest = [6, 5, 9, 8, 4, 9, 1, 9, 9, 9, 7, 9, 3, 9];
    let smallest = [1, 1, 2, 1, 1, 6, 1, 9, 5, 4, 1, 7, 1, 3];

    assert_eq!(monad(&smallest), 0);
    dbg!(smallest);
    assert_eq!(monad(&largest), 0);
    dbg!(largest);
}

/// Disassembled version of the computation performed on the ALU
///
/// This program can be thought of as operating on a number who's digits are 0-26, i.e a 26adic
/// number.
///
/// Every time the program multiplies by 26 it's the equivalent of shifting left one position.
/// Every time the program divides by 26 it's the equivalent of shifting right one position.
///
/// The program iterates over the digits of the input number. For each digit position, it looks up
/// the value of key0[position].
/// If that is positive then:
///     The `x != digit` branch is always taken because all the values in key0 are over 10 and
///        digits only go up to 9. So we always shift left by one position and then set the lower
///        digits to be `digit + key1[index]`
/// If that is negative then:
///     `x` is set to whatever we have on our lower digit plus an offset computed by indexing key0.
///     Then the lower digit of `result` is discarded by shifting right one position.
///     If the current digit matches `x` then `result` is left intact.
///
///
/// Since there are 7 positive and 7 negative values in key0, and then the value is positive the
/// code unconditionally shifts left we need to make sure that in every negative case we also shift
/// right always. Failing to do so will leave digits in `result` and it will not be zero at the
/// end.
///
/// Looking at key0 we can see that our number will first contain 3 digits because the first 3
/// entries are positive (12, 11, 12). So result will look like this:
/// [ digit0 + 7, digit1 + 15, digit2 + 2 ].
///
/// Then key0 has a value of `-3` so we'll discard the lower digit of `result` and compute `x` to
/// be: `x = digit2 + 2 - 3`. Since we MUST avoid the `x != digit` branch we can extract an
/// equation:
///   digit2 + 2 - 3 == digit3
///
/// If we follow the execution till the end we come up with the following equations:
///     digit2 + 2 - 3 == digit3
///     digit4 + 14 - 9 == digit5
///     digit6 + 15 - 7 == digit7
///     digit1 + 15 - 11 == digit8
///     digit0 + 7 - 4 == digit9
///     digit11 + 2 - 8 == digit12
///     digit10 + 12 - 10 == digit13
///
/// Since we know that each digit has a minimum value of 1 and a maximum of 9, deriving what are
/// the highest valid digits and the minimum valid digits follows from the equations. For example
/// if we look at `digit2 - 1 == digit3` we can deduce that the maximum values are `digit2 = 9 &&
/// digit3 = 8` and the minimum are `digit2 = 2 && digit3 = 1`. Repeating the process for all
/// equations provides the solution for the two parts of the problem
fn monad(input: &[i64]) -> i64 {
    let key0 = [12, 11, 12, -3, 10, -9, 10, -7, -11, -4, 14, 11, -8, -10];
    let key1 = [ 7, 15,  2, 15, 14,  2, 15,  1,  15, 15, 12,  2, 13,  13];

    let mut result = 0;
    for (index, digit) in input.iter().copied().enumerate() {
        let x = result % 26 + key0[index];

        if key0[index] < 0 {
            result /= 26;
        }

        if x != digit {
            result *= 26;
            result += digit + key1[index];
        }
    }

    result
}


#[allow(unreachable_code)]
#[allow(unused_mut)]
#[allow(unused_assignments)]
#[allow(dead_code)]
#[allow(clippy::all)]
fn monad_raw(input: &[i64]) -> i64 {
    let (mut x, mut y, mut z, mut digit) = (0, 0, 0, 0);
    let mut input = input.iter().copied();

    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 1;
    x += 12;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 7;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 1;
    x += 11;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 15;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 1;
    x += 12;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 2;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 26;
    x += -3;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 15;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 1;
    x += 10;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 14;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 26;
    x += -9;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 2;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 1;
    x += 10;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 15;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 26;
    x += -7;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 1;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 26;
    x += -11;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 15;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 26;
    x += -4;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 15;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 1;
    x += 14;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 12;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 1;
    x += 11;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 2;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 26;
    x += -8;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 13;
    y *= x;
    z += y;
    digit = input.next().unwrap();
    x = 0;
    x += z;
    x = x % 26;
    z = z / 26;
    x += -10;
    x = (x == digit) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += digit;
    y += 13;
    y *= x;
    z += y;

    return z;
}
