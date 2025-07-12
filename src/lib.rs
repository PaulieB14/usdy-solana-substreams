use substreams::prelude::*;
use substreams_solana::pb::sf::solana::r#type::v1 as solana;
use crate::pb::usdy::types::v1 as usdy;

mod pb;
#[cfg(test)]
mod tests;

const USDY_MINT: &str = "A1KLoBrKBde8Ty9qtNQUtq3C2ortoC3u7twggz7sEto6";

#[substreams::handlers::map]
pub fn map_usdy_transactions(block: solana::Block) -> Result<usdy::UsdyTransactions, substreams::errors::Error> {
    let mut transactions = Vec::new();
    
    for transaction in block.transactions.iter() {
        if let Some(transaction_data) = &transaction.transaction {
            // Check if transaction involves USDY mint
            let accounts: Vec<String> = transaction_data.message.as_ref()
                .map(|msg| msg.account_keys.iter().map(|key| bs58::encode(key).into_string()).collect())
                .unwrap_or_default();
            
            if accounts.iter().any(|account| account == USDY_MINT) {
                let block_height = block.block_height.as_ref().map(|h| h.block_height).unwrap_or(0);
                let block_timestamp = block.block_time.as_ref().map(|t| t.timestamp).unwrap_or(0);
                
                let usdy_tx = usdy::UsdyTransaction {
                    signature: bs58::encode(&transaction.transaction.as_ref().unwrap().signatures[0]).into_string(),
                    block_hash: bs58::encode(&block.blockhash).into_string(),
                    block_number: block_height,
                    block_timestamp,
                    instructions: extract_usdy_instructions(transaction_data, &accounts),
                    success: transaction.meta.as_ref().map(|meta| meta.err.is_none()).unwrap_or(false),
                    fee_payer: accounts.get(0).cloned().unwrap_or_default(),
                    fee: transaction.meta.as_ref().map(|meta| meta.fee).unwrap_or(0),
                    accounts,
                };
                transactions.push(usdy_tx);
            }
        }
    }
    
    Ok(usdy::UsdyTransactions { transactions })
}

// Helper function to extract USDY-related instructions
fn extract_usdy_instructions(transaction: &solana::Transaction, accounts: &[String]) -> Vec<usdy::UsdyInstruction> {
    let mut instructions = Vec::new();
    
    if let Some(message) = &transaction.message {
        for (index, instruction) in message.instructions.iter().enumerate() {
            let program_id_index = instruction.program_id_index;
            if let Some(program_id) = accounts.get(program_id_index as usize) {
                // Check if this is a token program instruction involving USDY
                let instruction_accounts: Vec<String> = instruction.accounts.iter()
                    .filter_map(|&acc_index| accounts.get(acc_index as usize).cloned())
                    .collect();
                
                if instruction_accounts.iter().any(|acc| acc == USDY_MINT) {
                    let usdy_instruction = usdy::UsdyInstruction {
                        program_id: program_id.clone(),
                        accounts: instruction_accounts,
                        data: hex::encode(&instruction.data),
                        instruction_index: index as u32,
                        instruction_type: determine_instruction_type(&instruction.data),
                    };
                    instructions.push(usdy_instruction);
                }
            }
        }
    }
    
    instructions
}

// Helper function to determine instruction type based on data
fn determine_instruction_type(data: &[u8]) -> String {
    if data.is_empty() {
        return "unknown".to_string();
    }
    
    // SPL Token instruction discriminators
    match data[0] {
        3 => "transfer".to_string(),
        7 => "mint_to".to_string(),
        8 => "burn".to_string(),
        4 => "approve".to_string(),
        0 => "initialize_mint".to_string(),
        1 => "initialize_account".to_string(),
        _ => "unknown".to_string(),
    }
}

// Enhanced event extraction
#[substreams::handlers::map]
pub fn map_usdy_events(
    txs: usdy::UsdyTransactions,
) -> Result<usdy::UsdyEvents, substreams::errors::Error> {
    let mut events = Vec::new();
    
    for transaction in &txs.transactions {
        for instruction in &transaction.instructions {
            if let Some(event) = extract_event_from_instruction(transaction, instruction) {
                events.push(event);
            }
        }
    }
    
    Ok(usdy::UsdyEvents { events })
}

