
use crate::loader::elf::Elf64Header;

#[test]
fn test_elf_header_validation() {
    let mut valid_header = [0u8; 64];
    // Set Magic
    valid_header[0] = 0x7F;
    valid_header[1] = b'E';
    valid_header[2] = b'L';
    valid_header[3] = b'F';
    
    assert!(Elf64Header::validate(&valid_header));
    
    let mut invalid_header = [0u8; 64];
    invalid_header[0] = 0x00;
    
    assert!(!Elf64Header::validate(&invalid_header));
}

#[test]
fn test_elf_too_short() {
    let short_data = [0u8; 10];
    assert!(!Elf64Header::validate(&short_data));
}
