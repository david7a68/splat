---
splat_schema: 1
splat_id: ace7edef-f701-5a08-8e58-48f203fe472e
splat_kind: rule
splat_key: dnd2024:rules/poison_pale_tincture_250_gp
splat_name: Pale Tincture (250 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_pale_tincture_250_gp
  parameters:
    cost_gp: 250
    exposure_route: ingested
    initial_save:
      ability: constitution
      dc: 16
    initial_failure:
      damage:
        dice:
          count: 1
          sides: 6
        average: 3
        type: poison
      condition: poisoned
    repeat_while_poisoned:
      interval:
        value: 24
        unit: hour
      ability: constitution
      dc: 16
      failure_damage:
        dice:
          count: 1
          sides: 6
        average: 3
        type: poison
    while_poisoned:
      healing_prohibited_for: damage_from_this_poison
      prohibition_scope: all_means
    ends:
      successful_saves: 7
      consecutive_required: false
  implementation: dnd2024:rules_poison_pale_tincture_250_gp
splat_contracts:
- id: dnd2024:rules_poison_pale_tincture_250_gp
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - poison.exposure
  - duration.expired
  - healing.pending
  capabilities:
  - poison
  - damage
  - healing
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 198
  engine_gaps: &id001
  - dnd2024:rules_poison_pale_tincture_250_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 2af086f83e1f0a64086f773e68a2265d7c1690a11887ed479d23481d8f992826
  parent_name: Poison
  line_span:
    first: 9882
    count: 11
splat_clauses:
- key: source_rules
  text: '# Pale Tincture (250 GP) Ingested Poison A creature subjected to Pale Tincture must succeed on
    a DC 16 Constitution saving throw or take 3 (1d6) Poison damage and have the Poisoned condition. The
    Poisoned creature repeats the save every 24 hours, taking 3 (1d6) Poison damage on a failed save.
    The damage the poison deals can’t be healed by any means while the creature remains Poisoned. After
    seven successful saves against the poison, the creature is no longer Poisoned.'
  sha256: c17fe1c44fb2c5e24ae9fcdf9de7508b1d3328f6c89762937c46ecccad3c5202
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pale Tincture (250 GP)

<!-- SRD page 198 -->

Ingested Poison\
A creature subjected to Pale Tincture must succeed\
on a DC 16 Constitution saving throw or take 3\
(1d6) Poison damage and have the Poisoned condi-\
tion. The Poisoned creature repeats the save every\
24 hours, taking 3 (1d6) Poison damage on a failed\
save. The damage the poison deals can’t be healed\
by any means while the creature remains Poisoned.\
After seven successful saves against the poison, the\
creature is no longer Poisoned.\
