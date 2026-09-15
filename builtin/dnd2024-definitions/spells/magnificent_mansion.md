---
splat_schema: 1
splat_id: 95e3592c-7670-5a21-82ef-7e2a93485251
splat_kind: spell
splat_key: dnd2024:spells/magnificent_mansion
splat_name: Magnificent Mansion
splat_origin:
  key: dnd2024:reference/spell/magnificent_mansion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 147
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: conjuration
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 24 hours
  casting_time: 1 minute
  ritual: false
  range:
    feet: 300
  material:
    description: a miniature door worth 15+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/magnificent_mansion
  implementation: dnd2024:spell_magnificent_mansion
  parameters:
    targeting:
      kind: doorway_location
      within_range: true
    doorway:
      width_feet: 5
      height_feet: 10
      duration: spell_duration
      entry_allowed: caster_and_creatures_designated_at_cast
      entry_requires: open
      open_close:
        economy: none
        caster_range_feet: 30
      closed: imperceptible
    dwelling:
      extradimensional: true
      air:
      - clean
      - fresh
      - warm
      floor_plan: caster_choice
      max_contiguous_cubes: 50
      cube_side_feet: 10
      furnishings: caster_choice
      banquet:
        courses: 9
        people: 100
      created_objects_removed: dissipate_into_smoke
    servants:
      count: 100
      appearance_and_attire: caster_choice
      near_transparent: true
      invulnerable: true
      obeys: caster_commands
      tasks: human_capable
      forbidden:
      - attack
      - directly_harm_creature
      - leave_dwelling
      serves: all_entrants
    end:
      expel: all_remaining_creatures_and_objects
      destination: nearest_unoccupied_spaces_to_entrance
    spell_lifetime:
      concentration: false
      maximum:
        value: 24
        unit: hour
splat_contracts:
- id: dnd2024:spell_magnificent_mansion
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      type: location_list
      required: true
    choices:
      type: choice
      required: true
    campaign:
      type: campaign
      required: true
  outputs:
  - effects
  - locations
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - spatial_effects
  - campaign_state
  - planar_travel
  - controlled_participants
splat_conversion:
  status: specified
  category: spell
  source_sha256: 51e2d311fbbda0ceebd867c9842f3eb1c5cde5efabcffac93dc25bca09c1c13f
  source_path: builtin/srd-5.2.1/spell/magnificent_mansion.md
  pages:
  - 147
  engine_gaps: &id001
  - dnd2024:spell_magnificent_mansion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 9f51d12d3759999dfa0ff4b3dc1f2793b6cdac6912bd16f79aa69e2e512957a1
splat_clauses:
- key: source_rules
  text: '# Magnificent Mansion Level 7 Conjuration (Bard, Wizard) Casting Time: 1 minute Range: 300 feet
    Components: V, S, M (a miniature door worth 15+ GP) Duration: 24 hours You conjure a shimmering door
    in range that lasts for the duration. The door leads to an extradimensional dwelling and is 5 feet
    wide and 10 feet tall. You and any creature you designate when you cast the spell can enter the extradimensional
    dwelling as long as the door remains open. You can open or close it (no action required) if you are
    within 30 feet of it. While closed, the door is imperceptible. Beyond the door is a magnificent foyer
    with numerous chambers beyond. The dwelling’s atmosphere is clean, fresh, and warm. You can create
    any floor plan you like for the dwelling, but it can’t exceed 50 contiguous 10-foot Cubes. The place
    is furnished and decorated as you choose. It contains sufficient food to serve a ninecourse banquet
    for up to 100 people. Furnishings and other objects created by this spell dissipate into smoke if
    removed from it. A staff of 100 near-transparent servants attends all who enter. You determine the
    appearance of these servants and their attire. They are invulnerable and obey your commands. Each
    servant can perform tasks that a human could perform, but they can’t attack or take any action that
    would directly harm another creature. Thus the servants can fetch things, clean, mend, fold clothes,
    light fires, serve food, pour wine, and so on. The servants can’t leave the dwelling. When the spell
    ends, any creatures or objects left inside the extradimensional space are expelled into the unoccupied
    spaces nearest to the entrance.'
  sha256: 114c6ef24a297644a199b0c45e9f5bc06922705e7028ce8c0d5bc0fb5ab3589b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magnificent Mansion

<!-- SRD page 147 -->

Level 7 Conjuration (Bard, Wizard)\
Casting Time: 1 minute\
Range: 300 feet\
Components: V, S, M (a miniature door worth 15+ GP)\
Duration: 24 hours\
You conjure a shimmering door in range that lasts\
for the duration. The door leads to an extradimen-\
sional dwelling and is 5 feet wide and 10 feet tall.\
You and any creature you designate when you cast\
the spell can enter the extradimensional dwelling\
as long as the door remains open. You can open or\
close it (no action required) if you are within 30 feet\
of it. While closed, the door is imperceptible.\
Beyond the door is a magnificent foyer with\
numerous chambers beyond. The dwelling’s atmo-\
sphere is clean, fresh, and warm.\
You can create any floor plan you like for the\
dwelling, but it can’t exceed 50 contiguous 10-foot\
Cubes. The place is furnished and decorated as you\
choose. It contains sufficient food to serve a nine-\
course banquet for up to 100 people. Furnishings\
and other objects created by this spell dissipate into\
smoke if removed from it.\
A staff of 100 near-transparent servants attends\
all who enter. You determine the appearance of\
these servants and their attire. They are invulner-\
able and obey your commands. Each servant can\
perform tasks that a human could perform, but they\
can’t attack or take any action that would directly\
harm another creature. Thus the servants can fetch\
things, clean, mend, fold clothes, light fires, serve\
food, pour wine, and so on. The servants can’t leave\
the dwelling.\
When the spell ends, any creatures or objects left\
inside the extradimensional space are expelled into\
the unoccupied spaces nearest to the entrance.\

Original layout: [[../../srd-5.2.1/pages/147|page 147]].
