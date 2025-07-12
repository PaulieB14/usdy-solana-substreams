#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_determine_instruction_type() {
        // Test transfer instruction (discriminator = 3)
        let transfer_data = vec![3, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(determine_instruction_type(&transfer_data), "transfer");
        
        // Test mint instruction (discriminator = 7)
        let mint_data = vec![7, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(determine_instruction_type(&mint_data), "mint_to");
        
        // Test burn instruction (discriminator = 8)
        let burn_data = vec![8, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(determine_instruction_type(&burn_data), "burn");
        
        // Test unknown instruction
        let unknown_data = vec![99, 0, 0, 0];
        assert_eq!(determine_instruction_type(&unknown_data), "unknown");
        
        // Test empty data
        assert_eq!(determine_instruction_type(&[]), "unknown");
    }
    
    #[test]
    fn test_extract_amount_from_data() {
        // Test with valid transfer data (amount = 1000000 = 0x000F4240 in little endian)
        let data_hex = "030040420F00000000";
        let amount = extract_amount_from_data(data_hex);
        assert_eq!(amount, "1000000");
        
        // Test with empty data
        assert_eq!(extract_amount_from_data(""), "0");
        
        // Test with invalid hex
        assert_eq!(extract_amount_from_data("invalid"), "0");
    }
    
    #[test]
    fn test_usdy_mint_constant() {
        assert_eq!(USDY_MINT, "A1KLoBrKBde8Ty9qtNQUtq3C2ortoC3u7twggz7sEto6");
    }
}
