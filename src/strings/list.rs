fn flick_switch(list: &[&str]) -> Vec<bool> {
    const flick:  &str = "flick";
    let mut result = vec![true;list.len()];
    let mut flag = true;
    for i in 0..list.len() {
        if list[i] != flick {
            result[i] = flag;
            continue;
        }

        flag = !flag;
        result[i] = flag;
    }
    result
}

fn flick_switch_smart(list: &[&str]) -> Vec<bool> {
    let mut f = true;
    list.iter().map(|&s| if s != "flick" {f} else {f=!f; f}).collect()
}

fn points(games: &[&str]) -> u32 {
    let mut result: u32 = 0;
    for value in games.iter() {
        let score = value.split(":")
        .collect::<Vec<&str>>();
        
        if score[0] > score[1] {
			result = result + 3;
			continue;
		}

		if score[0] == score[1] {
			result = result + 1;
			continue
		}
    }
    result
}

fn points_smart_one(games: &[&str]) -> u32 {
    games.iter().map(|s| {
        let (l,r) = s.split_once(':').unwrap();
        match l.cmp(r) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 1,
            std::cmp::Ordering::Greater => 3,
        }
    }).sum()
}

#[cfg(test)]
mod string_list_tests {
    use crate::strings::list;
    use std::borrow::Borrow;
    use std::collections::HashMap;

    #[test]
    fn test_flick_switch() {
        // Given
        let test_cases = HashMap::from([
            ("flick_1", (["codewars", "flick", "code", "wars"], [true, false, false, false])),
            ("flick_2", (["flick", "chocolate", "adventure", "sunshine"], [false, false, false, false])),
            ("flick_3", (["flick", "11037", "3.14", "53"], [false, false, false, false])),
        ]);

        for (test_name, test_data) in test_cases.iter() {
            test_flick(test_name, test_data.0, test_data.1);
        }
    }

    #[test]
    fn test_flick_switch_smart() {
        // Given
        let test_cases = HashMap::from([
            ("flick_1", (["codewars", "flick", "code", "wars"], [true, false, false, false])),
            ("flick_2", (["flick", "chocolate", "adventure", "sunshine"], [false, false, false, false])),
            ("flick_3", (["flick", "11037", "3.14", "53"], [false, false, false, false])),
        ]);

        for (test_name, test_data) in test_cases.iter() {
            test_flick_smart(test_name, test_data.0, test_data.1);
        }
    }

    fn test_flick<'a, S: Borrow<[&'a str]>, E: Borrow<[bool]>>(test_name: &str, strings: S, expected: E) {
        let strings: &[&'a str] = strings.borrow();
        let expected: &[bool] = expected.borrow();
        let got = list::flick_switch(strings);
        assert_eq!(got, expected, "{}: want: {:?}, but got: {:?}", test_name, expected, got);
    }

    fn test_flick_smart<'a, S: Borrow<[&'a str]>, E: Borrow<[bool]>>(test_name: &str, strings: S, expected: E) {
        let strings: &[&'a str] = strings.borrow();
        let expected: &[bool] = expected.borrow();
        let got = list::flick_switch_smart(strings);
        assert_eq!(got, expected, "{}: want: {:?}, but got: {:?}", test_name, expected, got);
    }

    #[test]
    fn test_points() {
        do_test_points(&["1:0", "2:0", "3:0", "4:0", "2:1", "3:1", "4:1", "3:2", "4:2", "4:3"], 30);
        do_test_points(&["1:1", "2:2", "3:3", "4:4", "2:2", "3:3", "4:4", "3:3", "4:4", "4:4"], 10);
        do_test_points(&["0:1", "0:2", "0:3", "0:4", "1:2", "1:3", "1:4", "2:3", "2:4", "3:4"], 0);
        do_test_points(&["1:0", "2:0", "3:0", "4:0", "2:1", "1:3", "1:4", "2:3", "2:4", "3:4"], 15);
        do_test_points(&["1:0", "2:0", "3:0", "4:4", "2:2", "3:3", "1:4", "2:3", "2:4", "3:4"], 12);
    }

    fn do_test_points(games: &[&str], want: u32) {
        let got = list::points(games);
        assert_eq!(want, got);
        let got = list::points_smart_one(games);
        assert_eq!(want, got);
    }
}