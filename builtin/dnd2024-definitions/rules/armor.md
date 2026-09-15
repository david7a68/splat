---
splat_schema: 1
splat_id: c2d97ced-3600-5a0b-9643-d39884784fca
splat_kind: rule
splat_key: dnd2024:rules/armor
splat_name: Armor
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/armor
  parameters:
    armor:
    - id: padded_armor
      category: light
      base_ac: 11
      dexterity_cap: null
      add_dexterity: true
      strength_minimum: null
      stealth_disadvantage: true
      weight_lb: 8
      cost_gp: 5
    - id: leather_armor
      category: light
      base_ac: 11
      dexterity_cap: null
      add_dexterity: true
      strength_minimum: null
      stealth_disadvantage: false
      weight_lb: 10
      cost_gp: 10
    - id: studded_leather_armor
      category: light
      base_ac: 12
      dexterity_cap: null
      add_dexterity: true
      strength_minimum: null
      stealth_disadvantage: false
      weight_lb: 13
      cost_gp: 45
    - id: hide_armor
      category: medium
      base_ac: 12
      dexterity_cap: 2
      add_dexterity: true
      strength_minimum: null
      stealth_disadvantage: false
      weight_lb: 12
      cost_gp: 10
    - id: chain_shirt
      category: medium
      base_ac: 13
      dexterity_cap: 2
      add_dexterity: true
      strength_minimum: null
      stealth_disadvantage: false
      weight_lb: 20
      cost_gp: 50
    - id: scale_mail
      category: medium
      base_ac: 14
      dexterity_cap: 2
      add_dexterity: true
      strength_minimum: null
      stealth_disadvantage: true
      weight_lb: 45
      cost_gp: 50
    - id: breastplate
      category: medium
      base_ac: 14
      dexterity_cap: 2
      add_dexterity: true
      strength_minimum: null
      stealth_disadvantage: false
      weight_lb: 20
      cost_gp: 400
    - id: half_plate_armor
      category: medium
      base_ac: 15
      dexterity_cap: 2
      add_dexterity: true
      strength_minimum: null
      stealth_disadvantage: true
      weight_lb: 40
      cost_gp: 750
    - id: ring_mail
      category: heavy
      base_ac: 14
      dexterity_cap: 0
      add_dexterity: false
      strength_minimum: null
      stealth_disadvantage: true
      weight_lb: 40
      cost_gp: 30
    - id: chain_mail
      category: heavy
      base_ac: 16
      dexterity_cap: 0
      add_dexterity: false
      strength_minimum: 13
      stealth_disadvantage: true
      weight_lb: 55
      cost_gp: 75
    - id: splint_armor
      category: heavy
      base_ac: 17
      dexterity_cap: 0
      add_dexterity: false
      strength_minimum: 15
      stealth_disadvantage: true
      weight_lb: 60
      cost_gp: 200
    - id: plate_armor
      category: heavy
      base_ac: 18
      dexterity_cap: 0
      add_dexterity: false
      strength_minimum: 15
      stealth_disadvantage: true
      weight_lb: 65
      cost_gp: 1500
    don_doff_minutes:
      light:
        don: 1
        doff: 1
      medium:
        don: 5
        doff: 1
      heavy:
        don: 10
        doff: 5
    insufficient_strength_speed_penalty_feet: 10
    shield:
      ac_bonus: 2
      don_action: utilize
      doff_action: utilize
      weight_lb: 6
      cost_gp: 10
  implementation: dnd2024:rules_armor
splat_contracts:
- id: dnd2024:rules_armor
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - item_attributes
  - actor_modifiers
  events: []
  capabilities:
  - equipment
  - armor_class
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 92
  engine_gaps: &id001
  - dnd2024:rules_armor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: fada2ba7cfa91d06ba112f57bc875e1c879c7a1e051b8b09422b363b9cce851a
  parent_name: Armor
  line_span:
    first: 7905
    count: 40
splat_clauses:
- key: source_rules
  text: '# Armor The Armor table lists the game’s main armor. The table includes the cost and weight of
    armor, as well as the following details: Category. Every type of armor falls into a category: Light,
    Medium, or Heavy. The category determines how long it takes to don or doff the armor (as shown in
    the table). Armor Class (AC). The table’s Armor Class column tells you what your base AC is when you
    wear a type of armor. For example, if you wear Leather Armor, your base AC is 11 plus your Dexterity
    modifier, whereas your AC is 16 in Chain Mail. Strength. If the table shows a Strength score in the
    Strength column for an armor type, that armor reduces the wearer’s speed by 10 feet unless the wearer
    has a Strength score equal to or higher than the listed score. Stealth. If the table shows “Disadvantage”
    in the Stealth column for an armor type, the wearer has Disadvantage on Dexterity (Stealth) checks.
    Armor Armor Armor Class (AC) Light Armor (1 Minute to Don or Doff) Padded Armor 11 + Dex modifier
    Leather Armor 11 + Dex modifier Studded Leather Armor 12 + Dex modifier Medium Armor (5 Minutes to
    Don and 1 Minute to Doff) Hide Armor 12 + Dex modifier (max 2) Chain Shirt 13 + Dex modifier (max
    2) Scale Mail 14 + Dex modifier (max 2) Breastplate 14 + Dex modifier (max 2) Half Plate Armor 15
    + Dex modifier (max 2) Heavy Armor (10 Minutes to Don and 5 Minutes to Doff) Ring Mail 14 Chain Mail
    16 Splint Armor 17 Plate Armor 18 Shield (Utilize Action to Don or Doff) Shield +2'
  sha256: 1257edbb30d16873184ac7d3ba7737c9ef97a768323c61c4fd252b32ccc0ce12
  status: contract_specified
splat_requires_mechanics: *id001
---

# Armor

<!-- SRD page 92 -->

The Armor table lists the game’s main armor. The\
table includes the cost and weight of armor, as well\
as the following details:\
Category. Every type of armor falls into a category:\
Light, Medium, or Heavy. The category determines\
how long it takes to don or doff the armor (as\
shown in the table).\
Armor Class (AC). The table’s Armor Class column\
tells you what your base AC is when you wear a\
type of armor. For example, if you wear Leather\
Armor, your base AC is 11 plus your Dexterity\
modifier, whereas your AC is 16 in Chain Mail.\
Strength. If the table shows a Strength score in the\
Strength column for an armor type, that armor\
reduces the wearer’s speed by 10 feet unless the\
wearer has a Strength score equal to or higher\
than the listed score.\
Stealth. If the table shows “Disadvantage” in the\
Stealth column for an armor type, the wearer has\
Disadvantage on Dexterity (Stealth) checks.\
Armor\
Armor Armor Class (AC)\
Light Armor (1 Minute to Don or Doff)\
Padded Armor 11 + Dex modifier\
Leather Armor 11 + Dex modifier\
Studded Leather Armor 12 + Dex modifier\
Medium Armor (5 Minutes to Don and 1 Minute to Doff)\
Hide Armor 12 + Dex modifier (max 2)\
Chain Shirt 13 + Dex modifier (max 2)\
Scale Mail 14 + Dex modifier (max 2)\
Breastplate 14 + Dex modifier (max 2)\
Half Plate Armor 15 + Dex modifier (max 2)\
Heavy Armor (10 Minutes to Don and 5 Minutes to Doff)\
Ring Mail 14\
Chain Mail 16\
Splint Armor 17\
Plate Armor 18\
Shield (Utilize Action to Don or Doff)\
Shield +2\
