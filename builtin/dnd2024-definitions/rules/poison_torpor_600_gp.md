---
splat_schema: 1
splat_id: f1f9e408-ca42-5199-9419-f34758028cc9
splat_kind: rule
splat_key: dnd2024:rules/poison_torpor_600_gp
splat_name: Torpor (600 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_torpor_600_gp
  parameters:
    price_gp_per_dose: 600
    poison_type: ingested
    save:
      ability: constitution
      dc: 15
      failure:
        condition: poisoned
        duration:
          unit: hours
          amount:
            dice:
              count: 4
              sides: 6
    while_poisoned_by_this_effect:
      speed_multiplier: 0.5
  implementation: dnd2024:rule_poison_torpor_600_gp
splat_contracts:
- id: dnd2024:rule_poison_torpor_600_gp
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - effects
  events:
  - poison.exposure
  capabilities:
  - poisons
  - conditions
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 198
  engine_gaps: &id001
  - dnd2024:rule_poison_torpor_600_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: f387b45a45bd8f9cdb5f51858939686fc3ae2da5c6f676242a90822275825a11
  parent_name: Poison
  line_span:
    first: 9914
    count: 7
splat_clauses:
- key: source_rules
  text: '# Torpor (600 GP) Ingested Poison A creature subjected to Torpor poison must succeed on a DC
    15 Constitution saving throw or have the Poisoned condition for 4d6 hours. The creature’s Speed is
    halved while the creature is Poisoned in this way.'
  sha256: 217c34db403ca79537cef1312c12b43bcdfc7b0644aaa6494235dc44318cf73e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Torpor (600 GP)

<!-- SRD page 198 -->

Ingested Poison\
A creature subjected to Torpor poison must succeed\
on a DC 15 Constitution saving throw or have the\
Poisoned condition for 4d6 hours. The creature’s\
Speed is halved while the creature is Poisoned in\
this way.\
