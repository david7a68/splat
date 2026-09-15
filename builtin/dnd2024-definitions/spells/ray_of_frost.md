---
splat_schema: 1
splat_id: 4619c253-1d46-5a03-99bb-40d356164048
splat_kind: spell
splat_key: dnd2024:spells/ray_of_frost
splat_name: Ray of Frost
splat_origin:
  key: dnd2024:reference/spell/ray_of_frost
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: evocation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/ray_of_frost
  implementation: dnd2024:spell_ray_of_frost
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    target:
      kind: creature
      range_feet: 60
    attack:
      kind: ranged_spell_attack
    hit:
      damage:
        type: cold
        dice:
          sides: 8
          counts_by_character_level:
          - minimum_level: 1
            count: 1
          - minimum_level: 5
            count: 2
          - minimum_level: 11
            count: 3
          - minimum_level: 17
            count: 4
      speed_modifier_feet: -10
      speed_modifier_until:
        event: turn.start
        subject: caster
        occurrence: next
    miss: no_effect
splat_contracts:
- id: dnd2024:spell_ray_of_frost
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    target:
      type: participant
      required: true
    character_level:
      type: integer
      required: true
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  - turn.start
  capabilities:
  - spellcasting
  - effects
  - attacks
  - damage
splat_conversion:
  status: specified
  category: spell
  source_sha256: a59d1bb8318090fdac3b9a194de5cacab56f3fff2fb8067f5d57ba362e928f2f
  source_path: builtin/srd-5.2.1/spell/ray_of_frost.md
  pages:
  - 157
  engine_gaps: &id001
  - dnd2024:spell_ray_of_frost
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 1f4066b20e8c1496985079eacf8e0a7a73f70f8194bd87ca9bb309c54f821f58
splat_clauses:
- key: source_rules
  text: '# Ray of Frost Evocation Cantrip (Sorcerer, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S Duration: Instantaneous A frigid beam of blue-white light streaks toward a creature within range.
    Make a ranged spell attack against the target. On a hit, it takes 1d8 Cold damage, and its Speed is
    reduced by 10 feet until the start of your next turn. Cantrip Upgrade. The damage increases by 1d8
    when you reach levels 5 (2d8), 11 (3d8), and 17 (4d8).'
  sha256: f5535bc847efa1c25a6f79085c459cb874811956e7be0a74521491c42ec446db
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ray of Frost

<!-- SRD page 157 -->

Evocation Cantrip (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
A frigid beam of blue-white light streaks toward a\
creature within range. Make a ranged spell attack\
against the target. On a hit, it takes 1d8 Cold dam-\
age, and its Speed is reduced by 10 feet until the\
start of your next turn.\
Cantrip Upgrade. The damage increases by 1d8\
when you reach levels 5 (2d8), 11 (3d8), and 17\
(4d8).\

Original layout: [[../../srd-5.2.1/pages/157|page 157]].
