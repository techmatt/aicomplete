use super::*;
use crate::config::{ModuleConfig, ParamValue};
use crate::modules::board;

#[test]
fn config_diff_names_a_changed_variant() {
    let a = Rules::new(&baseline()).unwrap();
    let b = Rules::new(&Config::from_json(SHARED_ROW).unwrap()).unwrap();
    assert_eq!(
        a.config.diff(&b.config),
        ["threats.variant: countries -> shared_row"]
    );
}

/// `shared_row.json` is derived from the baseline, not tuned on its own: the
/// file must be the baseline with the threat variant and the name changed,
/// parameters and all, so a baseline edit can't leave it behind.
#[test]
fn the_shared_row_config_is_the_baseline_with_the_shared_row_threats() {
    let mut derived = baseline();
    derived.name = "shared_row".into();
    let threats: Vec<&mut ModuleConfig> = derived
        .modules
        .iter_mut()
        .filter(|m| m.module == "threats")
        .collect();
    assert_eq!(threats.len(), 1, "the baseline has one threats module");
    for m in threats {
        m.variant = "shared_row".into();
    }
    assert!(
        derived == Config::from_json(SHARED_ROW).unwrap(),
        "configs/shared_row.json has drifted from configs/baseline.json; regenerate it with \
         `python scripts/configs/shared_row.py`"
    );
}

/// 016: `requires` lists only the modules whose parameters a module reads, so
/// the optional ones can be switched off whole.
#[test]
fn optional_modules_switch_off_whole() {
    for o in ["threats=off", "cards=off", "economy=off"] {
        let mut c = baseline();
        c.apply_override(o).unwrap();
        let r = Rules::new(&c).unwrap_or_else(|e| panic!("{o}: {e}"));
        let mut g = r.new_game(3);
        let mut legal = Vec::new();
        for _ in 0..400 {
            if g.is_over() {
                break;
            }
            r.legal_actions_into(&g, &mut legal);
            let a = legal[0].clone();
            r.apply_mut(&mut g, &a, None).unwrap();
        }
    }
}

