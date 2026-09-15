---
splat_schema: 1
splat_id: 92fc1503-04b1-5c5e-b686-6f411546cc55
splat_kind: item
splat_key: dnd2024:equipment/spell_scroll_cantrip_30_gp_level_1_50_gp
splat_name: Spell Scroll (Cantrip, 30 GP; Level 1, 50 GP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/spell_scroll_cantrip_30_gp_level_1_50_gp
  implementation: dnd2024:equipment_spell_scroll_cantrip_30_gp_level_1_50_gp
  parameters:
    magical: true
    variants:
    - id: cantrip
      spell_level: 0
      price:
        value: 30
        unit: gp
    - id: level_1
      spell_level: 1
      price:
        value: 50
        unit: gp
    spell_selection:
      authority: creator
      level: variant_spell_level
    cast:
      requires_spell_on_class_list: true
      requires_reading: true
      casting_time: spell_normal
      material_components_required: false
      save_dc: 13
      attack_bonus: 5
      on_completed: scroll_disintegrates
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_spell_scroll_cantrip_30_gp_level_1_50_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    spell:
      type: definition
      required: true
  outputs:
  - cast_eligibility
  - spell_cast
  - item_consumption
  events:
  - spell.cast_pending
  - spell.cast_completed
  capabilities:
  - item_spellcasting
  - spellcasting
  - consumables
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 99
  engine_gaps: &id001
  - dnd2024:equipment_spell_scroll_cantrip_30_gp_level_1_50_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 301e82dda6d82443bd54437732a0a855245e34e8b3c939bc430087e7d773abed
  line_span:
    first: 8638
    count: 12
splat_clauses:
- key: source_rules
  text: '# Spell Scroll (Cantrip, 30 GP; Level 1, 50 GP) A Spell Scroll (Cantrip) or Spell Scroll (Level
    1) is a magic item that bears the words of a cantrip or level 1 spell, respectively, determined by
    the scroll’s creator. If the spell is on your class’s spell list, you can read the scroll and cast
    the spell using its normal casting time and without providing any Material components. If the spell
    requires a saving throw or an attack roll, the spell save DC is 13, and the attack bonus is +5. The
    scroll disintegrates when the casting is completed.'
  sha256: 11bc0c033abe240262412baec140e6682613e6edc26cb1665f2cf649695add4b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spell Scroll (Cantrip, 30 GP; Level 1, 50 GP)

<!-- SRD page 99 -->

A Spell Scroll (Cantrip) or Spell Scroll (Level 1) is a\
magic item that bears the words of a cantrip or level\
1 spell, respectively, determined by the scroll’s cre-\
ator. If the spell is on your class’s spell list, you can\
read the scroll and cast the spell using its normal\
casting time and without providing any Material\
components.\
If the spell requires a saving throw or an attack\
roll, the spell save DC is 13, and the attack bonus\
is +5. The scroll disintegrates when the casting is\
completed.\
