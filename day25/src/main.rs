fn find_loop_size(subj: i64, pk: i64, modulus: i64) -> i64 {
    let mut val = 1;
    let mut counter = 0;
    while val != pk {
        counter += 1;
        val *= subj;
        val %= modulus;
    }
    counter
}

fn transform(subj: i64, times: i64, modulus: i64) -> i64 {
    let mut val = 1;
    for _ in 0..times {
        val *= subj;
        val %= modulus;
    }
    val
}

fn solve_part_1(card_pk: i64, door_pk: i64) -> i64 {
    let subj = 7;
    let modulus = 20201227;
    let loop_size = find_loop_size(subj, card_pk, modulus);
    transform(door_pk, loop_size, modulus)
}

fn main() {
    //let (card_pk, door_pk) = (5764801, 17807724);
    let (card_pk, door_pk) = (1717001, 523731);
    let result = solve_part_1(card_pk, door_pk);
    println!("Result of part 1 is: {}", result);
}
