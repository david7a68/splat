---
splat_schema: 1
splat_id: cfc106b6-22ae-589d-a591-314d0166fc92
splat_kind: spell
splat_key: dnd2024:spells/gaseous_form
splat_name: Gaseous Form
splat_origin:
  key: dnd2024:reference/spell/gaseous_form
splat_sources:
- title: SRD 5.2.1
  locator: pp. 134, 135
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: transmutation
  classes:
  - sorcerer
  - warlock
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
    mode: touch
  material:
    description: a bit of gauze
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/gaseous_form
  implementation: dnd2024:spell_gaseous_form
  parameters:
    targets:
      kind: creature
      maximum:
        op: add
        args:
        - 1
        - op: multiply
          args:
          - 1
          - op: subtract
            args:
            - binding: cast_level
            - 3
      within_spell_range: true
      willing: true
      touched: true
    transformation:
      form: misty_cloud
      include_worn_carried: true
      duration: spell_duration
      only_movement:
        fly_speed_feet: 10
        hover: true
      may_enter_and_occupy_other_creature_space: true
      damage_resistance:
      - bludgeoning
      - piercing
      - slashing
      condition_immunity:
      - prone
      save_advantage:
      - strength
      - dexterity
      - constitution
      pass_narrow_openings: true
      liquids_treated_as: solid_surfaces
      prohibit:
      - talk
      - manipulate_objects
      - drop_carried_or_held_objects
      - use_carried_or_held_objects
      - interact_with_carried_or_held_objects
      - attack
      - cast_spells
    end_on_target:
      hp_zero: true
      voluntary_cost: magic_action
splat_contracts:
- id: dnd2024:spell_gaseous_form
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
  - effect.ended
  - movement.completed
  - save.before
  - save.after
  - hp.changed
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
  - conditions
  - saving_throws
  - roll_modifiers
  - forms
splat_conversion:
  status: specified
  category: spell
  source_sha256: a7ad7677e5b6c1f466640376bd835081d612c313f7895e05630858a9bd65441e
  source_path: builtin/srd-5.2.1/spell/gaseous_form.md
  pages:
  - 134
  - 135
  engine_gaps: &id001
  - dnd2024:spell_gaseous_form
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 18532bdca7e0c394d5ad23be69d93bd9c355ec79317d190b944386251dc8a3eb
splat_clauses:
- key: source_rules
  text: '# Gaseous Form Level 3 Transmutation (Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    Touch Components: V, S, M (a bit of gauze) Duration: Concentration, up to 1 hour A willing creature
    you touch shape-shifts, along with everything it’s wearing and carrying, into a misty cloud for the
    duration. The spell ends on the target if it drops to 0 Hit Points or if it takes a Magic action to
    end the spell on itself. While in this form, the target’s only method of movement is a Fly Speed of
    10 feet, and it can hover. The target can enter and occupy the space of another creature. The target
    has Resistance to Bludgeoning, Piercing, and Slashing damage; it has Immunity to the Prone condition;
    and it has Advantage on Strength, Dexterity, and Constitution saving throws. The target can pass through
    narrow openings, but it treats liquids as though they were solid surfaces. The target can’t talk or
    manipulate objects, and any objects it was carrying or holding can’t be dropped, used, or otherwise
    interacted with. Finally, the target can’t attack or cast spells. Using a Higher-Level Spell Slot.
    You can target one additional creature for each spell slot level above 3.'
  sha256: 85b4e475928ba3246adddfc825b7c9ebd982d76ed67a8581a487a05e541d6bee
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gaseous Form

<!-- SRD page 134 -->

Level 3 Transmutation (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a bit of gauze)\
Duration: Concentration, up to 1 hour\
A willing creature you touch shape-shifts, along\
with everything it’s wearing and carrying, into a\
misty cloud for the duration. The spell ends on the\
target if it drops to 0 Hit Points or if it takes a Magic\
action to end the spell on itself.\

<!-- SRD page 135 -->

While in this form, the target’s only method\
of movement is a Fly Speed of 10 feet, and it can\
hover. The target can enter and occupy the space\
of another creature. The target has Resistance to\
Bludgeoning, Piercing, and Slashing damage; it has\
Immunity to the Prone condition; and it has Advan-\
tage on Strength, Dexterity, and Constitution saving\
throws. The target can pass through narrow open-\
ings, but it treats liquids as though they were solid\
surfaces.\
The target can’t talk or manipulate objects, and\
any objects it was carrying or holding can’t be\
dropped, used, or otherwise interacted with. Fi-\
nally, the target can’t attack or cast spells.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 3.\

Original layout: [[../../srd-5.2.1/pages/134|page 134]], [[../../srd-5.2.1/pages/135|page 135]].
