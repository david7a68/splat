---
splat_schema: 1
splat_id: 5ca3b212-eeda-5ac9-8547-56357a6eb8a1
splat_kind: rule
splat_key: dnd2024:rules/poison_crawler_mucus_200_gp
splat_name: Crawler Mucus (200 GP)
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_crawler_mucus_200_gp
  parameters:
    price_gp_per_dose: 200
    poison_type: contact
    save:
      ability: constitution
      dc: 13
      failure:
        condition: poisoned
        duration_minutes: 1
    while_poisoned_by_this_effect:
      condition: paralyzed
    repeat_save:
      timing: end_of_each_target_turn
      success: effect_ends
  implementation: dnd2024:rule_poison_crawler_mucus_200_gp
splat_contracts:
- id: dnd2024:rule_poison_crawler_mucus_200_gp
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
  - turn.ended
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
  - 197
  engine_gaps: &id001
  - dnd2024:rule_poison_crawler_mucus_200_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: c7da163aa932d98bb686ca7505f26e7383a960c264fbf27fb79e0bd700852988
  parent_name: Poison
  line_span:
    first: 9841
    count: 9
splat_clauses:
- key: source_rules
  text: '# Crawler Mucus (200 GP) Contact Poison A creature subjected to Crawler Mucus must succeed on
    a DC 13 Constitution saving throw or have the Poisoned condition for 1 minute. The creature also has
    the Paralyzed condition while Poisoned in this way. The creature repeats the save at the end of each
    of its turns, ending the effect on itself on a success.'
  sha256: 201f2c13c7baa02c3f287c8d246c04cb74e8d9f581d8dec1733e3bcf7c02b43f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Crawler Mucus (200 GP)

<!-- SRD page 197 -->

Contact Poison\
A creature subjected to Crawler Mucus must suc-\
ceed on a DC 13 Constitution saving throw or have\
the Poisoned condition for 1 minute. The creature\
also has the Paralyzed condition while Poisoned in\
this way. The creature repeats the save at the end\
of each of its turns, ending the effect on itself on a\
success.\
