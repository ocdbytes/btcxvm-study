/// Print the stack_ops in a formatted box
pub fn print_in_box(array: &mut Vec<String>) {
    array.reverse();
    // Find the longest string to determine the width of the box
    let max_length = array.iter().map(|s| s.len()).max().unwrap_or(0);

    // Print the top border
    println!("{}", "-".repeat(max_length + 4));

    // Print each item in the array, centered in the box
    for item in array {
        let padding = max_length - item.len();
        let left_padding = padding / 2;
        let _right_padding = padding - left_padding;

        println!(
            "|{padding:>width$}{item}{padding:>width$}|",
            item = item,
            padding = "",
            width = left_padding + 1
        );
    }

    // Print the bottom border
    println!("{}", "-".repeat(max_length + 4));
}
