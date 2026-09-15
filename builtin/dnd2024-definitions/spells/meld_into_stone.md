---
splat_schema: 1
splat_id: 25fed2c9-1bb0-556c-a7a6-58c3d2b8fe78
splat_kind: spell
splat_key: dnd2024:spells/meld_into_stone
splat_name: Meld into Stone
splat_origin:
  key: dnd2024:reference/spell/meld_into_stone
splat_sources:
- title: SRD 5.2.1
  locator: pp. 148
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: transmutation
  classes:
  - cleric
  - druid
  - ranger
  components:
  - verbal
  - somatic
  concentration: false
  duration: 8 hours
  casting_time: Action or Ritual
  ritual: true
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/meld_into_stone
  implementation: dnd2024:spell_meld_into_stone
  parameters:
    stone:
      kind:
      - object
      - surface
      touch: true
      must_fully_contain_body: true
    merge:
      subject: caster_and_equipment
      nonmagical_detection: false
      outside_vision: false
      outside_hearing_perception_checks: disadvantage
      time_awareness: true
      cast_spells_on_self: true
      movement: false
    exit:
      movement_cost_feet: 5
      location: entry_point
      ends_spell: true
    stone_damage:
      minor: no_harm
      partial_destruction_or_shape_change_no_longer_fits:
        expel: true
        damage:
          amount:
            dice:
              count: 6
              sides: 6
              add: 0
          type: force
      complete_destruction_or_transmutation:
        expel: true
        damage:
          amount: 50
          type: force
    expulsion:
      destination: nearest_unoccupied_to_entry
      condition: prone
splat_contracts:
- id: dnd2024:spell_meld_into_stone
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
  - merge_effect
  - expulsion
  - damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - conditions
  - movement
  - perception
  - inventory
  - checks
splat_conversion:
  status: specified
  category: spell
  source_sha256: fbaf60f68b2ff0296578a09f62e294d3222a6c26e49d6f831751383339ab7351
  source_path: builtin/srd-5.2.1/spell/meld_into_stone.md
  pages:
  - 148
  engine_gaps: &id001
  - dnd2024:spell_meld_into_stone
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 5f32563cfb3338ff2b0b859e25dd4b49c758e0c3b22853523f2353671de72c40
splat_clauses:
- key: source_rules
  text: '# Meld into Stone Level 3 Transmutation (Cleric, Druid, Ranger) Casting Time: Action or Ritual
    Range: Touch Components: V, S Duration: 8 hours You step into a stone object or surface large enough
    to fully contain your body, merging yourself and your equipment with the stone for the duration. You
    must touch the stone to do so. Nothing of your presence remains visible or otherwise detectable by
    nonmagical senses. While merged with the stone, you can’t see what occurs outside it, and any Wisdom
    (Perception) checks you make to hear sounds outside it are made with Disadvantage. You remain aware
    of the passage of time and can cast spells on yourself while merged in the stone. You can use 5 feet
    of movement to leave the stone where you entered it, which ends the spell. You otherwise can’t move.
    Minor physical damage to the stone doesn’t harm you, but its partial destruction or a change in its
    shape (to the extent that you no longer fit within it) expels you and deals 6d6 Force damage to you.
    The stone’s complete destruction (or transmutation into a different substance) expels you and deals
    50 Force damage to you. If expelled, you move into an unoccupied space closest to where you first
    entered and have the Prone condition.'
  sha256: 0c90b308f6f88693cd5b23256beceb48c5abe9b4326a576861e271998422c0e2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Meld into Stone

<!-- SRD page 148 -->

Level 3 Transmutation (Cleric, Druid, Ranger)\
Casting Time: Action or Ritual\
Range: Touch\
Components: V, S\
Duration: 8 hours\
You step into a stone object or surface large enough\
to fully contain your body, merging yourself and\
your equipment with the stone for the duration.\
You must touch the stone to do so. Nothing of your\
presence remains visible or otherwise detectable by\
nonmagical senses.\
While merged with the stone, you can’t see what\
occurs outside it, and any Wisdom (Perception)\
checks you make to hear sounds outside it are made\
with Disadvantage. You remain aware of the pas-\
sage of time and can cast spells on yourself while\
merged in the stone. You can use 5 feet of movement\
to leave the stone where you entered it, which ends\
the spell. You otherwise can’t move.\
Minor physical damage to the stone doesn’t harm\
you, but its partial destruction or a change in its\
shape (to the extent that you no longer fit within it)\
expels you and deals 6d6 Force damage to you. The\
stone’s complete destruction (or transmutation into\
a different substance) expels you and deals 50 Force\
damage to you. If expelled, you move into an unoc-\
cupied space closest to where you first entered and\
have the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/148|page 148]].
