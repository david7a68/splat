---
splat_schema: 1
splat_id: 210679f4-23a6-5183-80e9-87002c3038d3
splat_kind: item
splat_key: dnd2024:equipment/holy_symbol_varies
splat_name: Holy Symbol (Varies)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:equipment/holy_symbol_varies
  implementation: dnd2024:equipment_holy_symbol_varies
  parameters:
    spellcasting_focus:
      eligible_classes:
      - key: dnd2024:classes/cleric
      - key: dnd2024:classes/paladin
      rule:
        key: dnd2024:glossary/spellcasting_focus
    variants:
    - id: amulet
      mass:
        value: 1
        unit: lb
      price:
        value: 5
        unit: gp
      valid_use_states:
      - worn
      - held
    - id: emblem
      mass:
        value: 0
        unit: lb
      price:
        value: 5
        unit: gp
      valid_use_states:
      - borne_on_fabric
      - borne_on_shield
    - id: reliquary
      mass:
        value: 2
        unit: lb
      price:
        value: 5
        unit: gp
      valid_use_states:
      - held
splat_contracts:
- id: dnd2024:equipment_holy_symbol_varies
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
  - 97
  engine_gaps: &id001
  - dnd2024:equipment_holy_symbol_varies
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 4b59465809bcb3c9dad3732004963e5c3c64a14a958641b7985490743a6dc66d
  line_span:
    first: 8420
    count: 13
splat_clauses:
- key: source_rules
  text: '# Holy Symbol (Varies) A Holy Symbol takes one of the forms in the Holy Symbol table and is bejeweled
    or painted to channel divine magic. A Cleric or Paladin can use a Holy Symbol as a Spellcasting Focus.
    The table indicates whether a Holy Symbol needs to be held, worn, or borne on fabric (such as a tabard
    or banner) or a Shield. Holy Symbols Symbol Weight Cost Amulet (worn or held) 1 lb. 5 GP Emblem (borne
    on fabric or a Shield) — 5 GP Reliquary (held) 2 lb. 5 GP'
  sha256: 4daa7fd0251bf1d48b8df9098963ac4ad01650adb747f902370ebb2832ab24bb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Holy Symbol (Varies)

<!-- SRD page 97 -->

A Holy Symbol takes one of the forms in the Holy\
Symbol table and is bejeweled or painted to chan-\
nel divine magic. A Cleric or Paladin can use a Holy\
Symbol as a Spellcasting Focus.\
The table indicates whether a Holy Symbol needs\
to be held, worn, or borne on fabric (such as a tab-\
ard or banner) or a Shield.\
Holy Symbols\
Symbol Weight Cost\
Amulet (worn or held) 1 lb. 5 GP\
Emblem (borne on fabric or a Shield) — 5 GP\
Reliquary (held) 2 lb. 5 GP\
