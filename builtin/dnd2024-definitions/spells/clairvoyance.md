---
splat_schema: 1
splat_id: c26b332e-3a61-5f59-9596-b170d4d63562
splat_kind: spell
splat_key: dnd2024:spells/clairvoyance
splat_name: Clairvoyance
splat_origin:
  key: dnd2024:reference/spell/clairvoyance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 115
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: divination
  classes:
  - bard
  - cleric
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: 10 minutes
  ritual: false
  range:
    mode: 1 mile
  material:
    description: a focus worth 100+ GP, either a jeweled horn for hearing or a glass eye for seeing
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/clairvoyance
  implementation: dnd2024:spell_clairvoyance
  parameters:
    sensor:
      location_choice:
      - visited_or_seen
      - unfamiliar_obvious_location
      invisible: true
      intangible: true
      invulnerable: true
      stationary: true
      visible_appearance: fist_sized_luminous_orb
    sense:
      choice:
      - seeing
      - hearing
      perception_origin: sensor
      switch_action: bonus_action
    focus:
      seeing: glass_eye
      hearing: jeweled_horn
      minimum_gp: 100
splat_contracts:
- id: dnd2024:spell_clairvoyance
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - sensor
  - perception
  - granted_action
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - perception
  - choices
splat_conversion:
  status: specified
  category: spell
  source_sha256: ffc9b1450d57eec003a7d09e45808ef2096fff9d8bf3f9a066c58af82617089b
  source_path: builtin/srd-5.2.1/spell/clairvoyance.md
  pages:
  - 115
  engine_gaps: &id001
  - dnd2024:spell_clairvoyance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 50460de45aed17fa4b234816a074a79d9a86100e4a7ebdd3a8953a9ec86e0677
splat_clauses:
- key: source_rules
  text: '# Clairvoyance Level 3 Divination (Bard, Cleric, Sorcerer, Wizard) Casting Time: 10 minutes Range:
    1 mile Components: V, S, M (a focus worth 100+ GP, either a jeweled horn for hearing or a glass eye
    for seeing) Duration: Concentration, up to 10 minutes You create an Invisible sensor within range
    in a location familiar to you (a place you have visited or seen before) or in an obvious location
    that is unfamiliar to you (such as behind a door, around a corner, or in a grove of trees). The intangible,
    invulnerable sensor remains in place for the duration. When you cast the spell, choose seeing or hearing.
    You can use the chosen sense through the sensor as if you were in its space. As a Bonus Action, you
    can switch between seeing and hearing. A creature that sees the sensor (such as a creature benefiting
    from See Invisibility or Truesight) sees a luminous orb about the size of your fist.'
  sha256: a7c422f17592626b53f05194467de6f580397c5e945e00054d477d1511ef7ee9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Clairvoyance

<!-- SRD page 115 -->

Level 3 Divination (Bard, Cleric, Sorcerer, Wizard)\
Casting Time: 10 minutes\
Range: 1 mile\
Components: V, S, M (a focus worth 100+ GP, either a\
jeweled horn for hearing or a glass eye for seeing)\
Duration: Concentration, up to 10 minutes\
You create an Invisible sensor within range in a\
location familiar to you (a place you have visited\
or seen before) or in an obvious location that is\
unfamiliar to you (such as behind a door, around a\
corner, or in a grove of trees). The intangible, invul-\
nerable sensor remains in place for the duration.\
When you cast the spell, choose seeing or hearing.\
You can use the chosen sense through the sensor as\
if you were in its space. As a Bonus Action, you can\
switch between seeing and hearing.\
A creature that sees the sensor (such as a creature\
benefiting from See Invisibility or Truesight) sees a\
luminous orb about the size of your fist.\

Original layout: [[../../srd-5.2.1/pages/115|page 115]].
