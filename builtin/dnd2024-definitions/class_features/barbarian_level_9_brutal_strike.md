---
splat_schema: 1
splat_id: afb1b1fd-06d5-5f96-9c0e-2abbb7260a86
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_9_brutal_strike
splat_name: 'Level 9: Brutal Strike'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29, 30
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_9_brutal_strike
  parameters:
    requires:
      reckless_attack_used: true
      attack_ability: strength
      attack_disadvantage: false
      turn: actor
    limit:
      count: 1
      period: actor.turn
    cost:
      forgo_all_advantage: true
    on_hit:
      extra_damage:
        dice:
          count: 1
          sides: 10
        type: attack.weapon_or_unarmed_damage_type
      choose_effects: 1
      options:
        forceful_blow:
          push:
            feet: 15
            direction: straight_away_from_actor
          follow_movement:
            maximum_feet:
              op: multiply
              args:
              - var: actor.speed
              - 0.5
            direction: straight_toward_target
            opportunity_attacks: false
        hamstring_blow:
          speed_bonus_feet: -15
          end: actor.next_turn.start
          stacking: latest_only
    class_scope:
      class: barbarian
      minimum_level: 9
  implementation: dnd2024:class_features_barbarian_level_9_brutal_strike
splat_contracts:
- id: dnd2024:class_features_barbarian_level_9_brutal_strike
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
  - attack.roll_pending
  - attack.hit
  - turn.start
  capabilities:
  - character_features
  - damage
  - d20_tests
  - movement
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 29
  - 30
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_9_brutal_strike
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 642652e11e6f3afaf2996fa8c31ee8ba94fa122246f0d8397fd287fbd18b3199
  parent_name: Barbarian
  line_span:
    first: 2386
    count: 17
splat_clauses:
- key: source_rules
  text: '# Level 9: Brutal Strike If you use Reckless Attack, you can forgo any Advantage on one Strength-based
    attack roll of your choice on your turn. The chosen attack roll mustn’t have Disadvantage. If the
    chosen attack roll hits, the target takes an extra 1d10 damage of the same type dealt by the weapon
    or Unarmed Strike, and you can cause one Brutal Strike effect of your choice. You have the following
    effect options. Forceful Blow. The target is pushed 15 feet straight away from you. You can then move
    up to half your Speed straight toward the target without provoking Opportunity Attacks. Hamstring
    Blow. The target’s Speed is reduced by 15 feet until the start of your next turn. A target can be
    affected by only one Hamstring Blow at a time— the most recent one.'
  sha256: 21ca551a58bd362f7dea0fe347f6548f2b030fd537bd03c840c50b0994184e01
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 9: Brutal Strike

<!-- SRD page 29 -->

If you use Reckless Attack, you can forgo any Ad-\
vantage on one Strength-based attack roll of your\
choice on your turn. The chosen attack roll mustn’t\
have Disadvantage. If the chosen attack roll hits, the\
target takes an extra 1d10 damage of the same type\
dealt by the weapon or Unarmed Strike, and you can\
cause one Brutal Strike effect of your choice. You\
have the following effect options.\
Forceful Blow. The target is pushed 15 feet\
straight away from you. You can then move up to\
half your Speed straight toward the target without\
provoking Opportunity Attacks.\

<!-- SRD page 30 -->

Hamstring Blow. The target’s Speed is reduced by\
15 feet until the start of your next turn. A target can\
be affected by only one Hamstring Blow at a time—\
the most recent one.\
