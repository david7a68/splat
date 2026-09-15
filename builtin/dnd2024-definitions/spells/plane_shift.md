---
splat_schema: 1
splat_id: 3e7549e0-4480-5966-9405-1d87d9f3e672
splat_kind: spell
splat_key: dnd2024:spells/plane_shift
splat_name: Plane Shift
splat_origin:
  key: dnd2024:reference/spell/plane_shift
splat_sources:
- title: SRD 5.2.1
  locator: pp. 153
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: conjuration
  classes:
  - cleric
  - druid
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a forked, metal rod worth 250+ GP and attuned to a plane of existence
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/plane_shift
  implementation: dnd2024:spell_plane_shift
  parameters:
    targets:
      caster: true
      additional_creatures_maximum: 8
      willing: true
      link_hands_in_circle: true
    transport:
      different_plane: true
      plane_matches_attuned_material: true
      destination_modes:
        general:
          caster_specifies: general_terms
          arrival_chooser: gm
          arrival_constraint: in_or_near_destination
        teleportation_circle:
          requires_known_sigil_sequence: true
          destination: circle_on_other_plane
          overflow: closest_unoccupied_spaces_next_to_circle
splat_contracts:
- id: dnd2024:spell_plane_shift
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
    choices:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - spell.cast_completed
  - movement.completed
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5f120fea5c233fafc6842c563d511a099267ca4aaddc1c7b5aaea0809239efbf
  source_path: builtin/srd-5.2.1/spell/plane_shift.md
  pages:
  - 153
  engine_gaps: &id001
  - dnd2024:spell_plane_shift
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: eb88dfb8b93b5f4e0b7043defbd430f32cda7775e7d2acad623e8402acf301e9
splat_clauses:
- key: source_rules
  text: '# Plane Shift Level 7 Conjuration (Cleric, Druid, Sorcerer, Warlock, Wizard) Casting Time: Action
    Range: Touch Components: V, S, M (a forked, metal rod worth 250+ GP and attuned to a plane of existence)
    Duration: Instantaneous You and up to eight willing creatures who link hands in a circle are transported
    to a different plane of existence. You can specify a target destination in general terms, such as
    a specific city on the Elemental Plane of Fire or palace on the second level of the Nine Hells, and
    you appear in or near that destination, as determined by the GM. Alternatively, if you know the sigil
    sequence of a teleportation circle on another plane of existence, this spell can take you to that
    circle. If the teleportation circle is too small to hold all the creatures you transported, they appear
    in the closest unoccupied spaces next to the circle.'
  sha256: d64e4162bb4f5b70fa3c78a5385e10acb692f7638af6f3bdea47eeea53ee6710
  status: contract_specified
splat_requires_mechanics: *id001
---

# Plane Shift

<!-- SRD page 153 -->

Level 7 Conjuration (Cleric, Druid, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a forked, metal rod worth 250+\
GP and attuned to a plane of existence)\
Duration: Instantaneous\
You and up to eight willing creatures who link\
hands in a circle are transported to a different plane\
of existence. You can specify a target destination in\
general terms, such as a specific city on the Elemen-\
tal Plane of Fire or palace on the second level of the\
Nine Hells, and you appear in or near that destina-\
tion, as determined by the GM.\
Alternatively, if you know the sigil sequence of a\
teleportation circle on another plane of existence,\
this spell can take you to that circle. If the teleporta-\
tion circle is too small to hold all the creatures you\
transported, they appear in the closest unoccupied\
spaces next to the circle.\

Original layout: [[../../srd-5.2.1/pages/153|page 153]].
