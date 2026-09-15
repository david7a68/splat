---
splat_schema: 1
splat_id: 38502ecc-19d6-5bc4-8492-c5031eda6016
splat_kind: spell
splat_key: dnd2024:spells/moonbeam
splat_name: Moonbeam
splat_origin:
  key: dnd2024:reference/spell/moonbeam
splat_sources:
- title: SRD 5.2.1
  locator: pp. 150, 151
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: evocation
  classes:
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 120
  material:
    description: a moonseed leaf
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/moonbeam
  implementation: dnd2024:spell_moonbeam
  parameters:
    area:
      shape: cylinder
      radius_feet: 5
      height_feet: 40
      light: dim
    move:
      action: magic
      timing: later_caster_turns
      maximum_feet: 60
    triggers:
    - spell.cast_completed
    - area.moves_into_creature
    - area.enter
    - turn.end
    save:
      ability: constitution
      dc:
        binding: save_dc
      failure:
        damage:
          amount:
            dice:
              count: 2
              sides: 10
              add: 0
          type: radiant
        if_shape_shifted:
          revert_true_form: true
          prevent_shape_shift_until: leaves_cylinder
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
        revert: false
    limit:
      saves_per_creature_per_turn: 1
    scaling:
      per_slot_above: 2
      damage_dice: 1
      sides: 10
splat_contracts:
- id: dnd2024:spell_moonbeam
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
  - area_effect
  - saves
  - damage
  - form_reversion
  - shape_shift_prohibition
  events:
  - spell.cast_completed
  - turn.end
  - area.enter
  - area.moved
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - spatial_areas
  - movement
  - perception
  - forms
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: bc6b7af378fcaaeeea500261b8247a0a5c8b5ea672674c27af9e0943835ebdc1
  source_path: builtin/srd-5.2.1/spell/moonbeam.md
  pages:
  - 150
  - 151
  engine_gaps: &id001
  - dnd2024:spell_moonbeam
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 8f191514179e598351473a153b30c74a098051d6c7eea379ef86c5c5d7bff5f9
splat_clauses:
- key: source_rules
  text: '# Moonbeam Level 2 Evocation (Druid) Casting Time: Action Range: 120 feet Components: V, S, M
    (a moonseed leaf) Duration: Concentration, up to 1 minute A silvery beam of pale light shines down
    in a 5-foot-radius, 40-foot-high Cylinder centered on a point within range. Until the spell ends,
    Dim Light fills the Cylinder, and you can take a Magic action on later turns to move the Cylinder
    up to 60 feet. When the Cylinder appears, each creature in it makes a Constitution saving throw. On
    a failed save, a creature takes 2d10 Radiant damage, and if the creature is shape-shifted (as a result
    of the Polymorph spell, for example), it reverts to its true form and can’t shape-shift until it leaves
    the Cylinder. On a successful save, a creature takes half as much damage only. A creature also makes
    this save when the spell’s area moves into its space and when it enters the spell’s area or ends its
    turn there. A creature makes this save only once per turn. Using a Higher-Level Spell Slot. The damage
    increases by 1d10 for each spell slot level above 2.'
  sha256: 49515ce3b6061c264a4ba63decf2778df894ac422a844e01417c37fc3b19ca9e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Moonbeam

<!-- SRD page 150 -->

Level 2 Evocation (Druid)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a moonseed leaf)\
Duration: Concentration, up to 1 minute\

<!-- SRD page 151 -->

A silvery beam of pale light shines down in a\
5-foot-radius, 40-foot-high Cylinder centered on a\
point within range. Until the spell ends, Dim Light\
fills the Cylinder, and you can take a Magic action on\
later turns to move the Cylinder up to 60 feet.\
When the Cylinder appears, each creature in it\
makes a Constitution saving throw. On a failed save,\
a creature takes 2d10 Radiant damage, and if the\
creature is shape-shifted (as a result of the Poly-\
morph spell, for example), it reverts to its true form\
and can’t shape-shift until it leaves the Cylinder.\
On a successful save, a creature takes half as much\
damage only. A creature also makes this save when\
the spell’s area moves into its space and when it\
enters the spell’s area or ends its turn there. A crea-\
ture makes this save only once per turn.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d10 for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/150|page 150]], [[../../srd-5.2.1/pages/151|page 151]].
