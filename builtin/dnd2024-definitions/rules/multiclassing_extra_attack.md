---
splat_schema: 1
splat_id: 47bb1c5b-e5cc-5360-914e-4c91ed046089
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_extra_attack
splat_name: Extra Attack
splat_origin:
  key: dnd2024:reference/pages/025
splat_sources:
- title: SRD 5.2.1
  locator: pp. 25
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/multiclassing_extra_attack
  parameters:
    extra_attack:
      stacks_across_classes: false
      default_attacks: 2
      higher_count_requires_explicit_feature: true
    thirsting_blade:
      stacks_with_extra_attack: false
      weapon_scope: pact_weapon
  implementation: dnd2024:rules_multiclassing_extra_attack
splat_contracts:
- id: dnd2024:rules_multiclassing_extra_attack
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - character_updates
  events: []
  capabilities:
  - character_creation
  - progression
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 25
  engine_gaps: &id001
  - dnd2024:rules_multiclassing_extra_attack
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 2616c0dd357e8457a99fd76b94f532c19013471d511979a20edbd943c548743e
  parent_name: Multiclassing
  line_span:
    first: 1967
    count: 10
splat_clauses:
- key: source_rules
  text: '# Extra Attack If you gain the Extra Attack feature from more than one class, the features don’t
    stack. You can’t make more than two attacks with this feature unless you have a feature that says
    you can (such as the Fighter’s Two Extra Attacks feature). Similarly, the Warlock’s Thirsting Blade
    invocation, which grants you the Extra Attack feature with your pact weapon, doesn’t give you additional
    attacks if you also have Extra Attack.'
  sha256: 156ffccd8e4a7a540db181d480cbcceb8f23818d4aa88c65bedf572f077294bd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Extra Attack

<!-- SRD page 25 -->

If you gain the Extra Attack feature from more than\
one class, the features don’t stack. You can’t make\
more than two attacks with this feature unless you\
have a feature that says you can (such as the Fight-\
er’s Two Extra Attacks feature).\
Similarly, the Warlock’s Thirsting Blade invo-\
cation, which grants you the Extra Attack feature\
with your pact weapon, doesn’t give you additional\
attacks if you also have Extra Attack.\
