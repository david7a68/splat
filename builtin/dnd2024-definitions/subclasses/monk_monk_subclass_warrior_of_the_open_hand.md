---
splat_schema: 1
splat_id: 13143fc7-f575-57c2-8aba-013c1ad33eb0
splat_kind: rule_group
splat_key: dnd2024:subclasses/monk_monk_subclass_warrior_of_the_open_hand
splat_name: 'Monk Subclass: Warrior of the Open Hand'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: subclass
  members:
  - key: dnd2024:class_features/warrior_of_the_open_hand_level_11_fleet_step
  - key: dnd2024:class_features/warrior_of_the_open_hand_level_17_quivering_palm
  - key: dnd2024:class_features/warrior_of_the_open_hand_level_3_open_hand_technique
  - key: dnd2024:class_features/warrior_of_the_open_hand_level_6_wholeness_of_body
splat_rule:
  mechanic: dnd2024:subclasses/monk_monk_subclass_warrior_of_the_open_hand
  implementation: dnd2024:subclasses_monk_monk_subclass_warrior_of_the_open_hand
  parameters:
    class: monk
    subclass: warrior_of_the_open_hand
    feature_grants:
    - feature:
        key: dnd2024:class_features/warrior_of_the_open_hand_level_11_fleet_step
      minimum_class_level: 11
    - feature:
        key: dnd2024:class_features/warrior_of_the_open_hand_level_17_quivering_palm
      minimum_class_level: 17
    - feature:
        key: dnd2024:class_features/warrior_of_the_open_hand_level_3_open_hand_technique
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/warrior_of_the_open_hand_level_6_wholeness_of_body
      minimum_class_level: 6
splat_contracts:
- id: dnd2024:subclasses_monk_monk_subclass_warrior_of_the_open_hand
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - character.subclass_selected
  - character.class_level_gained
  capabilities:
  - class_progression
splat_conversion:
  status: specified
  category: subclass
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 52
  engine_gaps: &id001
  - dnd2024:subclasses_monk_monk_subclass_warrior_of_the_open_hand
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: a48816d37beabb0b5afa87d9f622fe9b32666407f9acdbb3d6a91eb23e721643
  parent_name: Monk
  line_span:
    first: 4350
    count: 6
splat_clauses:
- key: source_rules
  text: '# Monk Subclass: Warrior of the Open Hand Master Unarmed Combat Techniques Warriors of the Open
    Hand are masters of unarmed combat. They learn techniques to push and trip their opponents and manipulate
    their own energy to protect themselves from harm.'
  sha256: 475ab11ab6532a6a4e99b0d53cc3e32eec551a9b2af5a45fff4307a080cb285d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Monk Subclass: Warrior of the Open Hand

<!-- SRD page 52 -->

Master Unarmed Combat Techniques\
Warriors of the Open Hand are masters of unarmed\
combat. They learn techniques to push and trip\
their opponents and manipulate their own energy\
to protect themselves from harm.\
