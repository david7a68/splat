---
splat_schema: 1
splat_id: 8ec51fa9-2142-5cb4-8a48-0506239318cf
splat_kind: spell
splat_key: dnd2024:spells/sorcerous_burst
splat_name: Sorcerous Burst
splat_origin:
  key: dnd2024:reference/spell/sorcerous_burst
splat_sources:
- title: SRD 5.2.1
  locator: pp. 163
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: evocation
  classes:
  - sorcerer
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/sorcerous_burst
  implementation: dnd2024:spell_sorcerous_burst
  parameters:
    targeting:
      kind: creature_or_object
      count: 1
      within_range: true
    attack:
      kind: ranged_spell
      hit:
        damage:
          dice: 1d8
          type_choice:
          - acid
          - cold
          - fire
          - lightning
          - poison
          - psychic
          - thunder
    exploding_dice:
      trigger: any_spell_d8_result_is_8
      optional: true
      additional_die: 1d8
      add_to_damage: true
      added_dice_can_trigger: true
      total_added_dice_limit_per_cast:
        stat: caster_spellcasting_ability_modifier
    cantrip_scaling:
      character_level_dice:
        '1': 1d8
        '5': 2d8
        '11': 3d8
        '17': 4d8
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_sorcerous_burst
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
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
splat_conversion:
  status: specified
  category: spell
  source_sha256: d5b38970d7c1ecdab628cfdb4eb8a3fcce213b71caf2f564135891bec1285e79
  source_path: builtin/srd-5.2.1/spell/sorcerous_burst.md
  pages:
  - 163
  engine_gaps: &id001
  - dnd2024:spell_sorcerous_burst
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 777174af23f10fe374b41db54239d721fb4fc765397a25e479deef4bec9b69e7
splat_clauses:
- key: source_rules
  text: '# Sorcerous Burst Evocation Cantrip (Sorcerer) Casting Time: Action Range: 120 feet Component:
    V, S Duration: Instantaneous You cast sorcerous energy at one creature or object within range. Make
    a ranged spell attack against the target. On a hit, the target takes 1d8 damage of a type you choose:
    Acid, Cold, Fire, Lightning, Poison, Psychic, or Thunder. If you roll an 8 on a d8 for this spell,
    you can roll another d8, and add it to the damage. When you cast this spell, the maximum number of
    these d8s you can add to the spell’s damage equals your spellcasting ability modifier. Cantrip Upgrade.
    The damage increases by 1d8 when you reach levels 5 (2d8), 11 (3d8), and 17 (4d8).'
  sha256: 4497c4c38f347042aa64b4f2c0ad02c89f5df7d67170b6292c9b8e67733c5f94
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sorcerous Burst

<!-- SRD page 163 -->

Evocation Cantrip (Sorcerer)\
Casting Time: Action\
Range: 120 feet\
Component: V, S\
Duration: Instantaneous\
You cast sorcerous energy at one creature or object\
within range. Make a ranged spell attack against the\
target. On a hit, the target takes 1d8 damage of a\
type you choose: Acid, Cold, Fire, Lightning, Poison,\
Psychic, or Thunder.\
If you roll an 8 on a d8 for this spell, you can roll\
another d8, and add it to the damage. When you cast\
this spell, the maximum number of these d8s you\
can add to the spell’s damage equals your spellcast-\
ing ability modifier.\
Cantrip Upgrade. The damage increases by 1d8\
when you reach levels 5 (2d8), 11 (3d8), and 17\
(4d8).\

Original layout: [[../../srd-5.2.1/pages/163|page 163]].