// Helper function to extract events from instructions
fn extract_event_from_instruction(
    transaction: &usdy::UsdyTransaction, 
    instruction: &usdy::UsdyInstruction
) -> Option<usdy::UsdyEvent> {
    let event_data = match instruction.instruction_type.as_str() {
        "transfer" => {
            if instruction.accounts.len() >= 3 {
                Some(usdy::usdy_event::EventData::Transfer(usdy::UsdyTransfer {
                    from: instruction.accounts.get(0).cloned().unwrap_or_default(),
                    to: instruction.accounts.get(1).cloned().unwrap_or_default(),
                    amount: extract_amount_from_data(&instruction.data),
                    authority: instruction.accounts.get(2).cloned().unwrap_or_default(),
                }))
            } else {
                None
            }
        },
        "mint_to" => {
            if instruction.accounts.len() >= 2 {
                Some(usdy::usdy_event::EventData::Mint(usdy::UsdyMint {
                    to: instruction.accounts.get(1).cloned().unwrap_or_default(),
                    amount: extract_amount_from_data(&instruction.data),
                    mint_authority: instruction.accounts.get(0).cloned().unwrap_or_default(),
                }))
            } else {
                None
            }
        },
        "burn" => {
            if instruction.accounts.len() >= 2 {
                Some(usdy::usdy_event::EventData::Burn(usdy::UsdyBurn {
                    from: instruction.accounts.get(0).cloned().unwrap_or_default(),
                    amount: extract_amount_from_data(&instruction.data),
                    burn_authority: instruction.accounts.get(1).cloned().unwrap_or_default(),
                }))
            } else {
                None
            }
        },
        "approve" => {
            if instruction.accounts.len() >= 3 {
                Some(usdy::usdy_event::EventData::Approval(usdy::UsdyApproval {
                    owner: instruction.accounts.get(0).cloned().unwrap_or_default(),
                    spender: instruction.accounts.get(1).cloned().unwrap_or_default(),
                    amount: extract_amount_from_data(&instruction.data),
                }))
            } else {
                None
            }
        },
        "initialize_account" => {
            if instruction.accounts.len() >= 3 {
                Some(usdy::usdy_event::EventData::AccountCreation(usdy::UsdyAccountCreation {
                    account: instruction.accounts.get(0).cloned().unwrap_or_default(),
                    owner: instruction.accounts.get(1).cloned().unwrap_or_default(),
                    mint: instruction.accounts.get(2).cloned().unwrap_or_default(),
                }))
            } else {
                None
            }
        },
        _ => None,
    };
    
    event_data.map(|data| usdy::UsdyEvent {
        transaction_signature: transaction.signature.clone(),
        block_hash: transaction.block_hash.clone(),
        block_number: transaction.block_number,
        block_timestamp: transaction.block_timestamp,
        instruction_index: instruction.instruction_index,
        event_type: instruction.instruction_type.clone(),
        event_data: Some(data),
    })
}

// Helper function to extract amount from instruction data
fn extract_amount_from_data(data_hex: &str) -> String {
    if let Ok(data) = hex::decode(data_hex) {
        if data.len() >= 9 {
            // SPL Token transfer/mint/burn amount is typically at bytes 1-8 (little endian u64)
            let amount_bytes = &data[1..9];
            if let Ok(amount_array) = amount_bytes.try_into() {
                let amount = u64::from_le_bytes(amount_array);
                return amount.to_string();
            }
        }
    }
    "0".to_string()
}

