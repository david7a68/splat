---
splat_schema: 1
splat_id: 1366dc42-9425-52c3-a79c-6c0a7fe46c73
splat_kind: rule
splat_key: dnd2024:rules/starting_at_higher_levels_creating_your_character
splat_name: Creating Your Character
splat_origin:
  key: dnd2024:reference/pages/024
splat_sources:
- title: SRD 5.2.1
  locator: pp. 24
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/starting_at_higher_levels_creating_your_character
  parameters:
    normal_character_creation_and_advancement: true
    starting_xp: minimum_for_starting_level
    normal_starting_equipment: true
    starting_equipment_table:
    - levels:
      - 2
      - 4
      base_gp: 0
      random_gp: 0
      magic_items:
        common: 1
    - levels:
      - 5
      - 10
      base_gp: 500
      random_gp:
        op: multiply
        args:
        - dice:
            count: 1
            sides: 10
        - 25
      magic_items:
        common: 1
        uncommon: 1
    - levels:
      - 11
      - 16
      base_gp: 5000
      random_gp:
        op: multiply
        args:
        - dice:
            count: 1
            sides: 10
        - 250
      magic_items:
        common: 2
        uncommon: 3
        rare: 1
    - levels:
      - 17
      - 20
      base_gp: 20000
      random_gp:
        op: multiply
        args:
        - dice:
            count: 1
            sides: 10
        - 250
      magic_items:
        common: 2
        uncommon: 4
        rare: 3
        very_rare: 1
    level_twenty_bonus_feats:
      gm_option: true
      xp_baseline: 355000
      xp_per_feat: 30000
      choice: any_qualified_feat
      level_for_prerequisites: 20
  implementation: dnd2024:rule_starting_at_higher_levels_creating_your_character
splat_contracts:
- id: dnd2024:rule_starting_at_higher_levels_creating_your_character
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
  - character_changes
  events: []
  capabilities:
  - character_creation
  - advancement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 24
  engine_gaps: &id001
  - dnd2024:rule_starting_at_higher_levels_creating_your_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: dc7626c338137baceac3929148b71dab266e9c780c43a625774799d0b5f28aed
  parent_name: Starting at Higher Levels
  line_span:
    first: 1855
    count: 27
splat_clauses:
- key: source_rules
  text: '# Creating Your Character Creating a higher-level character uses the same character-creation
    steps outlined in this chapter and the rules for advancing beyond level 1 provided Starting Equipment
    at Higher Levels Starting Level Equipment and Money 2–4 Normal starting equipment 5–10 500 GP plus
    1d10 × 25 GP plus normal starting equipment 11–16 5,000 gp plus 1d10 × 250 GP plus normal starting
    equipment 17–20 20,000 GP plus 1d10 × 250 GP plus normal starting equipment Bonus FEats at lEvEl 20
    A GM can use feats as a form of advancement after characters reach level 20 to provide greater power
    to characters who have no more levels to gain. With this approach, each character gains one feat of
    their choice for every 30,000 XP the character earns above 355,000 XP. Epic Boon feats are especially
    appropriate for these bonus feats, but a player can choose any feat for which their level 20 character
    qualifies. in the “Level Advancement” section. You begin with the minimum amount of XP required to
    reach your starting level. For example, if the GM starts you at level 10, you have 64,000 XP.'
  sha256: 04975ced3a6f9f5c7933d49d4ff4b1fbd0490d6a59c6e7ffd17aa9f4ef2f6284
  status: contract_specified
splat_requires_mechanics: *id001
---

# Creating Your Character

<!-- SRD page 24 -->

Creating a higher-level character uses the same\
character-creation steps outlined in this chapter\
and the rules for advancing beyond level 1 provided\
Starting Equipment at Higher Levels\
Starting Level Equipment and Money\
2–4 Normal starting equipment\
5–10 500 GP plus 1d10 × 25 GP plus\
normal starting equipment\
11–16 5,000 gp plus 1d10 × 250 GP plus\
normal starting equipment\
17–20 20,000 GP plus 1d10 × 250 GP plus\
normal starting equipment\
Bonus FEats at lEvEl 20\
A GM can use feats as a form of advancement after\
characters reach level 20 to provide greater power\
to characters who have no more levels to gain. With\
this approach, each character gains one feat of their\
choice for every 30,000 XP the character earns\
above 355,000 XP. Epic Boon feats are especially\
appropriate for these bonus feats, but a player can\
choose any feat for which their level 20 character\
qualifies.\
in the “Level Advancement” section. You begin with\
the minimum amount of XP required to reach your\
starting level. For example, if the GM starts you at\
level 10, you have 64,000 XP.\
