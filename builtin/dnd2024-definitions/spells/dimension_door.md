---
splat_schema: 1
splat_id: 92805162-dfe9-551f-bc67-dba7847f5e2b
splat_kind: spell
splat_key: dnd2024:spells/dimension_door
splat_name: Dimension Door
splat_origin:
  key: dnd2024:reference/spell/dimension_door
splat_sources:
- title: SRD 5.2.1
  locator: pp. 123, 124
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: conjuration
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 500
splat_rule:
  mechanic: dnd2024:spells/dimension_door
  implementation: dnd2024:spell_dimension_door
  parameters:
    teleport:
      subject: caster
      destination_choice:
      - seen
      - visualized
      - distance_and_direction
      range_feet: 500
      exact_destination: true
    companion:
      maximum: 1
      willing: true
      initial_distance_feet: 5
      destination_distance_from_caster_feet: 5
    failure:
      if_any_destination:
      - creature_occupied
      - completely_object_filled
      teleport_fails_for_all: true
      damage_to_all_travelers:
        amount:
          dice:
            count: 4
            sides: 6
            add: 0
        type: force
splat_contracts:
- id: dnd2024:spell_dimension_door
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
  - teleport_result
  - damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - movement
  - inventory
  - choices
splat_conversion:
  status: specified
  category: spell
  source_sha256: f4f6f7440d22372cb485c7e965273bb5743975281d044d000f1af313ec2e5fff
  source_path: builtin/srd-5.2.1/spell/dimension_door.md
  pages:
  - 123
  - 124
  engine_gaps: &id001
  - dnd2024:spell_dimension_door
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 4d5a5d6db7d51cf2c0fa253e94de3681e8c1523d765b1d1fd0d4d516eebc4442
splat_clauses:
- key: source_rules
  text: '# Dimension Door Level 4 Conjuration (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    500 feet Components: V Duration: Instantaneous You teleport to a location within range. You arrive
    at exactly the spot desired. It can be a place you can see, one you can visualize, or one you can
    describe by stating distance and direction, such as “200 feet straight downward” or “300 feet upward
    to the northwest at a 45-degree angle.” You can also teleport one willing creature. The creature must
    be within 5 feet of you when you teleport, and it teleports to a space within 5 feet of your destination
    space. If you, the other creature, or both would arrive in a space occupied by a creature or completely
    filled by one or more objects, you and any creature traveling with you each take 4d6 Force damage,
    and the teleportation fails.'
  sha256: 12b31bb580bb13960d1dedc85054c1e36d51b217c87505c1f3cd41b21c1ce29b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dimension Door

<!-- SRD page 123 -->

Level 4 Conjuration (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 500 feet\
Components: V\
Duration: Instantaneous\
You teleport to a location within range. You arrive\
at exactly the spot desired. It can be a place you can\
see, one you can visualize, or one you can describe\
by stating distance and direction, such as “200 feet\
straight downward” or “300 feet upward to the\
northwest at a 45-degree angle.”\
You can also teleport one willing creature. The\
creature must be within 5 feet of you when you tele-\
port, and it teleports to a space within 5 feet of your\
destination space.\

<!-- SRD page 124 -->

If you, the other creature, or both would arrive in\
a space occupied by a creature or completely filled\
by one or more objects, you and any creature trav-\
eling with you each take 4d6 Force damage, and the\
teleportation fails.\

Original layout: [[../../srd-5.2.1/pages/123|page 123]], [[../../srd-5.2.1/pages/124|page 124]].
