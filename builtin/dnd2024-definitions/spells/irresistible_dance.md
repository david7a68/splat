---
splat_schema: 1
splat_id: 4113a55e-6142-5b7c-96b9-b896f5ecb073
splat_kind: spell
splat_key: dnd2024:spells/irresistible_dance
splat_name: Irresistible Dance
splat_origin:
  key: dnd2024:reference/spell/irresistible_dance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: enchantment
  classes:
  - bard
  - wizard
  components:
  - verbal
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/irresistible_dance
  implementation: dnd2024:spell_irresistible_dance
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      visible: true
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        condition: charmed
        duration: spell_duration
        while_charmed:
          dance: true
          all_movement: dance_in_place
          disadvantage:
          - dexterity_saves
          - attack_rolls
          other_creatures_attack_advantage_against_target: true
        repeat_save:
          cost: action
          timing: each_target_turn
          success: end_spell_on_target
      success:
        dance: true
        all_movement: dance_in_place
        until: target_next_turn_end
splat_contracts:
- id: dnd2024:spell_irresistible_dance
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
    save_dc:
      type: integer
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - spell.cast_completed
  - turn.end
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - conditions
  - saving_throws
  - roll_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: 17a68852c5047f009f251347e0b7602c6f25f6c820f6d2a74274e46f4f86b33d
  source_path: builtin/srd-5.2.1/spell/irresistible_dance.md
  pages:
  - 143
  engine_gaps: &id001
  - dnd2024:spell_irresistible_dance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 1a435ff43d1b973ee55e6176df2879e959831094bfb1d079c52f9521f94f83bd
splat_clauses:
- key: source_rules
  text: '# Irresistible Dance Level 6 Enchantment (Bard, Wizard) Casting Time: Action Range: 30 feet Components:
    V Duration: Concentration, up to 1 minute One creature that you can see within range must make a Wisdom
    saving throw. On a successful save, the target dances comically until the end of its next turn, during
    which it must spend all its movement to dance in place. On a failed save, the target has the Charmed
    condition for the duration. While Charmed, the target dances comically, must use all its movement
    to dance in place, and has Disadvantage on Dexterity saving throws and attack rolls, and other creatures
    have Advantage on attack rolls against it. On each of its turns, the target can take an action to
    collect itself and repeat the save, ending the spell on itself on a success.'
  sha256: 661766e50cc2677d19d0017913d1fa90ee79cb227d8c13a890e769c3961c4f02
  status: contract_specified
splat_requires_mechanics: *id001
---

# Irresistible Dance

<!-- SRD page 143 -->

Level 6 Enchantment (Bard, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V\
Duration: Concentration, up to 1 minute\
One creature that you can see within range must\
make a Wisdom saving throw. On a successful save,\
the target dances comically until the end of its next\
turn, during which it must spend all its movement\
to dance in place.\
On a failed save, the target has the Charmed con-\
dition for the duration. While Charmed, the target\
dances comically, must use all its movement to\
dance in place, and has Disadvantage on Dexterity\
saving throws and attack rolls, and other creatures\
have Advantage on attack rolls against it. On each\
of its turns, the target can take an action to collect\
itself and repeat the save, ending the spell on itself\
on a success.\

Original layout: [[../../srd-5.2.1/pages/143|page 143]].
