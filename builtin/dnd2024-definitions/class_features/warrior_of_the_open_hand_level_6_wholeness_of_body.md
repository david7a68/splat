---
splat_schema: 1
splat_id: 69d6f033-3559-59a4-805c-875a03cca778
splat_kind: rule
splat_key: dnd2024:class_features/warrior_of_the_open_hand_level_6_wholeness_of_body
splat_name: 'Level 6: Wholeness of Body'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/warrior_of_the_open_hand_level_6_wholeness_of_body
  parameters:
    action: bonus_action
    heal_self:
      op: max
      args:
      - 1
      - op: add
        args:
        - dice:
            count: 1
            sides:
              stat: martial_arts_die.sides
        - stat: abilities.wisdom.modifier
    uses:
      maximum:
        op: max
        args:
        - 1
        - stat: abilities.wisdom.modifier
      recover:
        event: rest.long_rest_completed
        amount: all
  implementation: dnd2024:class_features_warrior_of_the_open_hand_level_6_wholeness_of_body
splat_contracts:
- id: dnd2024:class_features_warrior_of_the_open_hand_level_6_wholeness_of_body
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - resource_changes
  - healing
  events:
  - action.bonus
  capabilities:
  - healing
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 52
  engine_gaps: &id001
  - dnd2024:class_features_warrior_of_the_open_hand_level_6_wholeness_of_body
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: bc45148f0de1e0cf91851fc7c9818ad21912c7841b8274de9a5088c7b374b7a1
  parent_name: Warrior of the Open Hand
  line_span:
    first: 4366
    count: 10
splat_clauses:
- key: source_rules
  text: '# Level 6: Wholeness of Body You gain the ability to heal yourself. As a Bonus Action, you can
    roll your Martial Arts die. You regain a number of Hit Points equal to the number rolled plus your
    Wisdom modifier (minimum of 1 Hit Point regained). You can use this feature a number of times equal
    to your Wisdom modifier (minimum of once), and you regain all expended uses when you finish a Long
    Rest.'
  sha256: 94e66c42924a86e3af43c5bd510c09c6e6d3b8772c230fe9d0c09b5273c29e9d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 6: Wholeness of Body

<!-- SRD page 52 -->

You gain the ability to heal yourself. As a Bonus Ac-\
tion, you can roll your Martial Arts die. You regain\
a number of Hit Points equal to the number rolled\
plus your Wisdom modifier (minimum of 1 Hit Point\
regained).\
You can use this feature a number of times equal\
to your Wisdom modifier (minimum of once), and\
you regain all expended uses when you finish a\
Long Rest.\
