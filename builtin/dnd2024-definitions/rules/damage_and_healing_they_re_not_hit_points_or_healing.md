---
splat_schema: 1
splat_id: bf9388da-b669-57ab-badc-bd78eb90219c
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_they_re_not_hit_points_or_healing
splat_name: They’re Not Hit Points or Healing
splat_origin:
  key: dnd2024:reference/pages/018
splat_sources:
- title: SRD 5.2.1
  locator: pp. 18
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_they_re_not_hit_points_or_healing
  parameters:
    temporary_hp:
      add_to_hp: false
      restored_by_healing: false
      counts_as_healing: false
      allowed_at_full_hp: true
      restores_consciousness_at_zero_hp: false
  implementation: dnd2024:rules_damage_and_healing_they_re_not_hit_points_or_healing
splat_contracts:
- id: dnd2024:rules_damage_and_healing_they_re_not_hit_points_or_healing
  inputs:
    target:
      type: participant
      required: true
    damage:
      required: false
      type: resolution
    event:
      type: event
      required: true
  outputs:
  - hit_point_updates
  - condition_updates
  - death_save_updates
  events: []
  capabilities:
  - damage
  - healing
  - death_saves
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 18
  engine_gaps: &id001
  - dnd2024:rules_damage_and_healing_they_re_not_hit_points_or_healing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 0caddda7c2b3ec5cbe43732a545861a7d4cec400e67393a5c9fcf36fa726829d
  parent_name: Damage and Healing
  line_span:
    first: 1308
    count: 10
splat_clauses:
- key: source_rules
  text: '# They’re Not Hit Points or Healing Temporary Hit Points can’t be added to your Hit Points, healing
    can’t restore them, and receiving Temporary Hit Points doesn’t count as healing. Because Temporary
    Hit Points aren’t Hit Points, a creature can be at full Hit Points and receive Temporary Hit Points.
    If you have 0 Hit Points, receiving Temporary Hit Points doesn’t restore you to consciousness. Only
    true healing can save you.'
  sha256: c69445c2cf6c0c41a96a46665b7936fd87acca877588a3b70ec51451663cc2ae
  status: contract_specified
splat_requires_mechanics: *id001
---

# They’re Not Hit Points or Healing

<!-- SRD page 18 -->

Temporary Hit Points can’t be added to your Hit\
Points, healing can’t restore them, and receiving\
Temporary Hit Points doesn’t count as healing. Be-\
cause Temporary Hit Points aren’t Hit Points, a crea-\
ture can be at full Hit Points and receive Temporary\
Hit Points.\
If you have 0 Hit Points, receiving Temporary Hit\
Points doesn’t restore you to consciousness. Only\
true healing can save you.\
