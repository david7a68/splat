---
splat_schema: 1
splat_id: 00f66409-316e-5ce6-805a-8f00df17b893
splat_kind: rule
splat_key: dnd2024:rules/creating_a_background_1_choose_abilities
splat_name: '1: Choose Abilities'
splat_origin:
  key: dnd2024:reference/pages/192
splat_sources:
- title: SRD 5.2.1
  locator: pp. 192, 193
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/creating_a_background_1_choose_abilities
  parameters:
    background_ability_choice:
      count: 3
      distinct: true
      options:
      - strength
      - dexterity
      - constitution
      - intelligence
      - wisdom
      - charisma
      criterion: appropriate_to_background
    interleaved_terrain_dc_columns:
      source:
        key: dnd2024:rules/travel_pace_special_movement
      field: terrain_table
  implementation: dnd2024:rule_creating_a_background_1_choose_abilities
splat_contracts:
- id: dnd2024:rule_creating_a_background_1_choose_abilities
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - background_definition
  events: []
  capabilities:
  - background_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 192
  - 193
  engine_gaps: &id001
  - dnd2024:rule_creating_a_background_1_choose_abilities
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 1bf4afd8ee055322e07f33e35e85a7ff9829e80b02409746af5c597389a15dda
  parent_name: Creating a Background
  line_span:
    first: 9392
    count: 24
splat_clauses:
- key: source_rules
  text: '# 1: Choose Abilities Choose three abilities that seem appropriate for the background: Strength
    or Dexterity. These abilities are ideal for a background involving physical exertion. Constitution.
    This ability is ideal for a background that involves endurance or long hours of activity. Foraging
    DC Navigation DC Search DC 20 10 10 10 5 15 20 10 10 10 15 15 15 5 15 15 10 15 20 15 20 10 15 20 20
    10 20 20 15 15 15 10 15 Intelligence or Wisdom. One or both abilities are ideal for a background that
    focuses on cerebral or spiritual matters. Charisma. This ability is ideal for a background that involves
    performance or social interaction.'
  sha256: ab2c9c20c77e0e98d1221ff1ba47b467599c1723192f9266e5f34a20d652ea08
  status: contract_specified
splat_requires_mechanics: *id001
---

# 1: Choose Abilities

<!-- SRD page 192 -->

Choose three abilities that seem appropriate for the\
background:\
Strength or Dexterity. These abilities are ideal for\
a background involving physical exertion.\
Constitution. This ability is ideal for a background\
that involves endurance or long hours of activity.\
Foraging DC Navigation DC Search DC\
20 10 10\
10 5 15\
20 10 10\
10 15 15\
15 5 15\
15 10 15\
20 15 20\
10 15 20\
20 10 20\
20 15 15\
15 10 15\

<!-- SRD page 193 -->

Intelligence or Wisdom. One or both abilities are\
ideal for a background that focuses on cerebral or\
spiritual matters.\
Charisma. This ability is ideal for a background\
that involves performance or social interaction.\
