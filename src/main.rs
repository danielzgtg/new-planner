use new_planner::{print_analysis, load_data, read_data};

fn main() {
    print_analysis(load_data(&read_data()));
}