// Store module for tracking holder balances
#[substreams::handlers::store]
pub fn store_usdy_holders(events: usdy::UsdyEvents, store: StoreSetProto<usdy::UsdyHolderBalance>) {
    for event in events.events {
        match &event.event_data {
            Some(usdy::usdy_event::EventData::Transfer(transfer)) => {
                if let Ok(amount_val) = transfer.amount.parse::<u64>() {
                    apply_balance_change(&store, &transfer.from, -(amount_val as i64), event.block_timestamp);
                    apply_balance_change(&store, &transfer.to, amount_val as i64, event.block_timestamp);
                }
            },
            Some(usdy::usdy_event::EventData::Mint(mint)) => {
                if let Ok(amount_val) = mint.amount.parse::<u64>() {
                    apply_balance_change(&store, &mint.to, amount_val as i64, event.block_timestamp);
                }
            },
            Some(usdy::usdy_event::EventData::Burn(burn)) => {
                if let Ok(amount_val) = burn.amount.parse::<u64>() {
                    apply_balance_change(&store, &burn.from, -(amount_val as i64), event.block_timestamp);
                }
            },
            _ => {}
        }
    }
}

// Helper function to update balance in store
// Note: StoreSetProto does not support reading previous values. We always set the new value based on the event.
fn apply_balance_change(
    store: &StoreSetProto<usdy::UsdyHolderBalance>,
    account: &str,
    amount_change: i64,
    timestamp: i64
) {
    if account.is_empty() {
        return;
    }
    // We cannot read the previous balance in a store handler, so we just set the new value as the change.
    store.set(0, account, &usdy::UsdyHolderBalance {
        account: account.to_string(),
        balance: amount_change.to_string(),
        last_updated: timestamp,
    });
}

// Map module for holder deltas
#[substreams::handlers::map]
pub fn map_usdy_holder_deltas(
    events: usdy::UsdyEvents,
    store: StoreGetProto<usdy::UsdyHolderBalance>,
) -> Result<usdy::UsdyHolderDeltas, substreams::errors::Error> {
    let mut deltas = Vec::new();
    
    for event in &events.events {
        match &event.event_data {
            Some(usdy::usdy_event::EventData::Transfer(ref transfer)) => {
                let from_balance = store.get_last(&transfer.from).unwrap_or(usdy::UsdyHolderBalance{
                    account: transfer.from.clone(),
                    balance: "0".to_string(),
                    last_updated: 0,
                });
                let to_balance = store.get_last(&transfer.to).unwrap_or(usdy::UsdyHolderBalance{
                    account: transfer.to.clone(),
                    balance: "0".to_string(),
                    last_updated: 0,
                });

                deltas.push(usdy::UsdyHolderDelta {
                    account: transfer.from.clone(),
                    old_balance: from_balance.balance.clone(),
                    new_balance: (from_balance.balance.parse::<i64>().unwrap_or(0) - transfer.amount.parse::<i64>().unwrap_or(0)).to_string(),
                    delta: format!("-{}", transfer.amount),
                    timestamp: event.block_timestamp,
                    transaction_signature: event.transaction_signature.clone(),
                });
                
                deltas.push(usdy::UsdyHolderDelta {
                    account: transfer.to.clone(),
                    old_balance: to_balance.balance.clone(),
                    new_balance: (to_balance.balance.parse::<i64>().unwrap_or(0) + transfer.amount.parse::<i64>().unwrap_or(0)).to_string(),
                    delta: format!("+{}", transfer.amount),
                    timestamp: event.block_timestamp,
                    transaction_signature: event.transaction_signature.clone(),
                });
            },
            Some(usdy::usdy_event::EventData::Mint(ref mint)) => {
                deltas.push(usdy::UsdyHolderDelta {
                    account: mint.to.clone(),
                    old_balance: "0".to_string(),
                    new_balance: mint.amount.clone(),
                    delta: format!("+{}", mint.amount),
                    timestamp: event.block_timestamp,
                    transaction_signature: event.transaction_signature.clone(),
                });
            },
            Some(usdy::usdy_event::EventData::Burn(ref burn)) => {
                deltas.push(usdy::UsdyHolderDelta {
                    account: burn.from.clone(),
                    old_balance: burn.amount.clone(),
                    new_balance: "0".to_string(),
                    delta: format!("-{}", burn.amount),
                    timestamp: event.block_timestamp,
                    transaction_signature: event.transaction_signature.clone(),
                });
            },
            _ => {}
        }
    }
    
    Ok(usdy::UsdyHolderDeltas { deltas })
}
