---
splat_schema: 1
splat_id: f3e9921a-757f-5580-a3cd-2d5c4bfe400a
splat_kind: spell
splat_key: dnd2024:spells/speak_with_plants
splat_name: Speak with Plants
splat_origin:
  key: dnd2024:reference/spell/speak_with_plants
splat_sources:
- title: SRD 5.2.1
  locator: pp. 164
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: transmutation
  classes:
  - bard
  - druid
  - ranger
  components:
  - verbal
  - somatic
  concentration: false
  duration: 10 minutes
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/speak_with_plants
  implementation: dnd2024:spell_speak_with_plants
  parameters:
    targeting:
      kind: plants_in_area
      shape: emanation
      radius_feet: 30
      origin: caster_at_cast
      immobile: true
    animation:
      duration: spell_duration
      limited_sentience: true
      communicates_with: caster
      follows: simple_commands
      movable_parts:
      - branches
      - tendrils
      - stalks
      uproot_or_move_about: false
    questions:
      scope: events_in_spell_area
      history_days: 1
      topics:
      - passing_creatures
      - weather
      - other_circumstances
    terrain_choices:
    - from: plant_growth_difficult_terrain
      to: ordinary
    - from: ordinary_with_plants
      to: difficult_terrain
    terrain_duration: spell_duration
    plant_creatures:
      communication: shared_language_equivalent
    spell_lifetime:
      concentration: false
      maximum:
        value: 10
        unit: minute
splat_contracts:
- id: dnd2024:spell_speak_with_plants
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    choices:
      type: choice
      required: true
    campaign:
      type: campaign
      required: true
  outputs:
  - effects
  - information
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - spatial_effects
  - campaign_state
  - controlled_participants
splat_conversion:
  status: specified
  category: spell
  source_sha256: 8b3f0f86a783dd5bc3ab1204e75f941c3ec50d5feb65c41b22c3262fb812fbae
  source_path: builtin/srd-5.2.1/spell/speak_with_plants.md
  pages:
  - 164
  engine_gaps: &id001
  - dnd2024:spell_speak_with_plants
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 9d1f8f31e41b3f4960cdb445ba0b5bec68336f29f70c1541fd253304db8480c9
splat_clauses:
- key: source_rules
  text: '# Speak with Plants Level 3 Transmutation (Bard, Druid, Ranger) Casting Time: Action Range: Self
    Components: V, S Duration: 10 minutes You imbue plants in an immobile 30-foot Emanation with limited
    sentience and animation, giving them the ability to communicate with you and follow your simple commands.
    You can question plants about events in the spell’s area within the past day, gaining information
    about creatures that have passed, weather, and other circumstances. You can also turn Difficult Terrain
    caused by plant growth (such as thickets and undergrowth) into ordinary terrain that lasts for the
    duration. Or you can turn ordinary terrain where plants are present into Difficult Terrain that lasts
    for the duration. The spell doesn’t enable plants to uproot themselves and move about, but they can
    move their branches, tendrils, and stalks for you. If a Plant creature is in the area, you can communicate
    with it as if you shared a common language.'
  sha256: cc008c45b01a7511280b4446eab90e013028798a0f6c6f6e73ca3e506e2c963e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Speak with Plants

<!-- SRD page 164 -->

Level 3 Transmutation (Bard, Druid, Ranger)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: 10 minutes\
You imbue plants in an immobile 30-foot Emanation\
with limited sentience and animation, giving them\
the ability to communicate with you and follow your\
simple commands. You can question plants about\
events in the spell’s area within the past day, gain-\
ing information about creatures that have passed,\
weather, and other circumstances.\
You can also turn Difficult Terrain caused by plant\
growth (such as thickets and undergrowth) into\
ordinary terrain that lasts for the duration. Or you\
can turn ordinary terrain where plants are present\
into Difficult Terrain that lasts for the duration.\
The spell doesn’t enable plants to uproot them-\
selves and move about, but they can move their\
branches, tendrils, and stalks for you.\
If a Plant creature is in the area, you can commu-\
nicate with it as if you shared a common language.\

Original layout: [[../../srd-5.2.1/pages/164|page 164]].
