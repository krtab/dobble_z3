use z3::ast::Ast;

extern crate z3;

// Number of symbols on card
const S: u32 = 3;
const N: u32 = S * S - S + 1;

const BV_SIZE : u32 = 32;

fn popcnt<'a>(v: &z3::ast::BV<'a>) -> z3::ast::BV<'a> {
    let mut v = v.clone();
    let mut acc = z3::ast::BV::from_i64(v.get_ctx(), 0, BV_SIZE);
    for _ in 0..BV_SIZE {
        acc += &v & z3::ast::BV::from_i64(v.get_ctx(), 1, BV_SIZE);
        v = z3::ast::BV::bvlshr(&v, &z3::ast::BV::from_i64(v.get_ctx(), 1, BV_SIZE));
    }
    acc
}
fn main() {
    // Create Z3 context
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);

    // Create Z3 solver
    let solver = z3::Solver::new(&ctx);

    // Create n variables x[i]
    let x: [_; N as usize] = std::array::from_fn(|i| {
        let var_name = format!("x{}", i);
        z3::ast::BV::new_const(&ctx, var_name, BV_SIZE)
    });

    for x in &x {
        solver.assert(&z3::ast::Ast::_eq(
            &z3::ast::BV::bvlshr(x, &z3::ast::BV::from_i64(&ctx, N as i64, BV_SIZE)),
            &z3::ast::BV::from_u64(&ctx, 0, BV_SIZE),
        ));
        solver.assert(&z3::ast::Ast::_eq(
            &popcnt(x),
            &z3::ast::BV::from_u64(&ctx, S as u64, BV_SIZE),
        ));
    }

    let mut acc = z3::ast::Bool::from_bool(&ctx, true);
    for i in 0..N {
        for j in (i + 1)..N {
            acc &= !z3::ast::Ast::_eq(&x[i as usize], &x[j as usize]);
            let z = z3::ast::BV::bvand(&x[i as usize], &x[j as usize]);
            acc &= !z3::ast::Ast::_eq(&z, &z3::ast::BV::from_i64(&ctx, 0, BV_SIZE));
            let power_of_2 = z3::ast::Ast::_eq(
                &z3::ast::BV::bvand(&(&z - 1_u64), &z),
                &z3::ast::BV::from_i64(&ctx, 0, BV_SIZE),
            );
            acc &= power_of_2;
        }
    }
    solver.assert(&acc);
    // dbg!(solver.get_assertions());
    // Check for satisfiability
    let res = solver.check();
    println!("Stats: {:?}", solver.get_statistics());
    match res {
        z3::SatResult::Unsat => println!("Unsatisfiable"),
        z3::SatResult::Sat => {
            let model = solver.get_model();
            println!("Model: {:?}", model);
        }
        z3::SatResult::Unknown => println!("Unknown"),
    }
}
