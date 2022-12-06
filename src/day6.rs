const MARKER_SIZE_MESSAGE: usize = 14;
const MARKER_SIZE_PACKET: usize = 4;

pub enum MarkerType {
    Packet,
    Message,
}

pub fn find_marker_index(input: &str, marker_type: MarkerType) -> Option<usize> {
    let marker_size = match marker_type {
        MarkerType::Message => MARKER_SIZE_MESSAGE,
        MarkerType::Packet => MARKER_SIZE_PACKET,
    };
    for i in 0..input.len() {
        if is_marker(input, i, marker_size) {
            return Some(i + marker_size);
        }
    }
    None
}

fn is_marker(input: &str, start_index: usize, marker_size: usize) -> bool {
    if start_index + marker_size >= input.len() {
        return false;
    }
    let chars = input[start_index..start_index + marker_size].as_bytes();

    for i in 0..marker_size {
        for j in 0..marker_size {
            if i != j && chars[i] == chars[j] {
                return false;
            }
        }
    }

    true
}
