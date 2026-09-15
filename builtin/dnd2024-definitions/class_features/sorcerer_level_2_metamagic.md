---
splat_schema: 1
splat_id: df3c746f-db36-5799-8b04-e777c124986e
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_2_metamagic
splat_name: 'Level 2: Metamagic'
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/sorcerer_level_2_metamagic
  parameters:
    metamagic_choices:
      count_by_sorcerer_level:
        '2': 2
        '10': 4
        '17': 6
      source:
        key: dnd2024:rules/sorcerer_metamagic_options
      replace:
        event: character.sorcerer_level_gained
        maximum: 1
        new_option_must_be_unknown: true
    pay_option_sorcery_point_cost: true
    maximum_options_per_cast: 1
    option_specific_stacking_exception_allowed: true
  implementation: dnd2024:class_features_sorcerer_level_2_metamagic
splat_contracts:
- id: dnd2024:class_features_sorcerer_level_2_metamagic
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
  outputs:
  - feature_state
  - choices
  - resource_changes
  events:
  - character.class_level_gained
  - spell.cast_pending
  capabilities:
  - metamagic
  - resources
  - character_choices
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 66
  engine_gaps: &id001
  - dnd2024:class_features_sorcerer_level_2_metamagic
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 3baa7d697bd4b86ec680cd4e8ab0e137cb55251381d9b8e171701e3bb8a56309
  parent_name: Sorcerer
  line_span:
    first: 5579
    count: 15
splat_clauses:
- key: source_rules
  text: '# Level 2: Metamagic Because your magic flows from within, you can alter your spells to suit
    your needs; you gain two Metamagic options of your choice from “Meta magic Options” later in this
    class’s description. You use the chosen options to temporarily modify spells you cast. To use an option,
    you must spend the number of Sorcery Points that it costs. You can use only one Metamagic option on
    a spell when you cast it unless otherwise noted in one of those options. Whenever you gain a Sorcerer
    level, you can replace one of your Metamagic options with one you don’t know. You gain two more options
    at Sorcerer level 10 and two more at Sorcerer level 17.'
  sha256: 389d06529b3f87b80b8a98dd0c9c1ea00bf7373a37c05987f474617b90484429
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Metamagic

<!-- SRD page 66 -->

Because your magic flows from within, you can\
alter your spells to suit your needs; you gain two\
Metamagic options of your choice from “Meta magic\
Options” later in this class’s description. You use\
the chosen options to temporarily modify spells you\
cast. To use an option, you must spend the number\
of Sorcery Points that it costs.\
You can use only one Metamagic option on a spell\
when you cast it unless otherwise noted in one of\
those options.\
Whenever you gain a Sorcerer level, you can re-\
place one of your Metamagic options with one you\
don’t know. You gain two more options at Sorcerer\
level 10 and two more at Sorcerer level 17.\
