---
splat_schema: 1
splat_id: d7a81f95-2075-5d5b-b785-3882892d079f
splat_kind: spell
splat_key: dnd2024:spells/conjure_fey
splat_name: Conjure Fey
splat_origin:
  key: dnd2024:reference/spell/conjure_fey
splat_sources:
- title: SRD 5.2.1
  locator: pp. 118
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: conjuration
  classes:
  - druid
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/conjure_fey
  implementation: dnd2024:spell_conjure_fey
  parameters:
    creation:
      kind: spirit
      size: medium
      origin_plane: feywild
      appearance_choice: fey_creature
      space:
        unoccupied: true
        visible: true
        within_spell_range: true
      duration: spell_duration
    attack:
      optional: true
      target:
        kind: creature
        distance_from_spirit_maximum_feet: 5
      kind: melee_spell
      hit:
        damage:
          dice:
            count:
              op: add
              args:
              - 3
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 6
            sides: 12
            add:
              binding: spellcasting_modifier
          type: psychic
        condition: frightened
        fear_sources:
        - caster
        - spirit
        until: caster_next_turn_start
    activation:
      initial: spirit_appears
      later:
        cost: bonus_action
        'on': caster_turn
        teleport_before_attack:
          maximum_feet: 30
          destination_unoccupied: true
          destination_visible_to_caster: true
splat_contracts:
- id: dnd2024:spell_conjure_fey
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
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    spellcasting_modifier:
      type: integer
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - damage
  - attacks
  - participants
  events:
  - spell.cast_completed
  - movement.completed
  - turn.start
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - attacks
  - spatial_relationships
  - conditions
  - participant_lifecycle
splat_conversion:
  status: specified
  category: spell
  source_sha256: 49c4b35e0f90a6717a38674e16684824d4506f0727175764ac200c5095f7854e
  source_path: builtin/srd-5.2.1/spell/conjure_fey.md
  pages:
  - 118
  engine_gaps: &id001
  - dnd2024:spell_conjure_fey
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 5ff9831ff5085c3a33c51dbab4849358dfa87e440e7c47bf9da8362affe435fe
splat_clauses:
- key: source_rules
  text: '# Conjure Fey Level 6 Conjuration (Druid) Casting Time: Action Range: 60 feet Components: V,
    S Duration: Concentration, up to 10 minutes You conjure a Medium spirit from the Feywild in an unoccupied
    space you can see within range. The spirit lasts for the duration, and it looks like a Fey creature
    of your choice. When the spirit appears, you can make one melee spell attack against a creature within
    5 feet of it. On a hit, the target takes Psychic damage equal to 3d12 plus your spellcasting ability
    modifier, and the target has the Frightened condition until the start of your next turn, with both
    you and the spirit as the source of the fear. As a Bonus Action on your later turns, you can teleport
    the spirit to an unoccupied space you can see within 30 feet of the space it left and make the attack
    against a creature within 5 feet of it. Using a Higher-Level Spell Slot. The damage increases by 1d12
    for each spell slot level above 6.'
  sha256: 7bc4610ba2edbf5c12155e1b2b13b982f266dcb4f9aa25c4d016b46dd4099f99
  status: contract_specified
splat_requires_mechanics: *id001
---

# Conjure Fey

<!-- SRD page 118 -->

Level 6 Conjuration (Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You conjure a Medium spirit from the Feywild in\
an unoccupied space you can see within range.\
The spirit lasts for the duration, and it looks like\
a Fey creature of your choice. When the spirit ap-\
pears, you can make one melee spell attack against\
a creature within 5 feet of it. On a hit, the target\
takes Psychic damage equal to 3d12 plus your\
spellcasting ability modifier, and the target has the\
Frightened condition until the start of your next\
turn, with both you and the spirit as the source of\
the fear.\
As a Bonus Action on your later turns, you can\
teleport the spirit to an unoccupied space you can\
see within 30 feet of the space it left and make the\
attack against a creature within 5 feet of it.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d12 for each spell slot level above 6.\

Original layout: [[../../srd-5.2.1/pages/118|page 118]].
