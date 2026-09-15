---
splat_schema: 1
splat_id: a3ca0881-5df9-56b7-866e-7a7ac7f348f9
splat_kind: spell
splat_key: dnd2024:spells/arcane_eye
splat_name: Arcane Eye
splat_origin:
  key: dnd2024:reference/spell/arcane_eye
splat_sources:
- title: SRD 5.2.1
  locator: pp. 110
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: divination
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a bit of bat fur
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/arcane_eye
  implementation: dnd2024:spell_arcane_eye
  parameters:
    targeting:
      kind: location
      within_range: true
    sensor:
      invisible: true
      invulnerable: true
      hovers: true
      perception: visual
      transmission: mental_to_caster
      directions: all
      darkvision_feet: 30
      duration: spell_duration
    move:
      economy: bonus_action
      max_feet: 30
      direction: any
      solid_barrier_blocks: true
      minimum_opening_diameter_inches: 1
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_arcane_eye
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      type: location_list
      required: true
  outputs:
  - effects
  - locations
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - spatial_effects
  - perception
splat_conversion:
  status: specified
  category: spell
  source_sha256: df83d8b06738eefded59442bde3016d84dc7d9df59fe52bd6dd295294608a30d
  source_path: builtin/srd-5.2.1/spell/arcane_eye.md
  pages:
  - 110
  engine_gaps: &id001
  - dnd2024:spell_arcane_eye
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 7aaeb556f70a76d5df108e3a580a96c4c57d9b5b6c209d4084dc2c35688f0e6c
splat_clauses:
- key: source_rules
  text: '# Arcane Eye Level 4 Divination (Wizard) Casting Time: Action Range: 30 feet Components: V, S,
    M (a bit of bat fur) Duration: Concentration, up to 1 hour You create an Invisible, invulnerable eye
    within range that hovers for the duration. You mentally receive visual information from the eye, which
    can see in every direction. It also has Darkvision with a range of 30 feet. As a Bonus Action, you
    can move the eye up to 30 feet in any direction. A solid barrier blocks the eye’s movement, but the
    eye can pass through an opening as small as 1 inch in diameter.'
  sha256: a914d221d7507ca8a789c56dbd25653f0855d4ee5e2eb0ee0d792190cc25a68e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Arcane Eye

<!-- SRD page 110 -->

Level 4 Divination (Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a bit of bat fur)\
Duration: Concentration, up to 1 hour\
You create an Invisible, invulnerable eye within\
range that hovers for the duration. You mentally\
receive visual information from the eye, which can\
see in every direction. It also has Darkvision with a\
range of 30 feet.\
As a Bonus Action, you can move the eye up to 30\
feet in any direction. A solid barrier blocks the eye’s\
movement, but the eye can pass through an opening\
as small as 1 inch in diameter.\

Original layout: [[../../srd-5.2.1/pages/110|page 110]].
