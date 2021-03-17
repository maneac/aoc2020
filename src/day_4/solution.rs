use crate::Day;

pub struct Container {
    input: String,
}

impl Container {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        self.input = input.to_owned();
        Ok(())
    }

    fn part_1(&self) -> Result<String, String> {
        Ok(self
            .input
            .split("\n\n")
            .filter(
                |entry| match entry.trim().split_ascii_whitespace().count() {
                    8 => true,
                    7 => !entry.contains("cid:"),
                    _ => false,
                },
            )
            .count()
            .to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        Ok(self
            .input
            .trim()
            .split("\n\n")
            .filter(|entry| {
                let mut has_cid = false;
                let mut field_count = 0;
                for field in entry.trim().split_ascii_whitespace() {
                    field_count += 1;
                    let parts =
                        field
                            .split(':')
                            .enumerate()
                            .fold([""; 2], |mut acc, (idx, chunk)| {
                                acc[idx] = chunk;
                                acc
                            });
                    match parts[0] {
                        "byr" => {
                            let year = parts[1].parse::<usize>().unwrap_or(0);
                            if !(1920..=2002).contains(&year) {
                                return false;
                            }
                        }
                        "iyr" => {
                            let year = parts[1].parse::<usize>().unwrap_or(0);
                            if !(2010..=2020).contains(&year) {
                                return false;
                            }
                        }
                        "eyr" => {
                            let year = parts[1].parse::<usize>().unwrap_or(0);
                            if !(2020..=2030).contains(&year) {
                                return false;
                            }
                        }
                        "hgt" => {
                            let height =
                                parts[1][..parts[1].len() - 2].parse::<usize>().unwrap_or(0);
                            match &parts[1][parts[1].len() - 2..] {
                                "in" => {
                                    if !(59..=76).contains(&height) {
                                        return false;
                                    }
                                }
                                "cm" => {
                                    if !(150..=193).contains(&height) {
                                        return false;
                                    }
                                }
                                _ => {
                                    return false;
                                }
                            }
                        }
                        "hcl" => {
                            if !parts[1].starts_with('#') || parts[1].len() != 7 {
                                return false;
                            }
                            if u32::from_str_radix(&parts[1][1..], 16).is_err() {
                                return false;
                            }
                        }
                        "ecl" => match parts[1] {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                            _ => {
                                return false;
                            }
                        },
                        "pid" => {
                            if parts[1].len() != 9 {
                                return false;
                            }
                            if parts[1].parse::<usize>().is_err() {
                                return false;
                            }
                        }
                        "cid" => {
                            has_cid = true;
                        }
                        _ => {}
                    }
                }
                (field_count == 8) || (field_count == 7 && !has_cid)
            })
            .count()
            .to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "abc";

        let expected = "abc";

        let mut cont = Container::new();
        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont.input);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            input: "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
                .to_string(),
        };

        let expected = 2.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example() {
        let input = Container {
            input: "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

hcl:#7haben

hcl:#123456

hcl:#888785
hgt:164cm byr:2003 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

hcl:#888785
hgt:190in byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

hcl:#888785
hgt:190 byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

hcl:#888785
hgt:21cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

hcl:#123abz
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

hcl:888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:wat
eyr:2022

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:01234567890 ecl:hzl
eyr:2022

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:f45766239 ecl:hzl
eyr:2022

hcl:#888785
hgt:164cm byr:2002 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

hcl:#888785
hgt:60in byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

hcl:#888785
hgt:190cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

hcl:#123abc
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:brn
eyr:2022

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:000000001 ecl:hzl
eyr:2022

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
                .to_string(),
        };

        let expected = 10.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
