---
splat_schema: 1
splat_id: af7ddf38-e3b0-5c31-9d7e-1a34cd4ba1f1
splat_kind: rule
splat_key: dnd2024:glossary/charmed_condition
splat_name: Charmed [Condition]
splat_origin:
  key: dnd2024:reference/glossary/charmed_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 178
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/charmed_condition
  parameters:
    condition: charmed
    forbidden_against_charmer:
    - attack
    - target_with_damaging_ability
    - target_with_damaging_magical_effect
    charmer_social_ability_checks: advantage
  implementation: dnd2024:glossary_charmed_condition
splat_contracts:
- id: dnd2024:glossary_charmed_condition
  inputs:
    subject:
      type: participant
      required: true
    charmer:
      type: participant
      required: true
  outputs:
  - condition_effects
  events:
  - attack.pending
  - effect.targeting_pending
  - ability_check.pending
  capabilities:
  - conditions
  - relationships
  - targeting
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 4f1dc10dd3350cabf47ca285a2c40dc518e576423922d9099ee9bcaec2b1849c
  source_path: builtin/srd-5.2.1/glossary/charmed_condition.md
  pages:
  - 178
  engine_gaps: &id001
  - dnd2024:glossary_charmed_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: fd7afb211d9dfaa2d3c9ef1383d61b6ee41e701d77d45bc23c39c4c8c702b924
splat_clauses:
- key: source_rules
  text: '# Charmed [Condition] While you have the Charmed condition, you experience the following effects.
    Can’t Harm the Charmer. You can’t attack the charmer or target the charmer with damaging abilities
    or magical effects. Social Advantage. The charmer has Advantage on any ability check to interact with
    you socially.'
  sha256: 3361ac4f3b03ba47552fb56859b8dad6c7b05f6d79559d61749a3eacafecbe59
  status: contract_specified
splat_requires_mechanics: *id001
---

# Charmed [Condition]

<!-- SRD page 178 -->

While you have the Charmed condition, you experi-\
ence the following effects.\
Can’t Harm the Charmer. You can’t attack the\
charmer or target the charmer with damaging abili-\
ties or magical effects.\
Social Advantage. The charmer has Advantage on\
any ability check to interact with you socially.\

Original layout: [[../../srd-5.2.1/pages/178|page 178]].
