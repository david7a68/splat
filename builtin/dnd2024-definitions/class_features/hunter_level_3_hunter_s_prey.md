---
splat_schema: 1
splat_id: ecc4959e-d0cc-55eb-a878-b0b56181e2e9
splat_kind: rule
splat_key: dnd2024:class_features/hunter_level_3_hunter_s_prey
splat_name: 'Level 3: Hunter’s Prey'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/hunter_level_3_hunter_s_prey
  parameters:
    choose: 1
    replacement:
      events:
      - rest.short_completed
      - rest.long_completed
      choose_other_option: true
    options:
      colossus_slayer:
        trigger: actor.weapon_hits_creature
        requires:
          target_missing_hit_points: true
        extra_damage:
          dice:
            count: 1
            sides: 8
          type: weapon.damage_type
        limit:
          count: 1
          period: turn
      horde_breaker:
        trigger: actor.weapon_attack
        limit:
          count: 1
          period: actor.turn
        extra_attack:
          weapon: same
          target:
            type: creature
            different_from_original: true
            within_original_target_feet: 5
            within_weapon_range: true
            not_attacked_by_actor_this_turn: true
    class_scope:
      class: ranger
      minimum_level: 3
  implementation: dnd2024:class_features_hunter_level_3_hunter_s_prey
splat_contracts:
- id: dnd2024:class_features_hunter_level_3_hunter_s_prey
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - rest.short_completed
  - rest.long_completed
  - attack.hit
  - attack.roll
  capabilities:
  - character_features
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 61
  engine_gaps: &id001
  - dnd2024:class_features_hunter_level_3_hunter_s_prey
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: eed344169a27adfbbdb3991c09dc272b8a4e53063a81081265f146da91a03ff8
  parent_name: Hunter
  line_span:
    first: 5090
    count: 17
splat_clauses:
- key: source_rules
  text: '# Level 3: Hunter’s Prey You gain one of the following feature options of your choice. Whenever
    you finish a Short or Long Rest, you can replace the chosen option with the other one. Colossus Slayer.
    Your tenacity can wear down even the most resilient foes. When you hit a creature with a weapon, the
    weapon deals an extra 1d8 damage to the target if it’s missing any of its Hit Points. You can deal
    this extra damage only once per turn. Horde Breaker. Once on each of your turns when you make an attack
    with a weapon, you can make another attack with the same weapon against a different creature that
    is within 5 feet of the original target, that is within the weapon’s range, and that you haven’t attacked
    this turn.'
  sha256: 336947ba39ac729a4c9b2303210baf3cbec6416bf2bc5c102e2eb5a29bed9058
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Hunter’s Prey

<!-- SRD page 61 -->

You gain one of the following feature options of your\
choice. Whenever you finish a Short or Long Rest,\
you can replace the chosen option with the other\
one.\
Colossus Slayer. Your tenacity can wear down\
even the most resilient foes. When you hit a crea-\
ture with a weapon, the weapon deals an extra 1d8\
damage to the target if it’s missing any of its Hit\
Points. You can deal this extra damage only once\
per turn.\
Horde Breaker. Once on each of your turns when\
you make an attack with a weapon, you can make\
another attack with the same weapon against a dif-\
ferent creature that is within 5 feet of the original\
target, that is within the weapon’s range, and that\
you haven’t attacked this turn.\
