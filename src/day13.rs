pub fn count_valid_packet_pairs(input: &str) -> usize {
    let packets = parse_packets(input);
    0
}

fn parse_packets(input: &str) -> usize {
    for pair in input.split("\r\n\r\n") {
        let mut rows = pair.split("\r\n");
        let mut first_row = String::from(rows.next().unwrap());
        //first_row.remove(0);
        //first_row.remove(first_row.len() - 1);
        let mut second_row = String::from(rows.next().unwrap());
        //second_row.remove(0);
        //second_row.remove(second_row.len() - 1);

        let mut lists: Vec<String> = vec![];
        find_lists(first_row, &mut lists);
        println!("{:?}", lists);
        // println!("first: {} -- second: {}", first_row, second_row);
    }
    0
}

fn find_lists(input: String, output: &mut Vec<String>) {
    if let Some(start) = input.find("[") {
        if let Some(end) = input[start..].find("]") {
            output.push(String::from(&input[start..=end]));

            let next = start + 3;
            if next < input.len() - 1 {
                find_lists(String::from(&input[next..]), output);
            }

            /*
            if let Some(another_start) = input[start + 1..end].find("[") {
                find_lists_recursive(String::from(&input[another_start..]), output);
            }*/
        }

        /*
        if let Some(end) = input[start..].find("]") {
            output.push(String::from(&input[start..=end]));
            let mut next = find_lists_recursive(String::from(&input[end..]));
            output.append(&mut next);
        }
        */
    }
}
