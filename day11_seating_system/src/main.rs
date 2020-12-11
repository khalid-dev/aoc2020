mod part1;
mod part2;
use part1::get_occupied_count;
use part2::get_occupied_count as get_super_occupied_count;

fn main() {
    get_occupied_count();
    get_super_occupied_count();
}
