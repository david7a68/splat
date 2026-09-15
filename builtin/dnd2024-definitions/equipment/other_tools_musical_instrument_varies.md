---
splat_schema: 1
splat_id: ad572d3b-16e7-5478-a402-ec28095e3c6e
splat_kind: item
splat_key: dnd2024:equipment/other_tools_musical_instrument_varies
splat_name: Musical Instrument (Varies)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:equipment/other_tools_musical_instrument_varies
  implementation: dnd2024:equipment_other_tools_musical_instrument_varies
  parameters:
    tool:
      ability: charisma
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: play_known_tune
        check:
          ability: charisma
          dc: 10
      - action: utilize
        operation: improvise_song
        check:
          ability: charisma
          dc: 15
      craftable: []
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    variants:
    - id: bagpipes
      price:
        value: 30
        unit: gp
      mass:
        value: 6
        unit: lb
    - id: drum
      price:
        value: 6
        unit: gp
      mass:
        value: 3
        unit: lb
    - id: dulcimer
      price:
        value: 25
        unit: gp
      mass:
        value: 10
        unit: lb
    - id: flute
      price:
        value: 2
        unit: gp
      mass:
        value: 1
        unit: lb
    - id: horn
      price:
        value: 3
        unit: gp
      mass:
        value: 2
        unit: lb
    - id: lute
      price:
        value: 35
        unit: gp
      mass:
        value: 2
        unit: lb
    - id: lyre
      price:
        value: 30
        unit: gp
      mass:
        value: 2
        unit: lb
    - id: pan_flute
      price:
        value: 12
        unit: gp
      mass:
        value: 2
        unit: lb
    - id: shawm
      price:
        value: 2
        unit: gp
      mass:
        value: 1
        unit: lb
    - id: viol
      price:
        value: 30
        unit: gp
      mass:
        value: 1
        unit: lb
splat_contracts:
- id: dnd2024:equipment_other_tools_musical_instrument_varies
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    operation:
      type: choice
      required: true
    target:
      required: false
      type: object
    choices:
      required: false
      type: choice
  outputs:
  - check_requests
  - tool_effects
  - crafting_options
  events:
  - action.utilize
  capabilities:
  - tool_proficiency
  - ability_checks
  - crafting
  - character_choices
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 94
  engine_gaps: &id001
  - dnd2024:equipment_other_tools_musical_instrument_varies
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: b5e2ddca3b15cc41adc983286852d4bff79b7178dde3f359a32ae81f3c6ef4b2
  parent_name: Other Tools
  line_span:
    first: 8120
    count: 8
splat_clauses:
- key: source_rules
  text: '# Musical Instrument (Varies) Ability: Charisma Weight: Varies Utilize: Play a known tune (DC
    10), or improvise a song (DC 15) Variants: Bagpipes (30 GP, 6 lb.), drum (6 GP, 3 lb.), dulcimer (25
    GP, 10 lb.), flute (2 GP, 1 lb.), horn (3 GP, 2 lb.), lute (35 GP, 2 lb.), lyre (30 GP, 2 lb.), pan
    flute (12 GP, 2 lb.), shawm (2 GP, 1 lb.), viol (30 GP, 1 lb.)'
  sha256: 89294d9499557b8ea2730629cc32f6f8602b21bb6849deda9cb04efa01b9c1b7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Musical Instrument (Varies)

<!-- SRD page 94 -->

Ability:  Charisma  Weight: Varies\
Utilize: Play a known tune (DC 10), or improvise a song\
(DC 15)\
Variants: Bagpipes (30 GP, 6 lb.), drum (6 GP, 3 lb.), dul-\
cimer (25 GP, 10 lb.), flute (2 GP, 1 lb.), horn (3 GP, 2\
lb.), lute (35 GP, 2 lb.), lyre (30 GP, 2 lb.), pan flute (12\
GP, 2 lb.), shawm (2 GP, 1 lb.), viol (30 GP, 1 lb.)\
