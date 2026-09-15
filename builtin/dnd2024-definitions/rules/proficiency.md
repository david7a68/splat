---
splat_schema: 1
splat_id: 3294f9d1-667e-5099-b2b2-6669bf3273e8
splat_kind: rule
splat_key: dnd2024:rules/proficiency
splat_name: Proficiency
splat_origin:
  key: dnd2024:reference/pages/008
splat_sources:
- title: SRD 5.2.1
  locator: pp. 8
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/proficiency
  parameters:
    basis:
      character: total_level
      monster: challenge_rating
    table:
    - minimum: 0
      maximum: 4
      bonus: 2
    - minimum: 5
      maximum: 8
      bonus: 3
    - minimum: 9
      maximum: 12
      bonus: 4
    - minimum: 13
      maximum: 16
      bonus: 5
    - minimum: 17
      maximum: 20
      bonus: 6
    - minimum: 21
      maximum: 24
      bonus: 7
    - minimum: 25
      maximum: 28
      bonus: 8
    - minimum: 29
      maximum: 30
      bonus: 9
    applications:
    - proficient_skill_test
    - proficient_save
    - proficient_item_test
    - spell_attack
    - spell_save_dc
  implementation: dnd2024:rules_proficiency
splat_contracts:
- id: dnd2024:rules_proficiency
  inputs:
    actor:
      type: participant
      required: true
    test:
      type: resolution
      required: true
  outputs:
  - test_modifiers
  - test_result
  events: []
  capabilities:
  - d20_tests
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 8
  engine_gaps: &id001
  - dnd2024:rules_proficiency
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: f02fd89a1e5f99bec307de1f38fc8a7365549c3ddcf2a1eda9d3bc5a3a404bb8
  parent_name: Proficiency
  line_span:
    first: 336
    count: 25
splat_clauses:
- key: source_rules
  text: '# Proficiency Characters and monsters are good at various things. Some are skilled with many
    weapons, while others can use only a few. Some are better at understanding people’s motives, and others
    are better at unlocking the secrets of the multiverse. All creatures have a Proficiency Bonus, which
    reflects the impact that training has on the creature’s capabilities. A character’s Proficiency Bonus
    increases as the character gains levels (described in “Character Creation”). A monster’s Proficiency
    Bonus is based on its Challenge Rating (see “Rules Glossary”). The Proficiency Bonus table shows how
    the bonus is determined. This bonus is applied to a D20 Test when the creature has proficiency in
    a skill, in a saving throw, or with an item that the creature uses to make the D20 Test. The bonus
    is also used for spell attacks and for calculating the DC of saving throws for spells. Proficiency
    Bonus Level or CR Bonus Level or CR Bonus Up to 4 +2 17–20 +6 5–8 +3 21–24 +7 9–12 +4 25–28 +8 13–16
    +5 29–30 +9'
  sha256: 3a9e888cb8566b1ffad69a3c838e74b3fc2614e4bfd21bf9e794c537b8547ebe
  status: contract_specified
splat_requires_mechanics: *id001
---

# Proficiency

<!-- SRD page 8 -->

Characters and monsters are good at various\
things. Some are skilled with many weapons, while\
others can use only a few. Some are better at under-\
standing people’s motives, and others are better at\
unlocking the secrets of the multiverse. All crea-\
tures have a Proficiency Bonus, which reflects the\
impact that training has on the creature’s capabil-\
ities. A character’s Proficiency Bonus increases as\
the character gains levels (described in “Character\
Creation”). A monster’s Proficiency Bonus is based\
on its Challenge Rating (see “Rules Glossary”). The\
Proficiency Bonus table shows how the bonus is\
determined.\
This bonus is applied to a D20 Test when the crea-\
ture has proficiency in a skill, in a saving throw, or\
with an item that the creature uses to make the D20\
Test. The bonus is also used for spell attacks and for\
calculating the DC of saving throws for spells.\
Proficiency Bonus\
Level or CR Bonus Level or CR Bonus\
Up to 4 +2 17–20 +6\
5–8 +3 21–24 +7\
9–12 +4 25–28 +8\
13–16 +5 29–30 +9\
