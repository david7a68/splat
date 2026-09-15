---
splat_schema: 1
splat_id: 2d7b19f1-002c-505c-a275-8ca535664a40
splat_kind: item
splat_key: dnd2024:equipment/arcane_focus_varies
splat_name: Arcane Focus (Varies)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:equipment/arcane_focus_varies
  implementation: dnd2024:equipment_arcane_focus_varies
  parameters:
    spellcasting_focus:
      eligible_classes:
      - key: dnd2024:classes/sorcerer
      - key: dnd2024:classes/warlock
      - key: dnd2024:classes/wizard
      rule:
        key: dnd2024:glossary/spellcasting_focus
    variants:
    - id: crystal
      mass:
        value: 1
        unit: lb
      price:
        value: 10
        unit: gp
      valid_use_states:
      - held
    - id: orb
      mass:
        value: 3
        unit: lb
      price:
        value: 20
        unit: gp
      valid_use_states:
      - held
    - id: rod
      mass:
        value: 2
        unit: lb
      price:
        value: 10
        unit: gp
      valid_use_states:
      - held
    - id: staff
      mass:
        value: 4
        unit: lb
      price:
        value: 5
        unit: gp
      also_weapon:
        key: dnd2024:equipment/quarterstaff
      valid_use_states:
      - held
    - id: wand
      mass:
        value: 1
        unit: lb
      price:
        value: 10
        unit: gp
      valid_use_states:
      - held
splat_contracts:
- id: dnd2024:equipment_arcane_focus_varies
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    variant:
      type: choice
      required: true
    spell:
      required: false
      type: definition
  outputs:
  - focus_eligibility
  - variant_properties
  events:
  - spell.cast_pending
  capabilities:
  - spellcasting_focus
  - item_variants
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 96
  engine_gaps: &id001
  - dnd2024:equipment_arcane_focus_varies
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 03a81a7491c9eb0ff965d610fb8a05e59f0c6f37043baa3cb18283c823a74607
  line_span:
    first: 8265
    count: 12
splat_clauses:
- key: source_rules
  text: '# Arcane Focus (Varies) An Arcane Focus takes one of the forms in the Arcane Focuses table and
    is bejeweled or carved to channel arcane magic. A Sorcerer, Warlock, or Wizard can use such an item
    as a Spellcasting Focus. Arcane Focuses Focus Weight Cost Crystal 1 lb. 10 GP Orb 3 lb. 20 GP Rod
    2 lb. 10 GP Staff (also a Quarterstaff) 4 lb. 5 GP Wand 1 lb. 10 GP'
  sha256: 667126ca6d925f037a18462e4034ca15002f3f9e461335f83bd3b7398ac46f1f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Arcane Focus (Varies)

<!-- SRD page 96 -->

An Arcane Focus takes one of the forms in the\
Arcane Focuses table and is bejeweled or carved to\
channel arcane magic. A Sorcerer, Warlock, or Wiz-\
ard can use such an item as a Spellcasting Focus.\
Arcane Focuses\
Focus Weight Cost\
Crystal 1 lb. 10 GP\
Orb 3 lb. 20 GP\
Rod 2 lb. 10 GP\
Staff (also a Quarterstaff) 4 lb. 5 GP\
Wand 1 lb. 10 GP\
