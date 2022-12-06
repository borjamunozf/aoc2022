use std::str::FromStr;

/*struct Task {}

impl FromStr for Task {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

    }
}*/

fn main() {
    let input: Vec<&str> = include_str!("../../input/input.test").split("\n").collect();
    let count_overlap = input
        .iter()
        .map(|f| {
            let (region_pair1, region_pair2) = f.split_once(",").unwrap();
            return (region_pair1, region_pair2);
        })
        .filter(|p| {
            let region1 = p.0.split("-");
            let region2 = p.1.split("-");
            let region1: Vec<i32> = region1.map(|m| m.parse().unwrap()).collect();
            let region2: Vec<i32> = region2.map(|m| m.parse().unwrap()).collect();

            if region1[0] <= region2[0] && region1[1] >= region2[1] {
                println!("Region overlap: {:?} >= {:?}", region1, region2);
                return true;
            } else if region2[0] <= region1[0] && region2[1] >= region1[1] {
                println!("Region overlap2: {:?} => {:?}", region2, region1);
                return true;
            } else {
                return false;
            }
        })
        .count();

        println!("Pt1: Overlap: {}", count_overlap);

}