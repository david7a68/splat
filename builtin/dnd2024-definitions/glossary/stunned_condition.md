---
splat_schema: 1
splat_id: 0a971496-ae3b-5b45-91b8-87841b2e2ae3
splat_kind: rule
splat_key: dnd2024:glossary/stunned_condition
splat_name: Stunned [Condition]
splat_origin:
  key: dnd2024:reference/glossary/stunned_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 189
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/stunned_condition
  parameters:
    condition: stunned
    grants_condition: incapacitated
    automatic_save_failure_abilities:
    - strength
    - dexterity
    incoming_attack_rolls: advantage
  implementation: dnd2024:glossary_stunned_condition
splat_contracts:
- id: dnd2024:glossary_stunned_condition
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - condition_effects
  events:
  - condition.applied
  - condition.ended
  - attack.pending
  - saving_throw.pending
  - speed.calculated
  capabilities:
  - conditions
  - checks
  - attacks
  - movement
  - effects
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 4cf37a78e1f6b18257167bd0d76d36cfb3988209cdc653c18340c1cc3fb412bc
  source_path: builtin/srd-5.2.1/glossary/stunned_condition.md
  pages:
  - 189
  engine_gaps: &id001
  - dnd2024:glossary_stunned_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 7fa0653a7c76e2e9a7ad398b6d5e79756026bce6585047c51c978299d034a416
splat_clauses:
- key: source_rules
  text: '# Stunned [Condition] While you have the Stunned condition, you experience the following effects.
    Incapacitated. You have the Incapacitated condition. Saving Throws Affected. You automatically fail
    Strength and Dexterity saving throws. Attacks Affected. Attack rolls against you have Advantage.'
  sha256: 5764442253c37fe09babd6d8b3ebb4f19e01dc1caf5e7ecf80071247a7e820ac
  status: contract_specified
splat_requires_mechanics: *id001
---

# Stunned [Condition]

<!-- SRD page 189 -->

While you have the Stunned condition, you experi-\
ence the following effects.\
Incapacitated. You have the Incapacitated\
condition.\
Saving Throws Affected. You automatically fail\
Strength and Dexterity saving throws.\
Attacks Affected. Attack rolls against you have\
Advantage.\

Original layout: [[../../srd-5.2.1/pages/189|page 189]].
