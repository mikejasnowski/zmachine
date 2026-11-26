/**
 * Main machine structure
 */

use std::fs;
use std::mem;


pub struct ZMachine {
    story_file: String,
    memory: Memory,
    pc: PC,
}

impl ZMachine{
    pub fn new(story_file: String) -> ZMachine {
        let memory = Memory::new();
        let pc = PC::new();
        ZMachine{story_file, memory, pc}
    }

    /*
    * Loads a story file into memory
    */
    pub fn load_story(&self) {
        println!("Loading Story: {}", self.story_file);

        //** Do something here with core
        let bytes: Vec<u8> = fs::read(self.story_file.clone()).unwrap();

        // println!("Read {} bytes", bytes.len());
        // println!("Version: {:?}",bytes.get(0));
        for hb in 0..64 {
            println!("Byte {:x} = {:?}",hb, bytes.get(hb));
        }

        println!("Story file loaded: {}", self.story_file);
    }
}

/**
 * Core structures of the Z-Machine 
 **/

 // .z(x) files 64 bytes
 #[derive(Default)]
 pub struct Header {
    version: u8, // 1 byte
    flags1: [u8; 3], // 3 bytes
    base_hi_mem : [u8; 2],
    initial_pc: [u8; 2],
    dict_location: [u8; 2],
    obj_table_location: [u8; 2],
    global_var_table_location: [u8; 2],
    base_static_mem: [u8; 2],
    flags2: [u8; 10],
    abbrev_table_location: [u8; 2],
    length_of_file: [u8; 2], // length of story file
    checksum: [u8; 2],
    interpreter_num: [u8; 1],
    interpreter_version: [u8; 1],
    screen_height: u8,
    screen_width: u8,
    screen_width_units: [u8; 2],
    screen_height_unites: [u8; 2],
    font_width_units: u8,
    font_height_units: u8,
    routine_offsets: [u8; 2],
    static_string_offsets: [u8; 2],
    default_background_color: u8,
    default_foreground_color: u8,
    addr_terminating_char_table: [u8; 2],
    total_width_pixels: [u8; 2],
    standard_revision: [u8; 2],
    alphabet_table_addr: [u8; 2],
    header_ext_table_addr: [u8; 2],
    unused: [u8; 6]
 }

 impl Header {
    pub fn new() -> Header {
        // defaults for now
        Header::default()
    }
 }

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_size() {
        // 64 is expected size
        assert_eq!(mem::size_of::<Header>(), 64);
    }
}

// Holds story file including: global variables, object tree, current routine and additional
pub struct Memory {
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
        }
    }
}

// Routine last executed
// Not stored in memory
pub struct Stack {

}

impl Stack {
    pub fn new() -> Stack {
        Stack {}
    }
}

// Instruction Processor
// Local variable values
// Temporary storage
//
// Interacts with PC and Memory
pub struct Processor {

}

impl Processor {
    pub fn new() -> Processor {
        Processor {}
    }
}

// Program Counter, interactions with Memory
// Not stored in memory
pub struct PC {

}

impl PC {
    pub fn new() -> PC {
        PC {}
    }
}