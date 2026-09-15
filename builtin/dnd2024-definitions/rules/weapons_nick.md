---
splat_schema: 1
splat_id: 28f409b2-54e1-5730-b98f-9ee06070e38a
splat_kind: rule
splat_key: dnd2024:rules/weapons_nick
splat_name: Nick
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_nick
  parameters:
    extra_light_attack_economy_choice:
    - part_of_attack_action
    - bonus_action
    extra_light_attack_per_turn_limit: 1
  implementation: dnd2024:rules_weapons_nick
splat_contracts:
- id: dnd2024:rules_weapons_nick
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
  - dnd2024:rules_weapons_nick
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: d2adcdf43a1087b6926773671c74fe332ba8b901011ef97c175c20b213686a87
  parent_name: Weapons
  line_span:
    first: 7788
    count: 5
splat_clauses:
- key: source_rules
  text: '# Nick When you make the extra attack of the Light property, you can make it as part of the Attack
    action instead of as a Bonus Action. You can make this extra attack only once per turn.'
  sha256: cfaac3ae715329f58e517ce142a250cdf6717c0873cdf57b625cc11a59c345dc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Nick

<!-- SRD page 90 -->

When you make the extra attack of the Light prop-\
erty, you can make it as part of the Attack action in-\
stead of as a Bonus Action. You can make this extra\
attack only once per turn.\