#[test]
fn bad_configs_are_rejected_with_a_reason() {
    let cases: &[(&str, &str)] = &[
        ("countries=6", "no map layout for 6 countries"),
        ("countries=5", "no map layout for 5 countries"),
        ("adversary.variant=nope", "unknown variant"),
        ("adversary.dificulty=3", "not a parameter"),
        ("adversary.difficulty=5", "outside"),
        ("board.clock=0,4", "outside"),
        ("threats.per_country=1,2", "expected an integer"),
        ("ai.boards=7", "no AI board 7"),
        ("adversary.stage_cards=3,4", "give 3 counts"),
        ("adversary.stage_cards=9,4,3", "stage 1 has only"),
        ("alignment.unity_ease=5,2", "give tip counts"),
        ("alignment=off", "needs module alignment"),
        ("board=off", "needs module board"),
    ];
    for (o, want) in cases {
        let mut c = baseline();
        c.apply_override(o).unwrap();
        let err = Rules::new(&c)
            .err()
            .unwrap_or_else(|| panic!("{o} was accepted"));
        assert!(err.contains(want), "{o}: {err}");
    }
    fn module<'a>(c: &'a mut Config, id: &str) -> &'a mut ModuleConfig {
        c.modules.iter_mut().find(|m| m.module == id).unwrap()
    }
    let mut c = baseline();
    module(&mut c, "adversary").params.remove("difficulty");
    assert!(Rules::new(&c)
        .err()
        .unwrap()
        .contains("adversary.difficulty: missing"));
    let mut c = baseline();
    module(&mut c, "cards").cards.push("nope".into());
    assert!(Rules::new(&c).err().unwrap().contains("unknown card"));
    let mut c = baseline();
    c.round.push("adversary.nope".into());
    assert!(Rules::new(&c).err().unwrap().contains("no phase"));
    let mut c = baseline();
    c.round.push("board.bases+board.war".into());
    assert!(Rules::new(&c).err().unwrap().contains("appears twice"));
    let mut c = baseline();
    c.modules.push(c.modules[0].clone());
    assert!(Rules::new(&c).err().unwrap().contains("appears twice"));
    assert!(Config::from_json(r#"{"countries": 3, "modules": [], "extra": 1}"#).is_err());
}

#[test]
fn list_parameters_accept_a_single_integer() {
    let rules = Rules::new(&baseline_with(&["ai.boards=1", "board.clock=2"])).unwrap();
    let params = |m: &str| rules.config.module(m).unwrap().params.clone();
    assert_eq!(params("ai")["boards"], ParamValue::List(vec![1]));
    assert_eq!(params("board")["clock"], ParamValue::List(vec![2]));
    // Every seat now plays the Protector, and every round is a war step only.
    let w = rules.world();
    assert_eq!(modules::ai::board(&w, 0).id, "protector");
    assert_eq!(modules::ai::board(&w, 1).id, "protector");
    let mut g = rules.new_game(1);
    g.piles.remove(CURRENT);
    for r in 1..=5 {
        g.round = r;
        assert!(
            board::war_round(&w, &g) && !board::production_round(&w, &g),
            "round {r}"
        );
    }
}

#[test]
fn config_hash_is_stable_and_ignores_the_name() {
    let rules = Rules::new(&baseline()).unwrap();
    // Resolving a resolved config changes nothing.
    let again = Rules::new(&rules.config).unwrap();
    assert_eq!(again.config, rules.config);
    assert_eq!(again.config_hash, rules.config_hash);
    let mut renamed = baseline();
    renamed.name = "other".into();
    assert_eq!(Rules::new(&renamed).unwrap().config_hash, rules.config_hash);
    // An omitted round resolves to module order, then phase order.
    let mut implicit = baseline();
    implicit.round.clear();
    let r = Rules::new(&implicit).unwrap();
    assert_eq!(
        &r.config.round[..3],
        ["board.act", "board.bases", "board.war"]
    );
    let tuned = baseline_with(&["adversary.difficulty=3"]);
    assert_ne!(Rules::new(&tuned).unwrap().config_hash, rules.config_hash);
}

#[test]
fn tunables_list_every_module_parameter() {
    let rules = Rules::new(&baseline()).unwrap();
    let t = rules.tunables();
    let find = |k: &str| {
        t.iter()
            .find(|x| x.key == k)
            .unwrap_or_else(|| panic!("{k} missing"))
    };
    for k in [
        "board.start_corrupted",
        "board.liberate_at",
        "board.clear_cost",
        "adversary.difficulty",
        "adversary.supply_per_country",
        "adversary.corrupt_at",
        "threats.per_country",
        "alignment.tip_per_player",
        "alignment.discord_tip_per_player",
        "cards.keep_tier1",
    ] {
        assert_eq!(find(k).kind, "int", "{k}");
    }
    for k in [
        "board.clock",
        "adversary.stage_cards",
        "alignment.unity_ease",
        "ai.boards",
    ] {
        assert_eq!(find(k).kind, "list", "{k}");
    }
    let clock = find("board.clock");
    assert_eq!((clock.min, clock.max), (0, 3));
    assert_eq!(clock.value, ParamValue::List(vec![1, 2, 1, 2, 1, 3]));
    // One per declared parameter: no duplicates.
    let keys: BTreeSet<&str> = t.iter().map(|x| x.key.as_str()).collect();
    assert_eq!(keys.len(), t.len());
}

/// 020: free standard actions and an Undecided smaller than a tip are
/// rejected (r48: the give-insight off-switch went with the action).
#[test]
fn off_switches_and_degenerate_values() {
    let mut c = baseline();
    assert!(c.apply_override("board.ambassador_credits=0").is_err() || Rules::new(&c).is_err());
    let mut c = baseline();
    c.apply_override("alignment.undecided_per_player=1").unwrap();
    assert!(Rules::new(&c).is_err());
    // An empty adversary deck is rejected; override errors name the key and the modules.
    let mut c = baseline();
    c.apply_override("adversary.stage_cards=0,0,0").unwrap();
    assert!(Rules::new(&c).err().is_some_and(|e| e.contains("stage_cards")));
    let e = baseline().apply_override("board.unit_cap=x").unwrap_err();
    assert!(e.contains("board.unit_cap"), "{e}");
    let e = baseline().apply_override("boardd.unit_cap=3").unwrap_err();
    assert!(e.contains("it has: board"), "{e}");
}

#[test]
fn stage_cards_are_checked_at_what_the_difficulty_deals() {
    let printed = baseline();
    let rules = Rules::new(&printed).unwrap();
    let have = rules.world().content.adversary.iter().filter(|c| c.stage == 3).count() as i64;
    let ok = baseline_with(&[&format!("adversary.stage_cards=4,5,{have}")]);
    assert!(Rules::new(&ok).is_ok(), "difficulty 0 adds no stage-III card");
    let over = baseline_with(&[&format!("adversary.stage_cards=4,5,{have}"), "adversary.difficulty=2"]);
    assert!(Rules::new(&over).is_err(), "difficulty 2 adds one");
    let empty = baseline_with(&["adversary.stage_cards=1,1,0", "adversary.stage1_removed=1", "adversary.stage2_removed=1", "adversary.stage3_added=0", "adversary.difficulty=3"]);
    assert!(Rules::new(&empty).is_err(), "nothing left to deal");
}


/// 022 (D14): a parameter value that is not an integer or a list of them is
/// reported with its key and the value, not serde's untagged-enum message.
#[test]
fn a_bad_parameter_value_names_its_key() {
    let json = BASELINE.replacen("\"unit_cap\": 2", "\"unit_cap\": \"3\"", 1);
    assert_ne!(json, BASELINE);
    let err = Config::from_json(&json).unwrap_err();
    assert!(
        err.contains(r#"board.unit_cap: expected an integer or a list of integers, got "3""#),
        "{err}"
    );
    for (bad, shown) in [("2.5", "2.5"), ("null", "null"), ("[1, \"x\"]", "[1,\"x\"]")] {
        let json = BASELINE.replacen("\"unit_cap\": 2", &format!("\"unit_cap\": {bad}"), 1);
        let err = Config::from_json(&json).unwrap_err();
        let want = format!("board.unit_cap: expected an integer or a list of integers, got {shown}");
        assert!(err.contains(&want), "{err}");
    }
    assert!(Config::from_json(BASELINE).is_ok());
}
