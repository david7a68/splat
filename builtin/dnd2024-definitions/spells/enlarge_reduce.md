---
splat_schema: 1
splat_id: 382204c4-9092-5f17-a08d-029db06492d8
splat_kind: spell
splat_key: dnd2024:spells/enlarge_reduce
splat_name: Enlarge/Reduce
splat_origin:
  key: dnd2024:reference/spell/enlarge_reduce
splat_sources:
- title: SRD 5.2.1
  locator: pp. 127, 128
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - bard
  - druid
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a pinch of powdered iron
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/enlarge_reduce
  implementation: dnd2024:spell_enlarge_reduce
  parameters:
    target:
      kind: creature_or_object
      count: 1
      visible_to_caster: true
      object_not_worn_or_carried: true
    unwilling_creature:
      may_save: true
      ability: constitution
      success: no_effect
    equipment:
      resize_worn_and_carried: true
      drop: revert_immediately
      thrown_weapon_or_ammunition: revert_after_hit_or_miss
    choice:
    - enlarge
    - reduce
    effects:
      enlarge:
        size_category_delta: 1
        strength_checks: advantage
        strength_saves: advantage
        weapon_and_unarmed_hit_damage_bonus:
          dice:
            count: 1
            sides: 4
            add: 0
      reduce:
        size_category_delta: -1
        strength_checks: disadvantage
        strength_saves: disadvantage
        weapon_and_unarmed_hit_damage_subtract:
          dice:
            count: 1
            sides: 4
            add: 0
        minimum_damage: 1
splat_contracts:
- id: dnd2024:spell_enlarge_reduce
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
  - save
  - size_effect
  - equipment_effect
  - attack_damage_modifier
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - forms
  - inventory
  - choices
  - checks
splat_conversion:
  status: specified
  category: spell
  source_sha256: 59a156b859471cae4625b0cc37f10303d2fb9f478a94db304c5fbbf3055da81d
  source_path: builtin/srd-5.2.1/spell/enlarge_reduce.md
  pages:
  - 127
  - 128
  engine_gaps: &id001
  - dnd2024:spell_enlarge_reduce
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 3e3fbb9eb62c89908fef794a026494d95b5d484f8d88a8c0a7633bab60c00254
splat_clauses:
- key: source_rules
  text: '# Enlarge/Reduce Level 2 Transmutation (Bard, Druid, Sorcerer, Wizard) Casting Time: Action Range:
    30 feet Components: V, S, M (a pinch of powdered iron) Duration: Concentration, up to 1 minute For
    the duration, the spell enlarges or reduces a creature or an object you can see within range (see
    the chosen effect below). A targeted object must be neither worn nor carried. If the target is an
    unwilling creature, it can make a Constitution saving throw. On a successful save, the spell has no
    effect. Everything that a targeted creature is wearing and carrying changes size with it. Any item
    it drops returns to normal size at once. A thrown weapon or piece of ammunition returns to normal
    size immediately after it hits or misses a target. Enlarge. The target’s size increases by one category—from
    Medium to Large, for example. The target also has Advantage on Strength checks and Strength saving
    throws. The target’s attacks with its enlarged weapons or Unarmed Strikes deal an extra 1d4 damage
    on a hit. Reduce. The target’s size decreases by one category—from Medium to Small, for example. The
    target also has Disadvantage on Strength checks and Strength saving throws. The target’s attacks with
    its reduced weapons or Unarmed Strikes deal 1d4 less damage on a hit (this can’t reduce the damage
    below 1).'
  sha256: 312466915284a6332585b835beb5475db7bbff2ef39578c74d270e16b7c36562
  status: contract_specified
splat_requires_mechanics: *id001
---

# Enlarge/Reduce

<!-- SRD page 127 -->

Level 2 Transmutation (Bard, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a pinch of powdered iron)\
Duration: Concentration, up to 1 minute\
For the duration, the spell enlarges or reduces a\
creature or an object you can see within range (see\

<!-- SRD page 128 -->

the chosen effect below). A targeted object must\
be neither worn nor carried. If the target is an un-\
willing creature, it can make a Constitution saving\
throw. On a successful save, the spell has no effect.\
Everything that a targeted creature is wearing\
and carrying changes size with it. Any item it drops\
returns to normal size at once. A thrown weapon or\
piece of ammunition returns to normal size imme-\
diately after it hits or misses a target.\
Enlarge. The target’s size increases by one cat-\
egory—from Medium to Large, for example. The\
target also has Advantage on Strength checks and\
Strength saving throws. The target’s attacks with\
its enlarged weapons or Unarmed Strikes deal an\
extra 1d4 damage on a hit.\
Reduce. The target’s size decreases by one cate-\
gory—from Medium to Small, for example. The tar-\
get also has Disadvantage on Strength checks and\
Strength saving throws. The target’s attacks with\
its reduced weapons or Unarmed Strikes deal 1d4\
less damage on a hit (this can’t reduce the damage\
below 1).\

Original layout: [[../../srd-5.2.1/pages/127|page 127]], [[../../srd-5.2.1/pages/128|page 128]].
