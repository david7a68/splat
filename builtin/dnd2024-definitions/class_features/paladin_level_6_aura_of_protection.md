---
splat_schema: 1
splat_id: e65baf9f-5f63-571a-8be6-01a7317efcf6
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_6_aura_of_protection
splat_name: 'Level 6: Aura of Protection'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_6_aura_of_protection
  parameters:
    aura:
      shape: emanation
      radius_feet: 10
      origin: self
      visible: false
      inactive_when_condition: incapacitated
    targets:
    - self
    - allies_in_aura
    saving_throw_bonus:
      op: max
      args:
      - 1
      - stat: abilities.charisma.modifier
    stacking:
      maximum_aura_of_protection_benefits: 1
      selector: affected_creature
  implementation: dnd2024:class_features_paladin_level_6_aura_of_protection
splat_contracts:
- id: dnd2024:class_features_paladin_level_6_aura_of_protection
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
  events:
  - saving_throw.pending
  - aura.membership_changed
  - condition.applied
  capabilities:
  - auras
  - saving_throws
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 55
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_6_aura_of_protection
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: fa79123baae762654d80ba8400c15d34407e000d6d98574629a93548818ab7f7
  parent_name: Paladin
  line_span:
    first: 4615
    count: 10
splat_clauses:
- key: source_rules
  text: '# Level 6: Aura of Protection You radiate a protective, unseeable aura in a 10-foot Emanation
    that originates from you. The aura is inactive while you have the Incapacitated condition. You and
    your allies in the aura gain a bonus to saving throws equal to your Charisma modifier (minimum bonus
    of +1). If another Paladin is present, a creature can benefit from only one Aura of Protection at
    a time; the creature chooses which aura while in them.'
  sha256: 300b9a76118074f995a79ad0f1b291f0bc84f069216245342c2d02eae2c5a873
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 6: Aura of Protection

<!-- SRD page 55 -->

You radiate a protective, unseeable aura in a 10-foot\
Emanation that originates from you. The aura is in-\
active while you have the Incapacitated condition.\
You and your allies in the aura gain a bonus to\
saving throws equal to your Charisma modifier\
(minimum bonus of +1).\
If another Paladin is present, a creature can ben-\
efit from only one Aura of Protection at a time; the\
creature chooses which aura while in them.\
