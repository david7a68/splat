---
splat_schema: 1
splat_id: e9e546bf-ca59-56f1-89e6-bd90198954a9
splat_kind: spell
splat_key: dnd2024:spells/disintegrate
splat_name: Disintegrate
splat_origin:
  key: dnd2024:reference/spell/disintegrate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 124
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: transmutation
  classes:
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
    feet: 60
  material:
    description: a lodestone and dust
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/disintegrate
  implementation: dnd2024:spell_disintegrate
  parameters:
    targets:
      kind_choices:
      - creature
      - nonmagical_object
      - magical_force_creation
      visible: true
      within_spell_range: true
    creature:
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          damage:
            dice:
              count:
                op: add
                args:
                - 10
                - op: multiply
                  args:
                  - 3
                  - op: subtract
                    args:
                    - binding: cast_level
                    - 6
              sides: 6
              add: 40
            type: force
          if_damage_reduces_to_zero_hp:
            transform_to: gray_dust
            include_worn_carried_nonmagical_items: true
            revival_spells_only:
            - key: dnd2024:spells/true_resurrection
            - key: dnd2024:spells/wish
        success:
          effect: none
    object_or_force:
      automatic_disintegration: true
      maximum_size_for_whole_target: large
      huge_or_larger:
        portion_shape: cube
        side_feet: 10
splat_contracts:
- id: dnd2024:spell_disintegrate
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
    objects:
      required: false
      type: object_list
    save_dc:
      type: integer
      required: true
    choices:
      required: false
      type: choice
    items:
      required: false
      type: item_list
  outputs:
  - effects
  - damage
  - items
  events:
  - spell.cast_completed
  - save.before
  - save.after
  - hp.changed
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - saving_throws
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: 33ac3f6de9d4619f8bae71960721c9f1df1819f099bce720521587449e6404c8
  source_path: builtin/srd-5.2.1/spell/disintegrate.md
  pages:
  - 124
  engine_gaps: &id001
  - dnd2024:spell_disintegrate
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: c9eb3e73ec4042ed5bf7608d9c2a84e7ca63bf4353c7578220243fbd8886342a
splat_clauses:
- key: source_rules
  text: '# Disintegrate Level 6 Transmutation (Sorcerer, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S, M (a lodestone and dust) Duration: Instantaneous You launch a green ray at a target you can
    see within range. The target can be a creature, a nonmagical object, or a creation of magical force,
    such as the wall created by Wall of Force. A creature targeted by this spell makes a Dexterity saving
    throw. On a failed save, the target takes 10d6 + 40 Force damage. If this damage reduces it to 0 Hit
    Points, it and everything nonmagical it is wearing and carrying are disintegrated into gray dust.
    The target can be revived only by a True Resurrection or a Wish spell. This spell automatically disintegrates
    a Large or smaller nonmagical object or a creation of magical force. If such a target is Huge or larger,
    this spell disintegrates a 10-foot-Cube portion of it. Using a Higher-Level Spell Slot. The damage
    increases by 3d6 for each spell slot level above 6.'
  sha256: 81507d3b34500771036e7bac0aff14bc920497ab7768f9d6069129e33c92d8c8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Disintegrate

<!-- SRD page 124 -->

Level 6 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a lodestone and dust)\
Duration: Instantaneous\
You launch a green ray at a target you can see\
within range. The target can be a creature, a non-\
magical object, or a creation of magical force, such\
as the wall created by Wall of Force.\
A creature targeted by this spell makes a Dexter-\
ity saving throw. On a failed save, the target takes\
10d6 + 40 Force damage. If this damage reduces it\
to 0 Hit Points, it and everything nonmagical it is\
wearing and carrying are disintegrated into gray\
dust. The target can be revived only by a True Res-\
urrection or a Wish spell.\
This spell automatically disintegrates a Large or\
smaller nonmagical object or a creation of magical\
force. If such a target is Huge or larger, this spell\
disintegrates a 10-foot-Cube portion of it.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 3d6 for each spell slot level above 6.\

Original layout: [[../../srd-5.2.1/pages/124|page 124]].
