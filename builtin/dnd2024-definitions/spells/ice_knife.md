---
splat_schema: 1
splat_id: 9e6c6bc6-acae-5416-8013-5c35fb48a40e
splat_kind: spell
splat_key: dnd2024:spells/ice_knife
splat_name: Ice Knife
splat_origin:
  key: dnd2024:reference/spell/ice_knife
splat_sources:
- title: SRD 5.2.1
  locator: pp. 141
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: conjuration
  classes:
  - druid
  - sorcerer
  - wizard
  components:
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
  material:
    description: a drop of water or a piece of ice
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/ice_knife
  implementation: dnd2024:spell_ice_knife
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
              count: 1
              sides: 10
              add: 0
          type: piercing
    explosion:
      after:
      - hit
      - miss
      targets:
      - original_target
      - creatures_within_5_feet_of_target
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          damage:
            amount:
              dice:
                count: 2
                sides: 6
                add: 0
            type: cold
        success:
          effect: none
    scaling:
      per_slot_above: 1
      cold_damage_dice: 1
      sides: 6
splat_contracts:
- id: dnd2024:spell_ice_knife
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
  - saves
  - damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - attacks
splat_conversion:
  status: specified
  category: spell
  source_sha256: 3823e1331dd8fa558a9e3b210590ce88ce8bdb946b3cc9ad0f0a057157ec0a2d
  source_path: builtin/srd-5.2.1/spell/ice_knife.md
  pages:
  - 141
  engine_gaps: &id001
  - dnd2024:spell_ice_knife
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: c9af11f4473d7879e882f97a5ac32861fd3e79e8b84dcdd7937768f71ec86d39
splat_clauses:
- key: source_rules
  text: '# Ice Knife Level 1 Conjuration (Druid, Sorcerer, Wizard) Casting Time: Action Range: 60 feet
    Components: S, M (a drop of water or a piece of ice) Duration: Instantaneous You create a shard of
    ice and fling it at one creature within range. Make a ranged spell attack against the target. On a
    hit, the target takes 1d10 Piercing damage. Hit or miss, the shard then explodes. The target and each
    creature within 5 feet of it must succeed on a Dexterity saving throw or take 2d6 Cold damage. Using
    a Higher-Level Spell Slot. The Cold damage increases by 1d6 for each spell slot level above 1.'
  sha256: eaa287fca4ab83734640d481e0672de43ff9f8117c8cac065e1bc8d9ed6c599c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ice Knife

<!-- SRD page 141 -->

Level 1 Conjuration (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: S, M (a drop of water or a piece of ice)\
Duration: Instantaneous\
You create a shard of ice and fling it at one creature\
within range. Make a ranged spell attack against\
the target. On a hit, the target takes 1d10 Piercing\
damage. Hit or miss, the shard then explodes. The\
target and each creature within 5 feet of it must\
succeed on a Dexterity saving throw or take 2d6\
Cold damage.\
Using a Higher-Level Spell Slot. The Cold damage\
increases by 1d6 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/141|page 141]].
