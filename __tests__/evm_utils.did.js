export const idlFactory = ({ IDL }) => {
  const List = IDL.Rec();
  const Signature = IDL.Record({
    'r' : IDL.Vec(IDL.Nat8),
    's' : IDL.Vec(IDL.Nat8),
    'v' : IDL.Nat64,
    'from' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'hash' : IDL.Vec(IDL.Nat8),
  });
  const AccessList = IDL.Record({
    'storage_keys' : IDL.Vec(IDL.Vec(IDL.Nat8)),
    'address' : IDL.Vec(IDL.Nat8),
  });
  const Transaction1559 = IDL.Record({
    'to' : IDL.Vec(IDL.Nat8),
    'value' : IDL.Vec(IDL.Nat8),
    'max_priority_fee_per_gas' : IDL.Vec(IDL.Nat8),
    'data' : IDL.Vec(IDL.Nat8),
    'sign' : IDL.Opt(Signature),
    'max_fee_per_gas' : IDL.Vec(IDL.Nat8),
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Vec(IDL.Nat8),
    'gas_limit' : IDL.Vec(IDL.Nat8),
    'access_list' : IDL.Vec(AccessList),
  });
  const Transaction2930 = IDL.Record({
    'to' : IDL.Vec(IDL.Nat8),
    'value' : IDL.Vec(IDL.Nat8),
    'data' : IDL.Vec(IDL.Nat8),
    'sign' : IDL.Opt(Signature),
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Vec(IDL.Nat8),
    'gas_limit' : IDL.Vec(IDL.Nat8),
    'access_list' : IDL.Vec(AccessList),
    'gas_price' : IDL.Vec(IDL.Nat8),
  });
  const TransactionLegacy = IDL.Record({
    'to' : IDL.Vec(IDL.Nat8),
    'value' : IDL.Vec(IDL.Nat8),
    'data' : IDL.Vec(IDL.Nat8),
    'sign' : IDL.Opt(Signature),
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Vec(IDL.Nat8),
    'gas_limit' : IDL.Vec(IDL.Nat8),
    'gas_price' : IDL.Vec(IDL.Nat8),
  });
  const Transaction = IDL.Variant({
    'EIP1559' : Transaction1559,
    'EIP2930' : Transaction2930,
    'Legacy' : TransactionLegacy,
  });
  const Result = IDL.Variant({
    'Ok' : IDL.Tuple(IDL.Vec(IDL.Nat8), IDL.Vec(IDL.Nat8)),
    'Err' : IDL.Text,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({ 'Ok' : Transaction, 'Err' : IDL.Text });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Nat8), 'Err' : IDL.Text });
  const Item = IDL.Variant({
    'Num' : IDL.Nat64,
    'Raw' : IDL.Vec(IDL.Nat8),
    'Empty' : IDL.Null,
    'List' : List,
    'Text' : IDL.Text,
  });
  List.fill(IDL.Record({ 'values' : IDL.Vec(Item) }));
  const Result_4 = IDL.Variant({ 'Ok' : List, 'Err' : IDL.Text });
  const Result_5 = IDL.Variant({
    'Ok' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'Err' : IDL.Text,
  });
  return IDL.Service({
    'create_transaction' : IDL.Func([Transaction], [Result], ['query']),
    'encode_signed_transaction' : IDL.Func([Transaction], [Result], ['query']),
    'is_valid_public' : IDL.Func([IDL.Vec(IDL.Nat8)], [Result_1], ['query']),
    'is_valid_signature' : IDL.Func([IDL.Vec(IDL.Nat8)], [Result_1], ['query']),
    'keccak256' : IDL.Func([IDL.Vec(IDL.Nat8)], [IDL.Vec(IDL.Nat8)], ['query']),
    'parse_transaction' : IDL.Func([IDL.Vec(IDL.Nat8)], [Result_2], ['query']),
    'pub_to_address' : IDL.Func([IDL.Vec(IDL.Nat8)], [Result_3], ['query']),
    'recover_public_key' : IDL.Func(
        [IDL.Vec(IDL.Nat8), IDL.Vec(IDL.Nat8)],
        [Result_3],
        ['query'],
      ),
    'rlp_decode' : IDL.Func([IDL.Vec(IDL.Nat8)], [Result_4], ['query']),
    'rlp_encode' : IDL.Func([List], [Result_3], ['query']),
    'verify_proof' : IDL.Func(
        [IDL.Vec(IDL.Nat8), IDL.Vec(IDL.Nat8), IDL.Vec(IDL.Vec(IDL.Nat8))],
        [Result_5],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
