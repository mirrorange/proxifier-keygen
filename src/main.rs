extern crate rand;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::time::{SystemTime, UNIX_EPOCH};

fn handle(s: &str) -> u128 {
    let mut res = 0;
    for c in s.chars().rev() {
        res <<= 5;
        let t = c as u128;
        match c {
            'W' => continue,
            'X' => res += 24,
            'Y' => res += 1,
            'Z' => res += 18,
            '0'..='9' => res += t - 48,
            _ => res += t - 55,
        }
    }
    res
}

fn handle_re(s: u128, len: usize) -> String {
    let mut res = String::new();
    let mut s = s;
    for _ in 0..len {
        let t = s.rem_euclid(32);
        s /= 32;
        let c = match t {
            0 => 'W',
            24 => 'X',
            1 => 'Y',
            18 => 'Z',
            0..=9 => std::char::from_u32((t + 48) as u32).unwrap(),
            _ => std::char::from_u32((t + 55) as u32).unwrap(),
        };
        res.push(c);
    }
    res
}



fn crc32_like(n: u128) -> u128 {
    let mut res = 0;
    for i in 0..12 {
        let v2 = ((n >> (8 * i)) & 0xff) << 24;
        if i != 0 {
            res ^= v2;
        } else {
            res = (!v2) & 0xffffffff;
        }
        for _ in 0..8 {
            res *= 2;
            if res >= 0xffffffff {
                res &= 0xffffffff;
                res ^= 0x4C11DB7;
            }
        }
    }
    res
}

fn keygen(version: &str) -> String {
    let product = match version {
        "setup" => 0,
        "portable" => 1,
        "mac" => 2,
        _ => panic!("版本参数错误!"),
    };

    let character_table = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXZY";
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let seed = since_the_epoch.as_secs();
    let mut rng = StdRng::seed_from_u64(seed);

    let mut key_4th = String::new();
    for _ in 0..5 {
        let idx = rng.gen_range(0..character_table.len());
        key_4th.push(character_table.chars().nth(idx).unwrap());
    }

    let low_4b = rng.gen_range(0x2580..0xFFFF) + (product << 21);
    let mid_4b = rng.gen_range(0..0xFFFF);
    let high_4b = handle(&key_4th);
    let res = crc32_like((high_4b << 64) + ((mid_4b as u128) << 32) + low_4b as u128);
    let v17 = res & 0x1FFFFFF;
    let v18 = v17 ^ (v17 << 7);
    let key_5th = handle_re(v17, 5);
    let key_0_7_ch = handle_re((low_4b as u128) ^ 0x12345678 ^ v18, 7);
    let key_7_14_ch = handle_re((mid_4b as u128) ^ 0x87654321 ^ v18, 7);

    let mut key = String::new();
    key += &key_0_7_ch[0..2];
    key.push(character_table.chars().nth(rng.gen_range(0..34)).unwrap());
    key += &key_0_7_ch[3..5];
    key.push('-');
    key += &key_0_7_ch[5..7];
    key += &key_7_14_ch[0..3];
    key.push('-');
    key += &key_7_14_ch[3..7];
    key.push(key_0_7_ch.chars().nth(2).unwrap());
    key.push('-');
    key += &key_4th;
    key.push('-');
    key += &key_5th;
    key
}

fn main() {
    println!("欢迎使用Proxifier激活工具。本程序仅供学习目的，请支持正版软件。");
    println!("------------------------------------------------------------");
    println!("安装版：{}", keygen("setup"));
    println!("便携版：{}", keygen("portable"));
    println!("Mac版：{}", keygen("mac"));
    println!("------------------------------------------------------------");
    println!("按任意键退出...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}