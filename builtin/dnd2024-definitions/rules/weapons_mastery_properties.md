---
splat_schema: 1
splat_id: e41205a0-0b7b-56f7-aed3-c79b3b489277
splat_kind: rule
splat_key: dnd2024:rules/weapons_mastery_properties
splat_name: Mastery Properties
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_mastery_properties
  parameters:
    mastery_requires_unlocking_feature: true
    improvised_weapon_rules:
      key: dnd2024:glossary/improvised_weapons
    improvised_cases:
    - makeshift_object_weapon
    - weapon_used_unusually
  implementation: dnd2024:rules_weapons_mastery_properties
splat_contracts:
- id: dnd2024:rules_weapons_mastery_properties
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    event:
      type: event
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - action_options
  - effects
  - resource_updates
  events: []
  capabilities:
  - action_economy
  - movement
  - combat
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 90
  engine_gaps: &id001
  - dnd2024:rules_weapons_mastery_properties
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 88d85333fd2c32a1f86957f12106eba4a50efeb2277ae92e4e9a0c1e9a2fb5e8
  parent_name: Weapons
  line_span:
    first: 7761
    count: 11
splat_clauses:
- key: source_rules
  text: '# Mastery Properties Each weapon has a mastery property, which is usable only by a character
    who has a feature, such as Weapon Mastery, that unlocks the property for the character. The properties
    are defined below. iMprovisEd wEapons If you use an object—such as a table leg, frying pan, or bottle—as
    a makeshift weapon, see “Improvised Weapons” in “Rules Glossary.” Also see those rules if you wield
    a weapon in an unusual way, such as using a Ranged weapon to make a melee attack.'
  sha256: c1d78d88a53c7bbb1a2b820bcb611461d11e60366164d881f79b983957da28a7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mastery Properties

<!-- SRD page 90 -->

Each weapon has a mastery property, which is us-\
able only by a character who has a feature, such as\
Weapon Mastery, that unlocks the property for the\
character. The properties are defined below.\
iMprovisEd wEapons\
If you use an object—such as a table leg, frying pan,\
or bottle—as a makeshift weapon, see “Improvised\
Weapons” in “Rules Glossary.” Also see those rules\
if you wield a weapon in an unusual way, such as using\
a Ranged weapon to make a melee attack.\
