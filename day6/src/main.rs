mod data;

fn find_start_offset(data_stream: &str, header_size: usize) -> Option<usize> {
    debug_assert!(header_size > 1);
    let data_stream = data_stream.as_bytes();
    let data_len = data_stream.len();
    // the header must fit into the data stream
    if data_len <= header_size {
        return None;
    }
    let mut correction = 0;
    let window_max_size = header_size - 1;
    for end_index in 1..data_len {
        // Test if the byte next to the window is unique to it. 
        // By expanding it sequentially, we can find the slice of the header 
        // size where all bytes are unique
        let start_index = if end_index < window_max_size + correction {
            correction
        } else {
            end_index - window_max_size
        };
        let window = &data_stream[start_index..end_index];
        let byte_to_find = data_stream[end_index];
        if let Some(duplicate_index) = window.iter().position(|b| b == &byte_to_find) {
            // skip to the byte after the duplicate
            correction = start_index + duplicate_index + 1;
        } else if window.len() == window_max_size {
            // one-based index
            return Some(end_index + 1);
        }
    }
    None
}

fn calculate_solution(data_stream: &str) -> (usize, usize) {
    (
        find_start_offset(data_stream, 4).expect("Packet start found"),
        find_start_offset(data_stream, 14).expect("Message start found"),
    )
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::DATASTREAM));
}
