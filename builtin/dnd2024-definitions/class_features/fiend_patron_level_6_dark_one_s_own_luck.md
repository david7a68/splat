---
splat_schema: 1
splat_id: ce9081a6-0245-5d7f-8d29-be9a3f5f2d46
splat_kind: rule
splat_key: dnd2024:class_features/fiend_patron_level_6_dark_one_s_own_luck
splat_name: 'Level 6: Dark One’s Own Luck'
splat_origin:
  key: dnd2024:reference/pages/076
splat_sources:
- title: SRD 5.2.1
  locator: pp. 76
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fiend_patron_level_6_dark_one_s_own_luck
  parameters:
    roll_types:
    - ability_check
    - saving_throw
    timing: after_roll_before_effects
    optional_bonus:
      dice:
        count: 1
        sides: 10
    maximum_uses_per_roll: 1
    uses:
      maximum:
        op: max
        args:
        - 1
        - stat: abilities.charisma.modifier
      recover:
        event: rest.long_rest_completed
        amount: all
  implementation: dnd2024:class_features_fiend_patron_level_6_dark_one_s_own_luck
splat_contracts:
- id: dnd2024:class_features_fiend_patron_level_6_dark_one_s_own_luck
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  events:
  - ability_check.rolled
  - saving_throw.rolled
  capabilities:
  - roll_modifiers
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 76
  engine_gaps: &id001
  - dnd2024:class_features_fiend_patron_level_6_dark_one_s_own_luck
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 17b6b7ce236d850f8be75e2f1a01687c6a16db8a88780be9d3739f21b9f0a18c
  parent_name: Fiend Patron
  line_span:
    first: 6499
    count: 10
splat_clauses:
- key: source_rules
  text: '# Level 6: Dark One’s Own Luck You can call on your fiendish patron to alter fate in your favor.
    When you make an ability check or a saving throw, you can use this feature to add 1d10 to your roll.
    You can do so after seeing the roll but before any of the roll’s effects occur. You can use this feature
    a number of times equal to your Charisma modifier (minimum of once), but you can use it no more than
    once per roll. You regain all expended uses when you finish a Long Rest.'
  sha256: 860d4cffb063a6377ac39156e7f3f21e376e463e8e6ac42894c357bdf883805b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 6: Dark One’s Own Luck

<!-- SRD page 76 -->

You can call on your fiendish patron to alter fate\
in your favor. When you make an ability check or a\
saving throw, you can use this feature to add 1d10\
to your roll. You can do so after seeing the roll but\
before any of the roll’s effects occur.\
You can use this feature a number of times equal\
to your Charisma modifier (minimum of once), but\
you can use it no more than once per roll. You regain\
all expended uses when you finish a Long Rest.\
