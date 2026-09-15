---
splat_schema: 1
splat_id: ce429d70-8e73-5bfa-8c4b-cb8c874d9b9a
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_instant_death
splat_name: Instant Death
splat_origin:
  key: dnd2024:reference/pages/017
splat_sources:
- title: SRD 5.2.1
  locator: pp. 17
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_instant_death
  parameters:
    death_conditions:
    - subject: monster
      hp: 0
      exception:
        authority: gm
        choice: use_character_death_rules
    - hp_maximum: 0
    - subject: character
      hp_reduced_to: 0
      remaining_damage_gte: hp_maximum
    death_rules:
      key: dnd2024:rules/damage_and_healing_death_saving_throws
  implementation: dnd2024:rules_damage_and_healing_instant_death
splat_contracts:
- id: dnd2024:rules_damage_and_healing_instant_death
  inputs:
    target:
      type: participant
      required: true
    damage:
      required: false
      type: resolution
    event:
      type: event
      required: true
  outputs:
  - hit_point_updates
  - condition_updates
  - death_save_updates
  events: []
  capabilities:
  - damage
  - healing
  - death_saves
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 17
  engine_gaps: &id001
  - dnd2024:rules_damage_and_healing_instant_death
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 1a7c89c9dd28a6b2c31374931181d5ca74dbf346d91553e2ee9b9bd765d34cad
  parent_name: Damage and Healing
  line_span:
    first: 1227
    count: 17
splat_clauses:
- key: source_rules
  text: '# Instant Death Here are the main ways a creature can die instantly. Monster Death. A monster
    dies the instant it drops to 0 Hit Points, although a Game Master can ignore this rule for an individual
    monster and treat it like a character. Hit Point Maximum of 0. A creature dies if its Hit Point maximum
    reaches 0. Certain effects drain life energy, reducing a creature’s Hit Point maximum. Massive Damage.
    When damage reduces a character to 0 Hit Points and damage remains, the character dies if the remainder
    equals or exceeds their Hit Point maximum. For example, if your character has a Hit Point maximum
    of 12, currently has 6 Hit Points, and takes 18 damage, the character drops to 0 Hit Points, but 12
    damage remains. The character then dies, since 12 equals their Hit Point maximum.'
  sha256: 6fca99ea82c8aac84c1c8734c3bef003cb23fa52b32fe58ca5cdae5e16b4122a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Instant Death

<!-- SRD page 17 -->

Here are the main ways a creature can die instantly.\
Monster Death. A monster dies the instant it\
drops to 0 Hit Points, although a Game Master can\
ignore this rule for an individual monster and treat\
it like a character.\
Hit Point Maximum of 0. A creature dies if its Hit\
Point maximum reaches 0. Certain effects drain life\
energy, reducing a creature’s Hit Point maximum.\
Massive Damage. When damage reduces a char-\
acter to 0 Hit Points and damage remains, the char-\
acter dies if the remainder equals or exceeds their\
Hit Point maximum. For example, if your character\
has a Hit Point maximum of 12, currently has 6 Hit\
Points, and takes 18 damage, the character drops to\
0 Hit Points, but 12 damage remains. The character\
then dies, since 12 equals their Hit Point maximum.\
