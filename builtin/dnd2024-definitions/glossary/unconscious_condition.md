---
splat_schema: 1
splat_id: 5654254d-8dca-55f6-a45a-c0d4c922c425
splat_kind: rule
splat_key: dnd2024:glossary/unconscious_condition
splat_name: Unconscious [Condition]
splat_origin:
  key: dnd2024:reference/glossary/unconscious_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 191
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/unconscious_condition
  parameters:
    condition: unconscious
    grants_condition: incapacitated
    automatic_save_failure_abilities:
    - strength
    - dexterity
    incoming_attack_rolls: advantage
    automatic_critical_if_hit_attacker_within_feet: 5
    speed: 0
    speed_can_increase: false
    also_apply_condition: prone
    drop_all_held_items: true
    prone_persists_after_end: true
    aware_of_surroundings: false
  implementation: dnd2024:glossary_unconscious_condition
splat_contracts:
- id: dnd2024:glossary_unconscious_condition
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
  source_sha256: 8e8a90c31a2d0ae239a57dbd5a7457ac92c7e2bf0adc8e0ab10764a0ab6ee804
  source_path: builtin/srd-5.2.1/glossary/unconscious_condition.md
  pages:
  - 191
  engine_gaps: &id001
  - dnd2024:glossary_unconscious_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: fc50e5c1eae7f8f069426dc59cda1a68ccd120a2d4f0270883ee4099127d03fd
splat_clauses:
- key: source_rules
  text: '# Unconscious [Condition] While you have the Unconscious condition, you experience the following
    effects. Inert. You have the Incapacitated and Prone conditions, and you drop whatever you’re holding.
    When this condition ends, you remain Prone. Speed 0. Your Speed is 0 and can’t increase. Attacks Affected.
    Attack rolls against you have Advantage. Saving Throws Affected. You automatically fail Strength and
    Dexterity saving throws. Automatic Critical Hits. Any attack roll that hits you is a Critical Hit
    if the attacker is within 5 feet of you. Unaware. You’re unaware of your surroundings.'
  sha256: e15f7d0dcad736698e5bcc9c6ec52a6ca2830237d47a479a484bbbb1c5e35943
  status: contract_specified
splat_requires_mechanics: *id001
---

# Unconscious [Condition]

<!-- SRD page 191 -->

While you have the Unconscious condition, you ex-\
perience the following effects.\
Inert. You have the Incapacitated and Prone\
conditions, and you drop whatever you’re holding.\
When this condition ends, you remain Prone.\
Speed 0. Your Speed is 0 and can’t increase.\
Attacks Affected. Attack rolls against you have\
Advantage.\
Saving Throws Affected. You automatically fail\
Strength and Dexterity saving throws.\
Automatic Critical Hits. Any attack roll that hits\
you is a Critical Hit if the attacker is within 5 feet of\
you.\
Unaware. You’re unaware of your surroundings.\

Original layout: [[../../srd-5.2.1/pages/191|page 191]].
