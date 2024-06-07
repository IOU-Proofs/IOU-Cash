pragma circom 2.1.5;

include "./node_modules/circomlib/circuits/poseidon.circom";
include "./node_modules/circomlib/circuits/comparators.circom";


template iou(){
   signal input step_in[3];
   signal output step_out[3];

   signal note_id <== step_in[0];
   signal index <== step_in[1];
   signal state_in <== step_in[2];
   signal input prevBlinder;
   signal input inBlinder;
   signal input changeBlinder;
   signal input transferBlinder;
   signal input inputVal;
   signal input outputVal;
   signal input input_index;
   signal input signature;
   signal input nullifierKey;
   signal input signerKey;
   signal input receiver;

   // recover sender
   var identityCommitment = Poseidon(2)([nullifierKey, signerKey]);
   // TODO n_in_pre
   // recover input note
   var input_note = Poseidon(5)([note_id, index,inputVal,identityCommitment,input_index]);
   // recover blinded input note
   var blinded_input_node = Poseidon(2)([input_note, inBlinder]);
   // TODO add dir
   var state_in_recovery = Poseidon(2)([blinded_input_node, prevBlinder]);
   component IsEqual = IsEqual();
   IsEqual.in <== [state_in, state_in_recovery];
   // recover nullifier
   var rec_nullifier = Poseidon(2)([identityCommitment, blinded_input_node]);
   // recover change
   var note_change = Poseidon(6)([note_id, index + 1, input_note, inputVal - outputVal, identityCommitment, 0]);
   // blind transfer
   var blinded_change = Poseidon(2)([note_change, changeBlinder]);
   // recover transfer
   var note_transfer = Poseidon(6)([note_id, index + 1, input_note, outputVal, receiver, 1]);
   // blind transfer
   var blinded_transfer = Poseidon(2)([note_transfer, transferBlinder]);
   // recover transition
   var recover_trans = Poseidon(3)([state_in, blinded_change, blinded_transfer]);
   // Check zero sum
   // TODO check 251 bits 
   component GreaterEqThan = GreaterEqThan(251);
   GreaterEqThan.in <== [inputVal, outputVal];
   GreaterEqThan.out === 1;
   // TODO Check signature


}

component main { public [step_in] } = iou();