---
splat_schema: 1
splat_id: 040600d5-0135-51ea-8bcf-08fa07854c03
splat_kind: rule
splat_key: dnd2024:rules/weapons_heavy
splat_name: Heavy
splat_origin:
  key: dnd2024:reference/pages/089
splat_sources:
- title: SRD 5.2.1
  locator: pp. 89
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_heavy
  parameters:
    requires_property: heavy
    disadvantage_cases:
    - weapon_category: melee
      strength_below: 13
    - weapon_category: ranged
      dexterity_below: 13
  implementation: dnd2024:rules_weapons_heavy
splat_contracts:
- id: dnd2024:rules_weapons_heavy
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - attack.calculate
  capabilities:
  - weapon_properties
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 89
  engine_gaps: &id001
  - dnd2024:rules_weapons_heavy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 986217bcef1f77cfa5c5040ce4c348e4a87e33127eefafcf3200cac2403f64cf
  parent_name: Weapons
  line_span:
    first: 7711
    count: 5
splat_clauses:
- key: source_rules
  text: '# Heavy You have Disadvantage on attack rolls with a Heavy weapon if it’s a Melee weapon and
    your Strength score isn’t at least 13 or if it’s a Ranged weapon and your Dexterity score isn’t at
    least 13.'
  sha256: 3e39d062b201b4e2f26bf281529f083800b5423dd3347e45a50a85d4f356c8ce
  status: contract_specified
splat_requires_mechanics: *id001
---

# Heavy

<!-- SRD page 89 -->

You have Disadvantage on attack rolls with a Heavy\
weapon if it’s a Melee weapon and your Strength\
score isn’t at least 13 or if it’s a Ranged weapon and\
your Dexterity score isn’t at least 13.\
