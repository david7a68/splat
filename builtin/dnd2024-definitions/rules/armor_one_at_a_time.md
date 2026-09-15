---
splat_schema: 1
splat_id: a5bcb9fa-4c3b-5e4a-be4d-fc4c65d7ddac
splat_kind: rule
splat_key: dnd2024:rules/armor_one_at_a_time
splat_name: One at a Time
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/armor_one_at_a_time
  parameters:
    worn_armor_suits_max: 1
    wielded_shields_max: 1
    armor_table:
    - armor: padded
      strength_min: null
      stealth_disadvantage: true
      weight_lb: 8
      cost_gp: 5
    - armor: leather
      strength_min: null
      stealth_disadvantage: false
      weight_lb: 10
      cost_gp: 10
    - armor: studded_leather
      strength_min: null
      stealth_disadvantage: false
      weight_lb: 13
      cost_gp: 45
    - armor: hide
      strength_min: null
      stealth_disadvantage: false
      weight_lb: 12
      cost_gp: 10
    - armor: chain_shirt
      strength_min: null
      stealth_disadvantage: false
      weight_lb: 20
      cost_gp: 50
    - armor: scale_mail
      strength_min: null
      stealth_disadvantage: true
      weight_lb: 45
      cost_gp: 50
    - armor: breastplate
      strength_min: null
      stealth_disadvantage: false
      weight_lb: 20
      cost_gp: 400
    - armor: half_plate
      strength_min: null
      stealth_disadvantage: true
      weight_lb: 40
      cost_gp: 750
    - armor: ring_mail
      strength_min: null
      stealth_disadvantage: true
      weight_lb: 40
      cost_gp: 30
    - armor: chain_mail
      strength_min: 13
      stealth_disadvantage: true
      weight_lb: 55
      cost_gp: 75
    - armor: splint
      strength_min: 15
      stealth_disadvantage: true
      weight_lb: 60
      cost_gp: 200
    - armor: plate
      strength_min: 15
      stealth_disadvantage: true
      weight_lb: 65
      cost_gp: 1500
    - armor: shield
      strength_min: null
      stealth_disadvantage: false
      weight_lb: 6
      cost_gp: 10
  implementation: dnd2024:rules_armor_one_at_a_time
splat_contracts:
- id: dnd2024:rules_armor_one_at_a_time
  inputs:
    subject:
      type: participant
      required: true
    equipment:
      type: item_list
      required: true
  outputs:
  - equipment_limits
  - armor_statistics
  events: []
  capabilities:
  - equipment
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 92
  engine_gaps: &id001
  - dnd2024:rules_armor_one_at_a_time
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 6536951feaaecfe150792dc498e970e84071451bfcd6264f3da76a079d8857dc
  parent_name: Armor
  line_span:
    first: 7960
    count: 17
splat_clauses:
- key: source_rules
  text: '# One at a Time A creature can wear only one suit of armor at a time and wield only one Shield
    at a time. Strength Stealth Weight Cost — Disadvantage 8 lb. 5 GP — — 10 lb. 10 GP — — 13 lb. 45 GP
    — — 12 lb. 10 GP — — 20 lb. 50 GP — Disadvantage 45 lb. 50 GP — — 20 lb. 400 GP — Disadvantage 40
    lb. 750 GP — Disadvantage 40 lb. 30 GP Str 13 Disadvantage 55 lb. 75 GP Str 15 Disadvantage 60 lb.
    200 GP Str 15 Disadvantage 65 lb. 1,500 GP — — 6 lb. 10 GP'
  sha256: 63ec3fd196de734b5a5ff336a86c8fa2b932c9157a1444ebe04206a9917d5a49
  status: contract_specified
splat_requires_mechanics: *id001
---

# One at a Time

<!-- SRD page 92 -->

A creature can wear only one suit of armor at a time\
and wield only one Shield at a time.\
Strength Stealth Weight Cost\
— Disadvantage 8 lb. 5 GP\
— — 10 lb. 10 GP\
— — 13 lb. 45 GP\
— — 12 lb. 10 GP\
— — 20 lb. 50 GP\
— Disadvantage 45 lb. 50 GP\
— — 20 lb. 400 GP\
— Disadvantage 40 lb. 750 GP\
— Disadvantage 40 lb. 30 GP\
Str 13 Disadvantage 55 lb. 75 GP\
Str 15 Disadvantage 60 lb. 200 GP\
Str 15 Disadvantage 65 lb. 1,500 GP\
— — 6 lb. 10 GP\
