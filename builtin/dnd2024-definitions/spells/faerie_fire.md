---
splat_schema: 1
splat_id: 5eeefa95-08af-588f-bed1-c359739b95ce
splat_kind: spell
splat_key: dnd2024:spells/faerie_fire
splat_name: Faerie Fire
splat_origin:
  key: dnd2024:reference/spell/faerie_fire
splat_sources:
- title: SRD 5.2.1
  locator: pp. 129
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: evocation
  classes:
  - bard
  - druid
  components:
  - verbal
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/faerie_fire
  implementation: dnd2024:spell_faerie_fire
  parameters:
    area:
      shape: cube
      side_feet: 20
    color_choice:
    - blue
    - green
    - violet
    objects:
      automatic: true
    creatures:
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          outlined: true
        success:
          effect: none
    outlined:
      dim_light_radius_feet: 10
      invisible_benefit: false
      attack_rolls_against: advantage_if_attacker_sees_target
splat_contracts:
- id: dnd2024:spell_faerie_fire
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
      type: integer
      required: true
    spell_attack_modifier:
      required: false
      type: integer
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
  - saves
  - outlined_effects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - saving_throws
  - attacks
  - spatial_areas
  - perception
  - inventory
  - choices
splat_conversion:
  status: specified
  category: spell
  source_sha256: c1cf5a11ac10c982047c847d9f879ce33534bd8c643de7630ffa377051a772ad
  source_path: builtin/srd-5.2.1/spell/faerie_fire.md
  pages:
  - 129
  engine_gaps: &id001
  - dnd2024:spell_faerie_fire
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: ba650e59d064cb7031956471ad2fa6c9eedcf1de595bce79e365df5eadac6697
splat_clauses:
- key: source_rules
  text: '# Faerie Fire Level 1 Evocation (Bard, Druid) Casting Time: Action Range: 60 feet Components:
    V Duration: Concentration, up to 1 minute Objects in a 20-foot Cube within range are outlined in blue,
    green, or violet light (your choice). Each creature in the Cube is also outlined if it fails a Dexterity
    saving throw. For the duration, objects and affected creatures shed Dim Light in a 10-foot radius
    and can’t benefit from the Invisible condition. Attack rolls against an affected creature or object
    have Advantage if the attacker can see it.'
  sha256: 3251bcf26312351bba6c9aedacf405080bc3eb11dcce3b7a3669ab700e68d507
  status: contract_specified
splat_requires_mechanics: *id001
---

# Faerie Fire

<!-- SRD page 129 -->

Level 1 Evocation (Bard, Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Concentration, up to 1 minute\
Objects in a 20-foot Cube within range are outlined\
in blue, green, or violet light (your choice). Each\
creature in the Cube is also outlined if it fails a Dex-\
terity saving throw. For the duration, objects and\
affected creatures shed Dim Light in a 10-foot ra-\
dius and can’t benefit from the Invisible condition.\
Attack rolls against an affected creature or object\
have Advantage if the attacker can see it.\

Original layout: [[../../srd-5.2.1/pages/129|page 129]].
