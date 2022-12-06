//start of a packet is indicated by a sequence of four characters that are all different.
use std::collections::HashSet;

fn packet_marker(data_stream: &str) -> u32 {

    for (ind, _) in data_stream.char_indices() {
        let set: HashSet<char> = data_stream[ind..ind+4].chars().collect();
        if set.len() == 4 {
            return ind as u32 +4;
        }
    }  
    return 0;
}

fn message_marker(data_stream: &str) -> u32 {
    for (ind, _) in data_stream.char_indices() {
        let set: HashSet<char> = data_stream[ind..ind+14].chars().collect();
        if set.len() == 14 {
            return ind as u32 +14;
        }
    }  
    return 0;
}

fn main() {
    
    let input: &str = include_str!("../input/input.txt");

    println!("How many chars need to be processed for first packet? {}", packet_marker(input));
    println!("How many chars need to be processed for first message? {}", message_marker(input));

}


#[cfg(test)]
mod tests {
    use crate::packet_marker;

    #[test]
    fn identify_packet_market_is_correct() {
        let example_1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let example_2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        let example_3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let example_4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(5, packet_marker(example_1));
        assert_eq!(6, packet_marker(example_2));
        assert_eq!(10, packet_marker(example_3));
        assert_eq!(11, packet_marker(example_4));
    }
}