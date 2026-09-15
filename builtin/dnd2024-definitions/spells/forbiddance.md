---
splat_schema: 1
splat_id: b89c1b71-68a2-5dae-b127-22e491d91252
splat_kind: spell
splat_key: dnd2024:spells/forbiddance
splat_name: Forbiddance
splat_origin:
  key: dnd2024:reference/spell/forbiddance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 133
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: abjuration
  classes:
  - cleric
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 day
  casting_time: 10 minutes or Ritual
  ritual: true
  range:
    mode: touch
  material:
    description: ruby dust worth 1,000+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/forbiddance
  implementation: dnd2024:spell_forbiddance
  parameters:
    targeting:
      kind: floor_area
      touch: true
      max_square_feet: 40000
      height_feet: 30
      cannot_overlap: other_forbiddance
    travel_block:
      entry_modes:
      - teleportation
      - portals
      - planar_travel
      includes:
      - astral
      - ethereal
      - feywild
      - shadowfell
      - plane_shift
      duration: spell_duration
    damage:
      types_choice:
        minimum: 1
        choices:
        - aberration
        - celestial
        - elemental
        - fey
        - fiend
        - undead
      events:
      - first_area_entry_each_turn
      - target_turn_end_inside
      dice: 5d10
      damage_type_choice:
      - radiant
      - necrotic
      exemption: password_spoken_on_entry
      password: caster_optional_at_cast
    persistence:
      same_location: true
      consecutive_days: 30
      frequency: daily
      resulting_duration: until_dispelled
      material_consumed: last_cast_only
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: day
splat_contracts:
- id: dnd2024:spell_forbiddance
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
  - damage_results
  - locations
  events:
  - spell.cast_completed
  - turn.ended
  - area.entered
  - time.elapsed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
  - campaign_state
  - planar_travel
splat_conversion:
  status: specified
  category: spell
  source_sha256: 06fc802171f0fca3c3b4e9e52f8874adeb5f6b61f9e81213481bfe9e485bc3e1
  source_path: builtin/srd-5.2.1/spell/forbiddance.md
  pages:
  - 133
  engine_gaps: &id001
  - dnd2024:spell_forbiddance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 74bb0b1260e3065258246598f1183df24873090cca40b41cf36639fdfdef0077
splat_clauses:
- key: source_rules
  text: '# Forbiddance Level 6 Abjuration (Cleric) Casting Time: 10 minutes or Ritual Range: Touch Components:
    V, S, M (ruby dust worth 1,000+ GP) Duration: 1 day You create a ward against magical travel that
    protects up to 40,000 square feet of floor space to a height of 30 feet above the floor. For the duration,
    creatures can’t teleport into the area or use portals, such as those created by the Gate spell, to
    enter the area. The spell proofs the area against planar travel, and therefore prevents creatures
    from accessing the area by way of the Astral Plane, the Ethereal Plane, the Feywild, the Shadowfell,
    or the Plane Shift spell. In addition, the spell damages types of creatures that you choose when you
    cast it. Choose one or more of the following: Aberrations, Celestials, Elementals, Fey, Fiends, and
    Undead. When a creature of a chosen type enters the spell’s area for the first time on a turn or ends
    its turn there, the creature takes 5d10 Radiant or Necrotic damage (your choice when you cast this
    spell). You can designate a password when you cast the spell. A creature that speaks the password
    as it enters the area takes no damage from the spell. The spell’s area can’t overlap with the area
    of another Forbiddance spell. If you cast Forbiddance every day for 30 days in the same location,
    the spell lasts until it is dispelled, and the Material components are consumed on the last casting.'
  sha256: 9a2abe4632e06523b8d678080bdb4924587a51116ef9e6ddff7c17c56fc4b4f6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Forbiddance

<!-- SRD page 133 -->

Level 6 Abjuration (Cleric)\
Casting Time: 10 minutes or Ritual\
Range: Touch\
Components: V, S, M (ruby dust worth 1,000+ GP)\
Duration: 1 day\
You create a ward against magical travel that pro-\
tects up to 40,000 square feet of floor space to a\
height of 30 feet above the floor. For the duration,\
creatures can’t teleport into the area or use portals,\
such as those created by the Gate spell, to enter the\
area. The spell proofs the area against planar travel,\
and therefore prevents creatures from accessing\
the area by way of the Astral Plane, the Ethereal\
Plane, the Feywild, the Shadowfell, or the Plane Shift\
spell.\
In addition, the spell damages types of creatures\
that you choose when you cast it. Choose one or\
more of the following: Aberrations, Celestials, Ele-\
mentals, Fey, Fiends, and Undead. When a creature\
of a chosen type enters the spell’s area for the first\
time on a turn or ends its turn there, the creature\
takes 5d10 Radiant or Necrotic damage (your\
choice when you cast this spell).\
You can designate a password when you cast the\
spell. A creature that speaks the password as it en-\
ters the area takes no damage from the spell.\
The spell’s area can’t overlap with the area of\
another Forbiddance spell. If you cast Forbiddance\
every day for 30 days in the same location, the spell\
lasts until it is dispelled, and the Material compo-\
nents are consumed on the last casting.\

Original layout: [[../../srd-5.2.1/pages/133|page 133]].
