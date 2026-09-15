---
splat_schema: 1
splat_id: 94e81453-9723-5b08-a541-f6f5bfd8074b
splat_kind: spell
splat_key: dnd2024:spells/locate_creature
splat_name: Locate Creature
splat_origin:
  key: dnd2024:reference/spell/locate_creature
splat_sources:
- title: SRD 5.2.1
  locator: pp. 145
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: divination
  classes:
  - bard
  - cleric
  - druid
  - paladin
  - ranger
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: fur from a bloodhound
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/locate_creature
  implementation: dnd2024:spell_locate_creature
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 3600
    target_choice:
      chooser: caster
      named_or_described: true
      familiar: true
      options:
      - known_specific_creature
      - nearest_of_specific_kind
      qualifying_prior_sighting:
        distance_feet_max: 30
        count_min: 1
    detection:
      distance_feet_max: 1000
      outputs:
      - direction_to_location
      moving_target_additional_output: movement_direction
    failure_conditions:
    - target_in_different_form
    - lead_blocks_direct_path
    lead_block:
      thickness: any
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_locate_creature
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    creature_description:
      type: choice
      required: true
    world_facts:
      type: resolution
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - information
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - information
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: dc6567ce624c63c24bd95e3885d0a2a858b5c3eb1ed7d5fa9770682aacb49aa1
  source_path: builtin/srd-5.2.1/spell/locate_creature.md
  pages:
  - 145
  engine_gaps: &id001
  - dnd2024:spell_locate_creature
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: d46983cc334cb367ed07b46cd3e3689c88982d947a48daae842601935682d63f
splat_clauses:
- key: source_rules
  text: '# Locate Creature Level 4 Divination (Bard, Cleric, Druid, Paladin, Ranger, Wizard) Casting Time:
    Action Range: Self Components: V, S, M (fur from a bloodhound) Duration: Concentration, up to 1 hour
    Describe or name a creature that is familiar to you. You sense the direction to the creature’s location
    if that creature is within 1,000 feet of you. If the creature is moving, you know the direction of
    its movement. The spell can locate a specific creature known to you or the nearest creature of a specific
    kind (such as a human or a unicorn) if you have seen such a creature up close—within 30 feet—at least
    once. If the creature you described or named is in a different form, such as under the effects of
    a Flesh to Stone or Polymorph spell, this spell doesn’t locate the creature. This spell can’t locate
    a creature if any thickness of lead blocks a direct path between you and the creature.'
  sha256: adf598f816faf2eb233f6b9dc3306d1c46c7e5a930bd663ec8362bb8d4e28834
  status: contract_specified
splat_requires_mechanics: *id001
---

# Locate Creature

<!-- SRD page 145 -->

Level 4 Divination (Bard, Cleric, Druid, Paladin,\
Ranger, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (fur from a bloodhound)\
Duration: Concentration, up to 1 hour\
Describe or name a creature that is familiar to you.\
You sense the direction to the creature’s location\
if that creature is within 1,000 feet of you. If the\
creature is moving, you know the direction of its\
movement.\
The spell can locate a specific creature known to\
you or the nearest creature of a specific kind (such\
as a human or a unicorn) if you have seen such a\
creature up close—within 30 feet—at least once.\
If the creature you described or named is in a dif-\
ferent form, such as under the effects of a Flesh to\
Stone or Polymorph spell, this spell doesn’t locate\
the creature.\
This spell can’t locate a creature if any thickness\
of lead blocks a direct path between you and the\
creature.\

Original layout: [[../../srd-5.2.1/pages/145|page 145]].
