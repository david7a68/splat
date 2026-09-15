---
splat_schema: 1
splat_id: 063a68d0-1f6c-52ac-94ac-e756933dc5d0
splat_kind: rule
splat_key: dnd2024:glossary/passive_perception
splat_name: Passive Perception
splat_origin:
  key: dnd2024:reference/glossary/passive_perception
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/passive_perception
  parameters:
    use:
      chooser: gm
      scope: notice_without_conscious_perception_check
    score:
      op: add
      args:
      - 10
      - var: wisdom_perception_check_bonus
    roll_mode_adjustment:
      advantage: 5
      disadvantage: -5
      normal: 0
  implementation: dnd2024:glossary_passive_perception
splat_contracts:
- id: dnd2024:glossary_passive_perception
  inputs:
    subject:
      type: participant
      required: true
    roll_mode:
      type: choice
      required: true
    gm_uses_passive:
      type: boolean
      required: true
  outputs:
  - passive_perception
  events: []
  capabilities:
  - checks
  - perception
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 740bede7f808a0c1c89e2ad4cb66a46a6308eff0cd7fb7ffe2e2e9ac6ad6616f
  source_path: builtin/srd-5.2.1/glossary/passive_perception.md
  pages:
  - 186
  engine_gaps: &id001
  - dnd2024:glossary_passive_perception
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 95e31da9eb0465ef86d69bdf6f912a1985dfa06ad47a05facbd2167e27a29b0b
splat_clauses:
- key: source_rules
  text: '# Passive Perception Passive Perception is a score that reflects a creature’s general awareness
    of its surroundings. The GM uses this score when determining whether a creature notices something
    without consciously making a Wisdom (Perception) check. A creature’s Passive Perception equals 10
    plus the creature’s Wisdom (Perception) check bonus. If the creature has Advantage on such checks,
    increase the score by 5. If the creature has Disadvantage on them, decrease the score by 5. For example,
    a level 1 character with a Wisdom of 15 and proficiency in Perception has a Passive Perception of
    14 (10 + 2 + 2). If that character has Advantage on Wisdom (Perception) checks, the score becomes
    19.'
  sha256: 79bf78bb43baa809e236bddb4aa5f905605d4db43f8247368368027007a60c43
  status: contract_specified
splat_requires_mechanics: *id001
---

# Passive Perception

<!-- SRD page 186 -->

Passive Perception is a score that reflects a crea-\
ture’s general awareness of its surroundings. The\
GM uses this score when determining whether a\
creature notices something without consciously\
making a Wisdom (Perception) check.\
A creature’s Passive Perception equals 10 plus the\
creature’s Wisdom (Perception) check bonus. If the\
creature has Advantage on such checks, increase\
the score by 5. If the creature has Disadvantage on\
them, decrease the score by 5. For example, a level\
1 character with a Wisdom of 15 and proficiency in\
Perception has a Passive Perception of 14 (10 + 2 +\
2). If that character has Advantage on Wisdom (Per-\
ception) checks, the score becomes 19.\

Original layout: [[../../srd-5.2.1/pages/186|page 186]].
