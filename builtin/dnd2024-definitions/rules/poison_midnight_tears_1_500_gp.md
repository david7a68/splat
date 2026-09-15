---
splat_schema: 1
splat_id: 650e5703-4479-5566-947b-0c3fb2b83fbf
splat_kind: rule
splat_key: dnd2024:rules/poison_midnight_tears_1_500_gp
splat_name: Midnight Tears (1,500 GP)
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197, 198
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_midnight_tears_1_500_gp
  parameters:
    price_gp_per_dose: 1500
    poison_type: ingested
    activation_timing: next_midnight
    before_activation_effect: none
    neutralized_by: any_effect_ending_poisoned_condition
    activation_save:
      ability: constitution
      dc: 17
      failure:
        damage:
          dice:
            count: 9
            sides: 6
        average: 31
        type: poison
      success:
        damage_fraction: 0.5
        rounding: floor
  implementation: dnd2024:rule_poison_midnight_tears_1_500_gp
splat_contracts:
- id: dnd2024:rule_poison_midnight_tears_1_500_gp
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
  - midnight
  capabilities:
  - poisons
  - damage
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 197
  - 198
  engine_gaps: &id001
  - dnd2024:rule_poison_midnight_tears_1_500_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 34a46d887c4690b3085035cc3685f058e8b1336faded7ae6f2c8a50ac9d31d84
  parent_name: Poison
  line_span:
    first: 9865
    count: 10
splat_clauses:
- key: source_rules
  text: '# Midnight Tears (1,500 GP) Ingested Poison A creature that ingests Midnight Tears suffers no
    effect until the stroke of midnight. Any effect that ends the Poisoned condition neutralizes this
    poison. If the poison hasn’t been neutralized before midnight, the creature makes a DC 17 Constitution
    saving throw, taking 31 (9d6) Poison damage on a failed save or half as much damage on a successful
    one.'
  sha256: d516d7532aa739bdf7dee7c3e682250c9755ce6148d972f8772b43d658ac5068
  status: contract_specified
splat_requires_mechanics: *id001
---

# Midnight Tears (1,500 GP)

<!-- SRD page 197 -->

Ingested Poison\
A creature that ingests Midnight Tears suffers no\
effect until the stroke of midnight. Any effect that\

<!-- SRD page 198 -->

ends the Poisoned condition neutralizes this poi-\
son. If the poison hasn’t been neutralized before\
midnight, the creature makes a DC 17 Constitution\
saving throw, taking 31 (9d6) Poison damage on a\
failed save or half as much damage on a successful\
one.\
