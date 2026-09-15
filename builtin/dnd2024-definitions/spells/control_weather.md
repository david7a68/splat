---
splat_schema: 1
splat_id: 6f5c8e82-bab2-52a5-8227-2d0780779f67
splat_kind: spell
splat_key: dnd2024:spells/control_weather
splat_name: Control Weather
splat_origin:
  key: dnd2024:reference/spell/control_weather
splat_sources:
- title: SRD 5.2.1
  locator: pp. 120
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: transmutation
  classes:
  - cleric
  - druid
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 8 hours
  casting_time: 10 minutes
  ritual: false
  range:
    mode: self
  material:
    description: burning incense
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: ac689ecfd3ba7eac0ae1c01d21dd15202fcffd7ddc4b8804c4d665cebeb2a807
  source_path: builtin/srd-5.2.1/spell/control_weather.md
  pages:
  - 120
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Control Weather Level 8 Transmutation (Cleric, Druid, Wizard) Casting Time: 10 minutes Range:
    Self Components: V, S, M (burning incense) Duration: Concentration, up to 8 hours You take control
    of the weather within 5 miles of you for the duration. You must be outdoors to cast this spell, and
    it ends early if you go indoors. When you cast the spell, you change the current weather conditions,
    which are determined by the GM. You can change precipitation, temperature, and wind. It takes 1d4
    × 10 minutes for the new conditions to take effect. Once they do so, you can change the conditions
    again. When the spell ends, the weather gradually returns to normal. When you change the weather conditions,
    find a current condition on the following tables and change its stage by one, up or down. When changing
    the wind, you can change its direction. Precipitation Stage Condition 1 Clear 2 Light clouds 3 Overcast
    or ground fog 4 Rain, hail, or snow 5 Torrential rain, driving hail, or blizzard Temperature Wind
    Stage Condition Stage Condition 1 Heat wave 1 Calm 2 Hot 2 Moderate wind 3 Warm 3 Strong wind 4 Cool
    4 Gale 5 Cold 5 Storm 6 Freezing'
  sha256: 1ac31d0af7fc6f434c5549ec68d8a70d2d997c03184af7b6da0ab6efe9f719f8
  status: needs_semantic_conversion
---

# Control Weather

<!-- SRD page 120 -->

Level 8 Transmutation (Cleric, Druid, Wizard)\
Casting Time: 10 minutes\
Range: Self\
Components: V, S, M (burning incense)\
Duration: Concentration, up to 8 hours\
You take control of the weather within 5 miles of\
you for the duration. You must be outdoors to cast\
this spell, and it ends early if you go indoors.\
When you cast the spell, you change the current\
weather conditions, which are determined by the\
GM. You can change precipitation, temperature,\
and wind. It takes 1d4 × 10 minutes for the new\
conditions to take effect. Once they do so, you can\
change the conditions again. When the spell ends,\
the weather gradually returns to normal.\
When you change the weather conditions, find\
a current condition on the following tables and\
change its stage by one, up or down. When changing\
the wind, you can change its direction.\
Precipitation\
Stage Condition\
1 Clear\
2 Light clouds\
3 Overcast or ground fog\
4 Rain, hail, or snow\
5 Torrential rain, driving hail, or blizzard\
Temperature Wind\
Stage Condition Stage Condition\
1 Heat wave 1 Calm\
2 Hot 2 Moderate wind\
3 Warm 3 Strong wind\
4 Cool 4 Gale\
5 Cold 5 Storm\
6 Freezing\

Original layout: [[../../srd-5.2.1/pages/120|page 120]].
