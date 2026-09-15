---
splat_schema: 1
splat_id: bd48fcf6-9d0b-52e1-8969-a7d7c0b411b0
splat_kind: rule
splat_key: dnd2024:class_features/thief_level_3_fast_hands
splat_name: 'Level 3: Fast Hands'
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/thief_level_3_fast_hands
  parameters:
    action: bonus_action
    choose: 1
    options:
      sleight_of_hand:
        ability: dexterity
        skill: sleight_of_hand
        tasks:
          pick_lock:
            tool: thieves_tools
          disarm_trap:
            tool: thieves_tools
          pick_pocket: {}
      utilize:
        take_action: utilize
      magic_item:
        take_action: magic
        scope: use_magic_item
        item_requires_action: magic
    class_scope:
      class: rogue
      minimum_level: 3
  implementation: dnd2024:class_features_thief_level_3_fast_hands
splat_contracts:
- id: dnd2024:class_features_thief_level_3_fast_hands
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - action.bonus
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 64
  engine_gaps: &id001
  - dnd2024:class_features_thief_level_3_fast_hands
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: d9b1a87be9fb187128251d12221a10be2ad9923394eb765262cd03fef1123d0b
  parent_name: Thief
  line_span:
    first: 5358
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 3: Fast Hands As a Bonus Action, you can do one of the following. Sleight of Hand. Make
    a Dexterity (Sleight of Hand) check to pick a lock or disarm a trap with Thieves’ Tools or to pick
    a pocket. Use an Object. Take the Utilize action, or take the Magic action to use a magic item that
    requires that action.'
  sha256: 0787698028930c75ba4be612b91658c71384944f60a59e40103c4e2fc08defb1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Fast Hands

<!-- SRD page 64 -->

As a Bonus Action, you can do one of the following.\
Sleight of Hand. Make a Dexterity (Sleight of\
Hand) check to pick a lock or disarm a trap with\
Thieves’ Tools or to pick a pocket.\
Use an Object. Take the Utilize action, or take the\
Magic action to use a magic item that requires that\
action.\
