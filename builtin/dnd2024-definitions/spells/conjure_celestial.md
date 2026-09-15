---
splat_schema: 1
splat_id: 424a4d39-1fae-5d61-8690-67878afe728d
splat_kind: spell
splat_key: dnd2024:spells/conjure_celestial
splat_name: Conjure Celestial
splat_origin:
  key: dnd2024:reference/spell/conjure_celestial
splat_sources:
- title: SRD 5.2.1
  locator: pp. 118
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: conjuration
  classes:
  - cleric
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 90
splat_rule:
  mechanic: dnd2024:spells/conjure_celestial
  implementation: dnd2024:spell_conjure_celestial
  parameters:
    area:
      shape: cylinder
      radius_feet: 10
      height_feet: 40
      light: bright
      origin: point_in_range
    target:
      kind: creature
      count: each_visible_in_area
      visible_to_caster: true
    choice_per_target:
      chooser: caster
      options:
      - healing_light
      - searing_light
      optional_on_later_triggers: true
    options:
      healing_light:
        heal:
          dice:
            count: 4
            sides: 12
            add:
              binding: spellcasting_modifier
      searing_light:
        save:
          ability: dexterity
          dc:
            binding: save_dc
          failure:
            damage:
              amount:
                dice:
                  count: 6
                  sides: 12
                  add: 0
              type: radiant
          success:
            damage_fraction:
              numerator: 1
              denominator: 2
    triggers:
    - spell.cast_completed
    - area.moves_into_creature
    - area.enter
    - turn.end
    movement:
      event: caster.moves_on_own_turn
      maximum_feet: 30
    limit:
      effects_per_creature_per_turn: 1
    scaling:
      per_slot_above: 7
      healing_dice: 1
      damage_dice: 1
      sides: 12
splat_contracts:
- id: dnd2024:spell_conjure_celestial
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
    spellcasting_modifier:
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
  - area_effect
  - healing
  - saves
  - damage
  events:
  - spell.cast_completed
  - turn.end
  - area.enter
  - area.moved
  capabilities:
  - spellcasting
  - effects
  - damage
  - healing
  - saving_throws
  - spatial_areas
  - movement
  - perception
  - choices
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 05832e3846e252de3175fb165eb014eed4990a9bed1e073a75461308a544eb6d
  source_path: builtin/srd-5.2.1/spell/conjure_celestial.md
  pages:
  - 118
  engine_gaps: &id001
  - dnd2024:spell_conjure_celestial
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 0179ce573ce391fbf58afd91a0cc0cf6f83fa1c04b3d906f394a0f175307c237
splat_clauses:
- key: source_rules
  text: '# Conjure Celestial Level 7 Conjuration (Cleric) Casting Time: Action Range: 90 feet Components:
    V, S Duration: Concentration, up to 10 minutes You conjure a spirit from the Upper Planes, which manifests
    as a pillar of light in a 10-foot-radius, 40-foot-high Cylinder centered on a point within range.
    For each creature you can see in the Cylinder, choose which of these lights shines on it: Healing
    Light. The target regains Hit Points equal to 4d12 plus your spellcasting ability modifier. Searing
    Light. The target makes a Dexterity saving throw, taking 6d12 Radiant damage on a failed save or half
    as much damage on a successful one. Until the spell ends, Bright Light fills the Cylinder, and when
    you move on your turn, you can also move the Cylinder up to 30 feet. Whenever the Cylinder moves into
    the space of a creature you can see and whenever a creature you can see enters the Cylinder or ends
    its turn there, you can bathe it in one of the lights. A creature can be affected by this spell only
    once per turn. Using a Higher-Level Spell Slot. The healing and damage increase by 1d12 for each spell
    slot level above 7.'
  sha256: 9b455e6a1b9304d4a4051ec6299a5117bc3b21b82e5dbec2d465f7369ac76293
  status: contract_specified
splat_requires_mechanics: *id001
---

# Conjure Celestial

<!-- SRD page 118 -->

Level 7 Conjuration (Cleric)\
Casting Time: Action\
Range: 90 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You conjure a spirit from the Upper Planes, which\
manifests as a pillar of light in a 10-foot-radius,\
40-foot-high Cylinder centered on a point within\
range. For each creature you can see in the Cylinder,\
choose which of these lights shines on it:\
Healing Light. The target regains Hit Points equal\
to 4d12 plus your spellcasting ability modifier.\
Searing Light. The target makes a Dexterity saving\
throw, taking 6d12 Radiant damage on a failed\
save or half as much damage on a successful one.\
Until the spell ends, Bright Light fills the Cylinder,\
and when you move on your turn, you can also move\
the Cylinder up to 30 feet.\
Whenever the Cylinder moves into the space of a\
creature you can see and whenever a creature you\
can see enters the Cylinder or ends its turn there,\
you can bathe it in one of the lights. A creature can\
be affected by this spell only once per turn.\
Using a Higher-Level Spell Slot. The healing and\
damage increase by 1d12 for each spell slot level\
above 7.\

Original layout: [[../../srd-5.2.1/pages/118|page 118]].
