---
splat_schema: 1
splat_id: 745adcb4-6a98-5fdf-8b2f-b5e1d7565d27
splat_kind: rule
splat_key: dnd2024:glossary/invisible_condition
splat_name: Invisible [Condition]
splat_origin:
  key: dnd2024:reference/glossary/invisible_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/invisible_condition
  parameters:
    condition: invisible
    initiative_roll: advantage
    concealment:
      unaffected_by_requires_seen_target_effects: true
      exception: effect_creator_can_see_subject
      includes_worn_and_carried_equipment: true
    attacks:
      incoming: disadvantage
      outgoing: advantage
      exception: other_creature_can_see_subject
  implementation: dnd2024:glossary_invisible_condition
splat_contracts:
- id: dnd2024:glossary_invisible_condition
  inputs:
    subject:
      type: participant
      required: true
    other:
      required: false
      type: participant
  outputs:
  - condition_effects
  events:
  - initiative.rolled
  - attack.pending
  - effect.targeting_pending
  capabilities:
  - conditions
  - visibility
  - attacks
  - initiative
splat_conversion:
  status: specified
  category: glossary
  source_sha256: f6de66e525c41c4b0be5f49a2a77b4d0bce194399872316f0602d5a0471a1737
  source_path: builtin/srd-5.2.1/glossary/invisible_condition.md
  pages:
  - 184
  engine_gaps: &id001
  - dnd2024:glossary_invisible_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 868f99446b41050a966475e3bffa30adf4b56bf6791edd312f62df3e3b5a7b46
splat_clauses:
- key: source_rules
  text: '# Invisible [Condition] While you have the Invisible condition, you experience the following
    effects. Surprise. If you’re Invisible when you roll Initiative, you have Advantage on the roll. Concealed.
    You aren’t affected by any effect that requires its target to be seen unless the effect’s creator
    can somehow see you. Any equipment you are wearing or carrying is also concealed. Attacks Affected.
    Attack rolls against you have Disadvantage, and your attack rolls have Advantage. If a creature can
    somehow see you, you don’t gain this benefit against that creature.'
  sha256: 8cde10a301dbdb87acc7a382b09435df300fea31e181564bed507502c3d0c0da
  status: contract_specified
splat_requires_mechanics: *id001
---

# Invisible [Condition]

<!-- SRD page 184 -->

While you have the Invisible condition, you experi-\
ence the following effects.\
Surprise. If you’re Invisible when you roll Initia-\
tive, you have Advantage on the roll.\
Concealed. You aren’t affected by any effect that\
requires its target to be seen unless the effect’s cre-\
ator can somehow see you. Any equipment you are\
wearing or carrying is also concealed.\
Attacks Affected. Attack rolls against you have\
Disadvantage, and your attack rolls have Advan-\
tage. If a creature can somehow see you, you don’t\
gain this benefit against that creature.\

Original layout: [[../../srd-5.2.1/pages/184|page 184]].
