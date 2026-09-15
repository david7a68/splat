---
splat_schema: 1
splat_id: 3372e62c-39f0-5f70-8e07-a95722ffd0a4
splat_kind: spell
splat_key: dnd2024:spells/freedom_of_movement
splat_name: Freedom of Movement
splat_origin:
  key: dnd2024:reference/spell/freedom_of_movement
splat_sources:
- title: SRD 5.2.1
  locator: pp. 134
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: abjuration
  classes:
  - bard
  - cleric
  - druid
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a leather strap
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/freedom_of_movement
  implementation: dnd2024:spell_freedom_of_movement
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: false
      willing: true
      touch: true
    effects:
      ignore_difficult_terrain: true
      magical_speed_reduction: false
      magically_imposed_conditions_prevented:
      - paralyzed
      - restrained
      swim_speed:
        stat:
          of: target
          key: speed
    escape:
      cost_movement_feet: 5
      automatic: true
      eligible:
      - nonmagical_restraints
      - nonmagical_grapple
    scaling:
      per_slot_above: 4
      additional_targets: 1
splat_contracts:
- id: dnd2024:spell_freedom_of_movement
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
  - movement_effect
  - condition_protection
  - escape
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - conditions
  - movement
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 16ae66dc402f4e46091360e3cf25fa2b1df076041314fdd75f23b0603693f2c0
  source_path: builtin/srd-5.2.1/spell/freedom_of_movement.md
  pages:
  - 134
  engine_gaps: &id001
  - dnd2024:spell_freedom_of_movement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 78d608eff22f5cfdbbdb77a0e5384244e2fa0d3e948f216def5b19b6d4043143
splat_clauses:
- key: source_rules
  text: '# Freedom of Movement Level 4 Abjuration (Bard, Cleric, Druid, Ranger) Casting Time: Action Range:
    Touch Components: V, S, M (a leather strap) Duration: 1 hour You touch a willing creature. For the
    duration, the target’s movement is unaffected by Difficult Terrain, and spells and other magical effects
    can neither reduce the target’s Speed nor cause the target to have the Paralyzed or Restrained conditions.
    The target also has a Swim Speed equal to its Speed. In addition, the target can spend 5 feet of movement
    to automatically escape from nonmagical restraints, such as manacles or a creature imposing the Grappled
    condition on it. Using a Higher-Level Spell Slot. You can target one additional creature for each
    spell slot level above 4.'
  sha256: d17939bff7a4068434fbc2136acb2e0d930bed93b4a5ba74b0fb4f09de397ed4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Freedom of Movement

<!-- SRD page 134 -->

Level 4 Abjuration (Bard, Cleric, Druid, Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a leather strap)\
Duration: 1 hour\
You touch a willing creature. For the duration,\
the target’s movement is unaffected by Difficult\
Terrain, and spells and other magical effects can\
neither reduce the target’s Speed nor cause the tar-\
get to have the Paralyzed or Restrained conditions.\
The target also has a Swim Speed equal to its Speed.\
In addition, the target can spend 5 feet of move-\
ment to automatically escape from nonmagical\
restraints, such as manacles or a creature imposing\
the Grappled condition on it.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 4.\

Original layout: [[../../srd-5.2.1/pages/134|page 134]].
