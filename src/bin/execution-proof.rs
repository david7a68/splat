//! Run a small SRD encounter from the executable Markdown definitions.
use serde_json::json;
use splat::execution::{ENGINE_ID, Engine, TargetFacts, Use, builtin, dice::Random};
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.iter().any(|s| s == "--help") {
        println!(
            "Usage: execution-proof [SEED]\nRuns Black Bear versus two Skeletons with declared reach and clear attack facts. Prints in-memory results and trace as JSON."
        );
        return Ok(());
    }
    if args.len() > 1 {
        return Err("Expected one optional integer seed".into());
    }
    let seed = args
        .first()
        .map(|s| {
            s.parse::<u64>()
                .map_err(|_| "Seed must be an unsigned integer")
        })
        .transpose()?
        .unwrap_or(42);
    let entries = builtin::entries()?;
    let mut e = Engine::new(&entries, Random::seeded(seed))?;
    e.add_actor("bear", "dnd2024:creatures/black_bear")?;
    e.add_actor("skeleton_a", "dnd2024:creatures/skeleton")?;
    e.add_actor("skeleton_b", "dnd2024:creatures/skeleton")?;
    // Identical skeletons share initiative. The controller supplies stable order.
    let mut init = Random::seeded(seed ^ 0x7363686564756c65);
    let bear_init = init.die(20)? as i64 + 1;
    let skeleton_init = init.die(20)? as i64 + 3;
    let order = if bear_init > skeleton_init {
        vec!["bear", "skeleton_a", "skeleton_b"]
    } else {
        vec!["skeleton_a", "skeleton_b", "bear"]
    };
    let mut rounds = 0;
    while rounds < 20
        && e.actors["bear"].hp > 0
        && (e.actors["skeleton_a"].hp > 0 || e.actors["skeleton_b"].hp > 0)
    {
        rounds += 1;
        for actor in &order {
            if e.actors[*actor].hp == 0 {
                continue;
            }
            let target = if *actor == "bear" {
                if e.actors["skeleton_a"].hp > 0 {
                    "skeleton_a"
                } else {
                    "skeleton_b"
                }
            } else {
                "bear"
            };
            if e.actors[target].hp == 0 {
                break;
            }
            e.start_turn(actor)?;
            let definition = e.actors[*actor].definition.clone();
            let mut request = Use::new(
                actor,
                &definition,
                if *actor == "bear" {
                    "multiattack"
                } else {
                    "shortsword"
                },
                &[target],
            );
            request.facts.insert(target.into(), TargetFacts::clear());
            e.use_action(request)?;
        }
    }
    let outcome = if e.actors["bear"].hp == 0 {
        "skeletons_win"
    } else if e.actors["skeleton_a"].hp == 0 && e.actors["skeleton_b"].hp == 0 {
        "bear_wins"
    } else {
        "round_limit"
    };
    let hp: std::collections::BTreeMap<_, _> = e
        .actors
        .iter()
        .map(|(key, actor)| (key, actor.hp))
        .collect();
    println!("{}",serde_json::to_string_pretty(&json!({"engine":ENGINE_ID,"rng":Random::ID,"seed":seed,"rounds":rounds,"outcome":outcome,"hp":hp,"initiative":{"bear":bear_init,"skeleton_group":skeleton_init,"order":order},"assumptions":["Every chosen melee target is within reach with a clear path.","No other Advantage, Disadvantage, cover, movement, reactions or active effects.","Monster zero-HP policy; fixed-target Multiattack strategy may forgo a remaining attack if its target dies."],"events":e.events})).map_err(|e|e.to_string())?);
    Ok(())
}
