---
splat_schema: 1
splat_id: 7b749381-128f-5563-8b8c-e68513aaa2f3
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_hit_points
splat_name: Hit Points
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_hit_points
  parameters:
    apply_definitions:
    - key: dnd2024:glossary/short_rest
    - key: dnd2024:glossary/long_rest
    hit_points:
      minimum: 0
      maximum: hit_point_maximum
      damage_operation: subtract
      loss_above_zero_impairs_capabilities: false
    bloodied:
      current_hit_points_lte_fraction_maximum: 0.5
      inherent_effect: false
    rest_duration_hours:
      short: 1
      long: 8
  implementation: dnd2024:rule_damage_and_healing_hit_points
splat_contracts:
- id: dnd2024:rule_damage_and_healing_hit_points
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - resolution
  events: []
  capabilities:
  - hit_points
  - rests
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 16
  engine_gaps: &id001
  - dnd2024:rule_damage_and_healing_hit_points
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 79bcf714d98a2023b55e1717e8e8de7c80c9cd8d0402c2ebf7762b8b9b4103b6
  parent_name: Damage and Healing
  line_span:
    first: 1107
    count: 20
splat_clauses:
- key: source_rules
  text: '# Hit Points Hit Points represent durability and the will to live. Creatures with more Hit Points
    are more difficult to kill. Your Hit Point maximum is the number of rEstinG Adventurers can’t spend
    every hour adventuring. They need rest. Any creature can take hour-long Short Rests in the midst of
    a day and an 8-hour Long Rest to end it. Regaining Hit Points is one of the main benefits of a rest.
    “Rules Glossary” provides the rules for Short and Long Rests. Hit Points you have when uninjured.
    Your current Hit Points can be any number from that maximum down to 0, which is the lowest Hit Points
    can go. Whenever you take damage, subtract it from your Hit Points. Hit Point loss has no effect on
    your capabilities until you reach 0 Hit Points. If you have half your Hit Points or fewer, you’re
    Bloodied, which has no game effect on its own but which might trigger other game effects.'
  sha256: 3dc912a6da7729ff8decaf85219c2836217dedeb3cfda93c7b338403bf661861
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hit Points

<!-- SRD page 16 -->

Hit Points represent durability and the will to live.\
Creatures with more Hit Points are more difficult\
to kill. Your Hit Point maximum is the number of\
rEstinG\
Adventurers can’t spend every hour adventuring.\
They need rest. Any creature can take hour-long\
Short Rests in the midst of a day and an 8-hour Long\
Rest to end it. Regaining Hit Points is one of the main\
benefits of a rest. “Rules Glossary” provides the\
rules for Short and Long Rests.\
Hit Points you have when uninjured. Your current\
Hit Points can be any number from that maximum\
down to 0, which is the lowest Hit Points can go.\
Whenever you take damage, subtract it from your\
Hit Points. Hit Point loss has no effect on your capa-\
bilities until you reach 0 Hit Points.\
If you have half your Hit Points or fewer, you’re\
Bloodied, which has no game effect on its own but\
which might trigger other game effects.\
