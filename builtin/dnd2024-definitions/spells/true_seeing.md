---
splat_schema: 1
splat_id: 97c986c3-b654-5430-ab0f-97c7ecf40a31
splat_kind: spell
splat_key: dnd2024:spells/true_seeing
splat_name: True Seeing
splat_origin:
  key: dnd2024:reference/spell/true_seeing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 171
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: divination
  classes:
  - bard
  - cleric
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: mushroom powder worth 25+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/true_seeing
  implementation: dnd2024:spell_true_seeing
  parameters:
    targeting:
      kind: creature
      count: 1
      willing: true
      touch: true
    sense:
      kind: truesight
      range_feet: 120
      duration: spell_duration
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_true_seeing
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
  outputs:
  - effects
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 3a4cda6445c981ef97e55bf30d517c47b1977f11343d6bc6f7f5bd23a4806412
  source_path: builtin/srd-5.2.1/spell/true_seeing.md
  pages:
  - 171
  engine_gaps: &id001
  - dnd2024:spell_true_seeing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 2011dbfae47a68103369bef69c4c9a3ece57b273652cf866fb1127cd41ad9ef2
splat_clauses:
- key: source_rules
  text: '# True Seeing Level 6 Divination (Bard, Cleric, Sorcerer, Warlock, Wizard) Casting Time: Action
    Range: Touch Components: V, S, M (mushroom powder worth 25+ GP, which the spell consumes) Duration:
    1 hour For the duration, the willing creature you touch has Truesight with a range of 120 feet.'
  sha256: 01a96ed61039183f7afe1a18a2fb13ca42814d21ef80756358977a955d13ee1e
  status: contract_specified
splat_requires_mechanics: *id001
---

# True Seeing

<!-- SRD page 171 -->

Level 6 Divination (Bard, Cleric, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (mushroom powder worth 25+\
GP, which the spell consumes)\
Duration: 1 hour\
For the duration, the willing creature you touch has\
Truesight with a range of 120 feet.\

Original layout: [[../../srd-5.2.1/pages/171|page 171]].
