---
splat_schema: 1
splat_id: 4a838293-61fd-5c3a-a04a-156e714967c3
splat_kind: spell
splat_key: dnd2024:spells/produce_flame
splat_name: Produce Flame
splat_origin:
  key: dnd2024:reference/spell/produce_flame
splat_sources:
- title: SRD 5.2.1
  locator: pp. 156
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: conjuration
  classes:
  - druid
  components:
  - verbal
  - somatic
  concentration: false
  duration: 10 minutes
  casting_time: Bonus Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/produce_flame
  implementation: dnd2024:spell_produce_flame
  parameters:
    flame:
      location: caster_hand
      duration: spell_duration
      emits_heat: false
      ignites_objects: false
      bright_radius_feet: 20
      additional_dim_feet: 20
      end_on_recast: true
    hurl:
      cost: magic_action
      available_until_spell_ends: true
      target:
        kind: creature_or_object
        maximum_distance_feet: 60
      attack: ranged_spell
      hit:
        dice:
          count:
            by_caster_level:
            - minimum: 1
              count: 1
            - minimum: 5
              count: 2
            - minimum: 11
              count: 3
            - minimum: 17
              count: 4
          sides: 8
          add: 0
        type: fire
      ends_spell: false
splat_contracts:
- id: dnd2024:spell_produce_flame
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    caster_level:
      type: integer
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - attacks
  events:
  - spell.cast_completed
  - effect.ended
  - movement.completed
  - spell.cast_pending
  capabilities:
  - spellcasting
  - effects
  - dice
  - attacks
  - spatial_relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 2642a7a97c5168408bb1979fda4d445939c51f9f74aac6b38b8c442bdaa6732f
  source_path: builtin/srd-5.2.1/spell/produce_flame.md
  pages:
  - 156
  engine_gaps: &id001
  - dnd2024:spell_produce_flame
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: b01e6c06c3a468802abce670319004646930ac0ef78d40fa1f619ed0826e6f65
splat_clauses:
- key: source_rules
  text: '# Produce Flame Conjuration Cantrip (Druid) Casting Time: Bonus Action Range: Self Components:
    V, S Duration: 10 minutes A flickering flame appears in your hand and remains there for the duration.
    While there, the flame emits no heat and ignites nothing, and it sheds Bright Light in a 20-foot radius
    and Dim Light for an additional 20 feet. The spell ends if you cast it again. Until the spell ends,
    you can take a Magic action to hurl fire at a creature or an object within 60 feet of you. Make a
    ranged spell attack. On a hit, the target takes 1d8 Fire damage. Cantrip Upgrade. The damage increases
    by 1d8 when you reach levels 5 (2d8), 11 (3d8), and 17 (4d8).'
  sha256: 1ab3b6a86f1a6c72008d755defc5482e0ea3e36515eba5ecbee03ad462fa282d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Produce Flame

<!-- SRD page 156 -->

Conjuration Cantrip (Druid)\
Casting Time: Bonus Action\
Range: Self\
Components: V, S\
Duration: 10 minutes\
A flickering flame appears in your hand and re-\
mains there for the duration. While there, the flame\
emits no heat and ignites nothing, and it sheds\
Bright Light in a 20-foot radius and Dim Light for an\
additional 20 feet. The spell ends if you cast it again.\
Until the spell ends, you can take a Magic action to\
hurl fire at a creature or an object within 60 feet of\
you. Make a ranged spell attack. On a hit, the target\
takes 1d8 Fire damage.\
Cantrip Upgrade. The damage increases by 1d8\
when you reach levels 5 (2d8), 11 (3d8), and 17\
(4d8).\

Original layout: [[../../srd-5.2.1/pages/156|page 156]].
