---
splat_schema: 1
splat_id: 980b488b-41db-5b44-a10f-103ef931e39e
splat_kind: spell
splat_key: dnd2024:spells/druidcraft
splat_name: Druidcraft
splat_origin:
  key: dnd2024:reference/spell/druidcraft
splat_sources:
- title: SRD 5.2.1
  locator: pp. 126
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: transmutation
  classes:
  - druid
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/druidcraft
  implementation: dnd2024:spell_druidcraft
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    choice:
      chooser: caster
      count: 1
      range_feet: 30
    options:
      weather_sensor:
        size: tiny
        harmless: true
        forecast_location: caster_location
        forecast_hours: 24
        duration_rounds: 1
        sensory_manifestation:
          chooser: caster
      bloom:
        instant: true
        target_choices:
        - flower
        - seed_pod
        - leaf_bud
        operations:
          flower: blossom
          seed_pod: open
          leaf_bud: bloom
      sensory_effect:
        harmless: true
        maximum_cube_side_feet: 5
        sensory_design:
          chooser: caster
      fire_play:
        operations:
        - light
        - snuff
        targets:
        - candle
        - torch
        - campfire
splat_contracts:
- id: dnd2024:spell_druidcraft
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    option:
      type: choice
      required: true
    origin:
      type: location
      required: true
    design:
      type: choice
      required: true
  outputs:
  - effects
  - objects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - illusions
  - world_objects
splat_conversion:
  status: specified
  category: spell
  source_sha256: ded6af1c6d3973f65e3ae3a9995c866510e574b89efb68327256e8c9c6d5137d
  source_path: builtin/srd-5.2.1/spell/druidcraft.md
  pages:
  - 126
  engine_gaps: &id001
  - dnd2024:spell_druidcraft
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 68886bef033371f6850b4e31e258889e833e8f7fbf685e9144764e3fffa84486
splat_clauses:
- key: source_rules
  text: '# Druidcraft Transmutation Cantrip (Druid) Casting Time: Action Range: 30 feet Components: V,
    S Duration: Instantaneous Whispering to the spirits of nature, you create one of the following effects
    within range. Weather Sensor. You create a Tiny, harmless sensory effect that predicts what the weather
    will be at your location for the next 24 hours. The effect might manifest as a golden orb for clear
    skies, a cloud for rain, falling snowflakes for snow, and so on. This effect persists for 1 round.
    Bloom. You instantly make a flower blossom, a seed pod open, or a leaf bud bloom. Sensory Effect.
    You create a harmless sensory effect, such as falling leaves, spectral dancing fairies, a gentle breeze,
    the sound of an animal, or the faint odor of skunk. The effect must fit in a 5-foot Cube. Fire Play.
    You light or snuff out a candle, a torch, or a campfire.'
  sha256: e076080a478ea079a170a6578df12df1560b6cb84b1e9893e1c08e5a34343011
  status: contract_specified
splat_requires_mechanics: *id001
---

# Druidcraft

<!-- SRD page 126 -->

Transmutation Cantrip (Druid)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Instantaneous\
Whispering to the spirits of nature, you create one\
of the following effects within range.\
Weather Sensor. You create a Tiny, harmless sen-\
sory effect that predicts what the weather will be at\
your location for the next 24 hours. The effect might\
manifest as a golden orb for clear skies, a cloud for\
rain, falling snowflakes for snow, and so on. This\
effect persists for 1 round.\
Bloom. You instantly make a flower blossom, a\
seed pod open, or a leaf bud bloom.\
Sensory Effect. You create a harmless sensory ef-\
fect, such as falling leaves, spectral dancing fairies,\
a gentle breeze, the sound of an animal, or the faint\
odor of skunk. The effect must fit in a 5-foot Cube.\
Fire Play. You light or snuff out a candle, a torch,\
or a campfire.\

Original layout: [[../../srd-5.2.1/pages/126|page 126]].
