---
splat_schema: 1
splat_id: 077d23fe-fa92-5924-8729-ba18e9d7a37f
splat_kind: spell
splat_key: dnd2024:spells/ray_of_sickness
splat_name: Ray of Sickness
splat_origin:
  key: dnd2024:reference/spell/ray_of_sickness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: necromancy
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/ray_of_sickness
  implementation: dnd2024:spell_ray_of_sickness
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: false
    attack:
      kind: ranged_spell
      hit:
        damage:
          amount:
            dice:
              count: 2
              sides: 8
              add: 0
          type: poison
        condition: poisoned
        condition_until: caster.next_turn.end
    scaling:
      per_slot_above: 1
      damage_dice: 1
      sides: 8
splat_contracts:
- id: dnd2024:spell_ray_of_sickness
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    spell_attack_modifier:
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
  - attack_result
  - damage
  - poisoned
  events:
  - spell.cast_completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
  - conditions
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: e3e5b3d23ded1bc9515ec48ad5a83f2eff02600f9537c009e604d613e9d76d65
  source_path: builtin/srd-5.2.1/spell/ray_of_sickness.md
  pages:
  - 158
  engine_gaps: &id001
  - dnd2024:spell_ray_of_sickness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 46e1507bb527ca0846d15ec91a0ca33ed67c2ac867a3726c12c84fb8230ab2ca
splat_clauses:
- key: source_rules
  text: '# Ray of Sickness Level 1 Necromancy (Sorcerer, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S Duration: Instantaneous You shoot a greenish ray at a creature within range. Make a ranged spell
    attack against the target. On a hit, the target takes 2d8 Poison damage and has the Poisoned condition
    until the end of your next turn. Using a Higher-Level Spell Slot. The damage increases by 1d8 for
    each spell slot level above 1.'
  sha256: 6096d8ef8540537a884325abfa85143d9733b4fd3d67ab6f66828f6758bc6b62
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ray of Sickness

<!-- SRD page 158 -->

Level 1 Necromancy (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
You shoot a greenish ray at a creature within range.\
Make a ranged spell attack against the target. On a\
hit, the target takes 2d8 Poison damage and has the\
Poisoned condition until the end of your next turn.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/158|page 158]].
