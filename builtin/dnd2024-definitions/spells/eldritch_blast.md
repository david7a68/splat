---
splat_schema: 1
splat_id: 955fbc1e-8225-57c6-89f5-0c4bdc89b22a
splat_kind: spell
splat_key: dnd2024:spells/eldritch_blast
splat_name: Eldritch Blast
splat_origin:
  key: dnd2024:reference/spell/eldritch_blast
splat_sources:
- title: SRD 5.2.1
  locator: pp. 127
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: evocation
  classes:
  - warlock
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/eldritch_blast
  implementation: dnd2024:spell_eldritch_blast
  parameters:
    targeting:
      kind: creature_or_object
      within_range: true
      per_beam: true
      same_or_different: true
    beams:
      character_level_count:
        '1': 1
        '5': 2
        '11': 3
        '17': 4
      independent_attacks: true
    attack:
      kind: ranged_spell
      hit:
        damage:
          dice: 1d10
          type: force
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_eldritch_blast
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    objects:
      required: false
      type: object_list
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
splat_conversion:
  status: specified
  category: spell
  source_sha256: b25abdda311e65c7f64e11c0c3e9751a01b66a517cc4f4acc2b8c58bce7b6780
  source_path: builtin/srd-5.2.1/spell/eldritch_blast.md
  pages:
  - 127
  engine_gaps: &id001
  - dnd2024:spell_eldritch_blast
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: a32717a62a249963c18d36ff4185e0bbb48052c95821915a8f8659ebd6cb119e
splat_clauses:
- key: source_rules
  text: '# Eldritch Blast Evocation Cantrip (Warlock) Casting Time: Action Range: 120 feet Components:
    V, S Duration: Instantaneous You hurl a beam of crackling energy. Make a ranged spell attack against
    one creature or object in range. On a hit, the target takes 1d10 Force damage. Cantrip Upgrade. The
    spell creates two beams at level 5, three beams at level 11, and four beams at level 17. You can direct
    the beams at the same target or at different ones. Make a separate attack roll for each beam.'
  sha256: 59bbb87ca5221dac073fbc2db102993262e2d7e31ac0d2505c7841a63c49da4b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Eldritch Blast

<!-- SRD page 127 -->

Evocation Cantrip (Warlock)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous\
You hurl a beam of crackling energy. Make a ranged\
spell attack against one creature or object in range.\
On a hit, the target takes 1d10 Force damage.\
Cantrip Upgrade. The spell creates two beams at\
level 5, three beams at level 11, and four beams at\
level 17. You can direct the beams at the same target\
or at different ones. Make a separate attack roll for\
each beam.\

Original layout: [[../../srd-5.2.1/pages/127|page 127]].
