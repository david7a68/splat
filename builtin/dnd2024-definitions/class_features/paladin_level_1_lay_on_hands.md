---
splat_schema: 1
splat_id: 0304382e-8fd5-501d-b7c9-a25fa0d8e15f
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_1_lay_on_hands
splat_name: 'Level 1: Lay On Hands'
splat_origin:
  key: dnd2024:reference/pages/053
splat_sources:
- title: SRD 5.2.1
  locator: pp. 53, 54
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_1_lay_on_hands
  parameters:
    resource:
      id: lay_on_hands_pool
      maximum:
        op: multiply
        args:
        - 5
        - stat: classes.paladin.level
      recover:
        event: rest.long_rest_completed
        amount: all_expended
    action: bonus_action
    target:
      type: creature
      range: touch
      self_allowed: true
    healing:
      chosen_points:
        type: integer
        minimum: 0
        maximum: remaining_pool
      hit_points_per_pool_point: 1
    remove_condition:
      condition: poisoned
      pool_cost: 5
      cost_points_also_heal: false
  implementation: dnd2024:class_features_paladin_level_1_lay_on_hands
splat_contracts:
- id: dnd2024:class_features_paladin_level_1_lay_on_hands
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - resource_changes
  - healing
  - conditions
  events:
  - action.bonus
  - rest.long_rest_completed
  capabilities:
  - healing
  - conditions
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 53
  - 54
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_1_lay_on_hands
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 90b09fe35cd79ca9ccd165abf1b5dded75a11c74eae92a1f5bde743607bc9774
  parent_name: Paladin
  line_span:
    first: 4456
    count: 37
splat_clauses:
- key: source_rules
  text: '# Level 1: Lay On Hands Your blessed touch can heal wounds. You have a pool of healing power
    that replenishes when you finish a Long Rest. With that pool, you can restore a total number of Hit
    Points equal to five times your Paladin level. Channel Prepared —Spell Slots per Spell Level— Divinity
    Spells 1 2 3 4 5 — 2 2 — — — — — 3 2 — — — — 2 4 3 — — — — 2 5 3 — — — — 2 6 4 2 — — — 2 6 4 2 — —
    — 2 7 4 3 — — — 2 7 4 3 — — — 2 9 4 3 2 — — 2 9 4 3 2 — — 3 10 4 3 3 — — 3 10 4 3 3 — — 3 11 4 3 3
    1 — 3 11 4 3 3 1 — 3 12 4 3 3 2 — 3 12 4 3 3 2 — 3 14 4 3 3 3 1 3 14 4 3 3 3 1 3 15 4 3 3 3 2 3 15
    4 3 3 3 2 As a Bonus Action, you can touch a creature (which could be yourself) and draw power from
    the pool of healing to restore a number of Hit Points to that creature, up to the maximum amount remaining
    in the pool. You can also expend 5 Hit Points from the pool of healing power to remove the Poisoned
    condition from the creature; those points don’t also restore Hit Points to the creature.'
  sha256: b62bcee42b4743cdbf8b255508b44959bc4b00edb8c02ae58cdbc2b72a8173c8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Lay On Hands

<!-- SRD page 53 -->

Your blessed touch can heal wounds. You have a\
pool of healing power that replenishes when you\
finish a Long Rest. With that pool, you can restore a\
total number of Hit Points equal to five times your\
Paladin level.\
Channel Prepared —Spell Slots per Spell Level—\
Divinity Spells 1 2 3 4 5\
— 2 2 — — — —\
— 3 2 — — — —\
2 4 3 — — — —\
2 5 3 — — — —\
2 6 4 2 — — —\
2 6 4 2 — — —\
2 7 4 3 — — —\
2 7 4 3 — — —\
2 9 4 3 2 — —\
2 9 4 3 2 — —\
3 10 4 3 3 — —\
3 10 4 3 3 — —\
3 11 4 3 3 1 —\
3 11 4 3 3 1 —\
3 12 4 3 3 2 —\
3 12 4 3 3 2 —\
3 14 4 3 3 3 1\
3 14 4 3 3 3 1\
3 15 4 3 3 3 2\
3 15 4 3 3 3 2\

<!-- SRD page 54 -->

As a Bonus Action, you can touch a creature\
(which could be yourself) and draw power from the\
pool of healing to restore a number of Hit Points to\
that creature, up to the maximum amount remain-\
ing in the pool.\
You can also expend 5 Hit Points from the pool of\
healing power to remove the Poisoned condition\
from the creature; those points don’t also restore\
Hit Points to the creature.\
