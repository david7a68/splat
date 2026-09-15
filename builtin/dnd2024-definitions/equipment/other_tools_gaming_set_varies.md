---
splat_schema: 1
splat_id: 7e27ca57-e047-5c7c-a358-f31c2dae1961
splat_kind: item
splat_key: dnd2024:equipment/other_tools_gaming_set_varies
splat_name: Gaming Set (Varies)
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
  mechanic: dnd2024:equipment/other_tools_gaming_set_varies
  implementation: dnd2024:equipment_other_tools_gaming_set_varies
  parameters:
    tool:
      ability: wisdom
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: detect_cheating
        check:
          ability: wisdom
          dc: 10
      - action: utilize
        operation: win_game
        check:
          ability: wisdom
          dc: 20
      craftable: []
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    variants:
    - id: dice
      price:
        value: 1
        unit: sp
      mass:
        value: 0
        unit: lb
        negligible: true
    - id: dragonchess
      price:
        value: 1
        unit: gp
      mass:
        value: 0
        unit: lb
        negligible: true
    - id: playing_cards
      price:
        value: 5
        unit: sp
      mass:
        value: 0
        unit: lb
        negligible: true
    - id: three_dragon_ante
      price:
        value: 1
        unit: gp
      mass:
        value: 0
        unit: lb
        negligible: true
splat_contracts:
- id: dnd2024:equipment_other_tools_gaming_set_varies
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
  - dnd2024:equipment_other_tools_gaming_set_varies
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: ca21c7e50c1d56b36356aed8944d40d850a7f0d1103517b8f7100b388cfd16ec
  parent_name: Other Tools
  line_span:
    first: 8110
    count: 6
splat_clauses:
- key: source_rules
  text: '# Gaming Set (Varies) Ability: Wisdom Weight: — Utilize: Discern whether someone is cheating
    (DC 10), or win the game (DC 20) Variants: Dice (1 SP), dragonchess (1 GP), playing cards (5 SP),
    three-dragon ante (1 GP)'
  sha256: 9cd5507a9fadd6f266536d41126ba8af5cff0db44a7cc17ef77b6a50532d87b3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gaming Set (Varies)

<!-- SRD page 94 -->

Ability: Wisdom   Weight: —\
Utilize: Discern whether someone is cheating (DC 10),\
or win the game (DC 20)\
Variants: Dice (1 SP), dragonchess (1 GP), playing cards\
(5 SP), three-dragon ante (1 GP)\
