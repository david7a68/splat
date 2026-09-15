---
splat_schema: 1
splat_id: fbc7531b-6863-52d4-a864-6056ce2201b1
splat_kind: spell
splat_key: dnd2024:spells/ice_storm
splat_name: Ice Storm
splat_origin:
  key: dnd2024:reference/spell/ice_storm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 141, 142
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: evocation
  classes:
  - druid
  - sorcerer
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
    feet: 300
  material:
    description: a mitten
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/ice_storm
  implementation: dnd2024:spell_ice_storm
  parameters:
    targeting:
      kind: creatures_in_area
      shape: cylinder
      radius_feet: 20
      height_feet: 40
      origin: chosen_point_in_range
    save:
      ability: dexterity
      failure:
        damage:
        - dice: 2d10
          type: bludgeoning
        - dice: 4d6
          type: cold
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    terrain:
      area: ground_in_cylinder
      difficult_terrain: true
      until: caster_next_turn_end
    slot_scaling:
      above: 4
      bludgeoning_damage_per_level: 1d10
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_ice_storm
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
    save_dc:
      type: integer
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - damage_results
  - locations
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 76f296a109811b0bda67e55316d9db20bca86ee070d26b6b8a7812de62afcf92
  source_path: builtin/srd-5.2.1/spell/ice_storm.md
  pages:
  - 141
  - 142
  engine_gaps: &id001
  - dnd2024:spell_ice_storm
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: e251cc7acb2c4356eaa5821979efc4d0501c492b8b651b02c443de1eed49770e
splat_clauses:
- key: source_rules
  text: '# Ice Storm Level 4 Evocation (Druid, Sorcerer, Wizard) Casting Time: Action Range: 300 feet
    Components: V, S, M (a mitten) Duration: Instantaneous Hail falls in a 20-foot-radius, 40-foot-high
    Cylinder centered on a point within range. Each creature in the Cylinder makes a Dexterity saving
    throw. A creature takes 2d10 Bludgeoning damage and 4d6 Cold damage on a failed save or half as much
    damage on a successful one. Hailstones turn ground in the Cylinder into Difficult Terrain until the
    end of your next turn. Using a Higher-Level Spell Slot. The Bludgeoning damage increases by 1d10 for
    each spell slot level above 4.'
  sha256: d952f8fb0f19a17772d81603d51b540e59ed809d040479bc3506acf47fb6bdff
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ice Storm

<!-- SRD page 141 -->

Level 4 Evocation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 300 feet\
Components: V, S, M (a mitten)\
Duration: Instantaneous\
Hail falls in a 20-foot-radius, 40-foot-high Cylinder\
centered on a point within range. Each creature\
in the Cylinder makes a Dexterity saving throw. A\
creature takes 2d10 Bludgeoning damage and 4d6\
Cold damage on a failed save or half as much dam-\
age on a successful one.\
Hailstones turn ground in the Cylinder into Diffi-\
cult Terrain until the end of your next turn.\

<!-- SRD page 142 -->

Using a Higher-Level Spell Slot. The Bludgeoning\
damage increases by 1d10 for each spell slot level\
above 4.\

Original layout: [[../../srd-5.2.1/pages/141|page 141]], [[../../srd-5.2.1/pages/142|page 142]].
