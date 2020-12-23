use crate::prelude::*;

pub fn day08() -> Result<()> {
    let data = INPUT;
    let instructions: Vec<Instruction> = parse_each(data.lines());

    let (total, _) = part1(&instructions);
    println!("2020 08.1 -> {}", total);

    let total2 = part2(&instructions);
    println!("2020 08.2 -> {}", total2);
    Ok(())
}

fn part1(instructions: &[Instruction]) -> (i64, bool) {
    let mut acc = 0;
    let mut cur_idx: i64 = 0;
    let mut visited: Vec<bool> = (0..instructions.len()).map(|_| false).collect();
    loop {
        if cur_idx as usize > instructions.len() - 1 {
            return (acc, true);
        }
        if visited[cur_idx as usize] {
            return (acc, false);
        }
        visited[cur_idx as usize] = true;
        let (acc_mod, idx_mod) = match instructions[cur_idx as usize] {
            Instruction::Acc(n) => (n, 1),
            Instruction::Jmp(n) => (0, n),
            Instruction::Nop(_) => (0, 1),
        };
        acc += acc_mod;
        cur_idx += idx_mod;
    }
}

fn part2(instructions: &[Instruction]) -> i64 {
    let mut idx = 0;
    loop {
        let mut local_instructions: Vec<Instruction> = instructions.to_vec();
        local_instructions[idx] = match instructions[idx] {
            Instruction::Nop(n) => Instruction::Jmp(n),
            Instruction::Jmp(n) => Instruction::Nop(n),
            Instruction::Acc(n) => Instruction::Acc(n),
        };
        let (acc, ran_to_end) = part1(&local_instructions);
        if ran_to_end {
            return acc;
        }
        idx += 1;
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        let num = parts[1].parse::<i64>()?;
        match parts[0] {
            "acc" => Ok(Instruction::Acc(num)),
            "jmp" => Ok(Instruction::Jmp(num)),
            "nop" => Ok(Instruction::Nop(num)),
            x => Err(anyhow!("Unrecognised operation `{}`", x)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let data = "nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6";
        let instructions: Vec<Instruction> = data
            .trim()
            .lines()
            .map(|x| x.trim())
            .filter_map(|x| Some(x.parse().ok()?))
            .collect();
        assert_eq!(part1(&instructions), (5, false));
    }

    #[test]
    fn example_part2() {
        let data = "nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6";
        let instructions: Vec<Instruction> = data
            .trim()
            .lines()
            .map(|x| x.trim())
            .filter_map(|x| Some(x.parse().ok()?))
            .collect();
        assert_eq!(part2(&instructions), 8);
    }
}

const INPUT: &str = "jmp +232
acc +21
nop +120
jmp +239
acc +18
acc +41
jmp +72
acc +47
jmp +314
jmp +1
acc +47
nop +175
acc +33
jmp +115
nop -5
acc +37
acc +25
acc +18
jmp +304
acc +0
acc +16
jmp +77
acc +9
acc -3
jmp +93
acc +16
acc -15
jmp +110
jmp +76
acc +36
acc +11
acc -3
jmp +258
jmp +241
acc +42
jmp +514
nop +103
acc +36
acc -18
jmp +47
acc +5
acc +37
jmp +480
acc -16
jmp +1
nop +498
jmp +1
jmp +12
acc +0
acc +35
jmp +437
jmp +326
acc -15
acc -7
nop -2
jmp +548
jmp -4
jmp +395
jmp +258
acc +37
acc +17
acc -18
jmp +345
acc -18
acc +37
acc +36
jmp +217
acc -4
acc +39
jmp -35
jmp +252
jmp +1
nop +91
jmp +402
nop -40
jmp +371
jmp -72
jmp +9
acc +41
jmp +95
nop +252
nop +30
jmp +240
nop +266
jmp +462
jmp +137
acc -14
jmp +203
jmp +1
acc +45
acc -14
acc -6
jmp -9
acc -15
acc +6
nop +298
jmp -56
jmp +14
acc +32
jmp +40
acc +17
nop +62
acc +14
jmp +119
acc +49
jmp -29
acc +27
acc -12
acc +14
acc +19
jmp +253
acc +19
jmp +345
acc -17
acc +39
jmp +1
jmp +133
jmp +268
acc -14
acc -16
acc +45
jmp +373
jmp +116
jmp +245
acc -19
acc +32
jmp -22
jmp +105
acc -9
acc +27
acc +16
nop +397
jmp +110
acc +13
acc -10
acc +10
jmp -69
jmp +29
jmp +94
acc +38
acc +49
acc +40
jmp +261
acc +43
acc -13
jmp +214
acc -10
nop -80
acc +15
jmp +228
acc +0
jmp +275
jmp -69
acc +46
acc +4
acc +24
acc +6
jmp +279
acc -9
nop +281
jmp +286
acc -4
jmp +306
jmp +342
acc -14
jmp +357
acc -10
nop -9
acc +10
acc +40
jmp +427
acc +0
acc +32
jmp +405
acc +45
acc +34
nop +281
acc +34
jmp +394
acc +41
acc +20
jmp -98
jmp -60
acc -3
acc +17
jmp +19
acc +6
nop +168
acc +35
jmp -141
nop -62
acc +8
acc +16
jmp +117
acc +34
acc -8
acc +35
acc -15
jmp +85
acc +2
acc -9
acc -4
acc +49
jmp +394
nop -145
acc +47
jmp +16
acc +10
acc +0
jmp +87
nop -88
acc -9
acc -16
acc +45
jmp +374
acc +28
acc +38
jmp -139
acc -13
acc +13
jmp +143
jmp -135
jmp -4
jmp -130
acc +5
nop -196
jmp +48
acc -10
jmp +149
acc -14
jmp +210
jmp +325
acc +45
acc +11
acc -15
jmp +97
nop +107
jmp -98
acc -7
acc -18
jmp -181
jmp +122
acc -15
jmp -49
jmp +1
acc +36
acc -10
jmp +1
jmp +62
acc +39
jmp +105
acc +19
nop +253
acc -11
acc -9
jmp +77
acc +50
acc +3
acc -18
acc +17
jmp +56
nop -209
nop +272
acc -13
jmp +270
nop +229
acc +12
jmp +1
jmp -44
acc -13
jmp +1
nop +275
acc +45
jmp -254
acc -2
acc -2
nop -148
jmp -91
acc +2
nop -30
acc -8
acc +0
jmp -96
nop +1
jmp -74
acc -19
acc +10
acc +26
acc +30
jmp -280
acc +46
acc -2
acc -8
jmp +277
acc -9
jmp +205
acc -13
acc +10
jmp +1
jmp +219
acc +38
acc +24
acc +11
jmp -129
jmp -86
jmp +1
acc +0
jmp +1
acc +46
jmp -135
nop +218
acc -14
acc +0
jmp +55
acc +24
jmp +213
acc +19
acc +16
jmp -266
acc +24
acc +15
jmp +158
acc +3
jmp -94
acc +16
acc +24
acc +42
jmp +201
jmp -32
acc +34
nop -321
jmp +212
acc +12
acc +41
jmp -212
acc +32
jmp +236
acc +45
nop +253
jmp +129
nop -3
acc +38
jmp +35
acc -15
acc +21
acc -7
acc -6
jmp +46
jmp -5
acc +5
acc +4
acc +42
jmp +142
acc +36
jmp -180
acc +23
jmp -46
acc +12
jmp +5
jmp +201
acc +36
acc -14
jmp -30
jmp -338
acc +12
acc +34
acc +2
jmp -310
acc -15
jmp -104
jmp -148
jmp +108
acc +37
acc -6
acc +0
acc +13
jmp -324
acc +49
acc +37
acc +37
jmp +131
acc +2
acc +30
acc +12
jmp -238
acc -12
acc +4
jmp -155
acc +45
acc -10
nop -168
nop +114
jmp +113
acc +15
acc +41
acc +6
acc +34
jmp +25
acc +46
acc +28
acc +44
acc -3
jmp -70
acc +2
acc +37
jmp -101
jmp +51
acc +45
nop -399
nop -60
jmp -391
acc +41
jmp -57
jmp -54
acc +46
jmp +90
acc +6
jmp +83
acc +37
jmp +1
acc -6
jmp -189
acc +0
jmp -241
acc +35
jmp -396
acc +35
acc +42
acc +37
acc +20
jmp -81
nop +74
acc +41
acc +23
jmp +1
jmp -349
jmp -232
acc +37
acc +24
jmp +121
jmp -144
acc +35
acc +39
acc -12
acc +14
jmp -113
acc +2
acc +29
acc -6
acc +0
jmp -326
jmp -426
acc +18
acc +39
acc +22
jmp +79
jmp +23
acc -17
nop +42
acc -8
jmp -47
acc -12
jmp -276
jmp -126
acc +20
acc +3
acc +41
jmp -31
acc -1
jmp +1
jmp -241
acc +9
acc +12
acc +0
jmp +26
acc +30
nop +46
jmp -134
jmp -361
acc +50
nop -1
nop -225
jmp -226
acc +42
acc +0
jmp +1
jmp -170
acc +14
acc +19
jmp -199
nop +15
acc -11
acc +20
jmp -161
nop -348
acc -6
acc +49
jmp -468
acc +11
jmp -413
acc -11
acc -1
acc +45
jmp -181
jmp -380
nop -128
acc +40
jmp -179
acc -9
acc +24
jmp -358
acc +50
acc +13
acc -15
jmp +14
acc +4
acc +12
jmp -365
nop -269
jmp -443
nop -224
jmp -108
acc +46
acc -11
jmp -515
acc -8
nop -284
jmp -444
acc +15
nop -11
jmp -288
acc +28
acc +35
jmp -416
acc +27
acc -8
acc -10
acc +0
jmp -167
acc -9
acc +42
acc +20
jmp -63
jmp -107
acc -6
jmp -335
jmp -460
acc -2
jmp -420
acc +27
acc +6
jmp -458
acc +31
nop +19
nop -396
jmp -479
nop -234
acc +42
jmp -142
jmp -511
nop +28
acc -9
acc +36
acc +38
jmp +27
acc -3
acc +9
acc -19
acc +3
jmp -133
jmp -503
jmp -267
acc +40
acc +41
acc +13
nop -492
jmp -327
jmp -339
acc +17
acc +4
acc +45
acc +13
jmp -419
acc +31
acc +0
acc +37
acc -13
jmp -210
jmp -517
acc -15
jmp -47
acc -16
jmp -129
acc +16
nop -455
nop -263
jmp -74
acc +5
acc +20
acc +45
acc +23
jmp -490
jmp -53
acc +40
jmp +1
acc -14
acc -1
jmp +1";
