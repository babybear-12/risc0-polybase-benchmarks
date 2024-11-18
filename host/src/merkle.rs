use methods::MERKLE_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt, SessionStats};
use shared::{hash::Sha, Tree};

pub fn merkle(tree1: Tree<Sha>, tree2: Tree<Sha>) -> impl FnMut() -> (Receipt, SessionStats) {
    let env = ExecutorEnv::builder()
        .write(&tree1)
        .expect("Failed to write tree1")
        .write(&tree2)
        .expect("Failed to write tree2")
        .build()
        .expect("Failed to build env");

    let prover = default_prover();
    let prove_info = prover.prove(env, MERKLE_ELF).expect("Proving failed");

    move || {
        let session = &prove_info.stats;
        let receipt = &prove_info.receipt;
        let session_stats = SessionStats {
            segments: session.segments,
            total_cycles: session.total_cycles,
            user_cycles: session.user_cycles,
        };
        (receipt.clone(), session_stats)
    }
}

// pub fn merkle_membership(path_size: usize) -> impl FnMut() -> (Receipt, Session) {
//     let path = core::iter::from_fn(|| Some(Sha::random()))
//         .take(path_size + 1)
//         .flat_map(|sha| sha.as_bytes().to_vec())
//         .collect::<Vec<_>>();

//     let env = ExecutorEnv::builder()
//         .add_input(&to_vec(&path).unwrap())
//         .build()
//         .unwrap();

//     let mut exec = Executor::from_elf(env, MERKLE_MEMBERSHIP_ELF).unwrap();

//     move || {
//         let session = exec.run().unwrap();
//         let receipt = session.prove().unwrap();

//         (receipt, session)
//     }
// }
