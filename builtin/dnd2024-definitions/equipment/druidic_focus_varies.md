---
splat_schema: 1
splat_id: 834bcd37-0663-53ce-aac3-a2718dacb30f
splat_kind: item
splat_key: dnd2024:equipment/druidic_focus_varies
splat_name: Druidic Focus (Varies)
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
  mechanic: dnd2024:equipment/druidic_focus_varies
  implementation: dnd2024:equipment_druidic_focus_varies
  parameters:
    spellcasting_focus:
      eligible_classes:
      - key: dnd2024:classes/druid
      - key: dnd2024:classes/ranger
      rule:
        key: dnd2024:glossary/spellcasting_focus
    variants:
    - id: sprig_of_mistletoe
      mass:
        value: 0
        unit: lb
      price:
        value: 1
        unit: gp
      valid_use_states:
      - held
    - id: wooden_staff
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
    - id: yew_wand
      mass:
        value: 1
        unit: lb
      price:
        value: 10
        unit: gp
      valid_use_states:
      - held
splat_contracts:
- id: dnd2024:equipment_druidic_focus_varies
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
  - dnd2024:equipment_druidic_focus_varies
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 2e558e9f6b94df553d64c8b1dca8752cfffe52457fd7074ea5afefbf5c349a8f
  line_span:
    first: 8383
    count: 10
splat_clauses:
- key: source_rules
  text: '# Druidic Focus (Varies) A Druidic Focus takes one of the forms in the Druidic Focuses table
    and is carved, tied with ribbon, or painted to channel primal magic. A Druid or Ranger can use such
    an object as a Spellcasting Focus. Druidic Focuses Focus Weight Cost Sprig of mistletoe — 1 GP Wooden
    staff (also a Quarterstaff) 4 lb. 5 GP Yew wand 1 lb. 10 GP'
  sha256: 4b18f4df319581e94842578eceb8f5bd212ff10208dad70519357df77c73723b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Druidic Focus (Varies)

<!-- SRD page 97 -->

A Druidic Focus takes one of the forms in the Dru-\
idic Focuses table and is carved, tied with ribbon, or\
painted to channel primal magic. A Druid or Ranger\
can use such an object as a Spellcasting Focus.\
Druidic Focuses\
Focus Weight Cost\
Sprig of mistletoe — 1 GP\
Wooden staff (also a Quarterstaff) 4 lb. 5 GP\
Yew wand 1 lb. 10 GP\
