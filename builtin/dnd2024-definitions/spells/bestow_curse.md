---
splat_schema: 1
splat_id: d8025a32-bf23-5965-b773-6e93bdf2f343
splat_kind: spell
splat_key: dnd2024:spells/bestow_curse
splat_name: Bestow Curse
splat_origin:
  key: dnd2024:reference/spell/bestow_curse
splat_sources:
- title: SRD 5.2.1
  locator: pp. 112, 113
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: necromancy
  classes:
  - bard
  - cleric
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/bestow_curse
  implementation: dnd2024:spell_bestow_curse
  parameters:
    spell_lifetime:
      determined_by: slot_duration
    target:
      kind: creature
      touch: true
    save:
      ability: wisdom
      dc: spell_save_dc
      success: unaffected
    failure:
      apply: curse
      choose_effect: 1
      chooser: caster
    options:
      ability_impairment:
        ability_choice:
        - strength
        - dexterity
        - constitution
        - intelligence
        - wisdom
        - charisma
        disadvantage:
        - chosen_ability_checks
        - chosen_ability_saves
      attack_impairment:
        disadvantage: attack_rolls
        against: caster
      forced_dodge:
        in_combat_only: true
        event: turn.start
        subject: target
        save:
          ability: wisdom
          dc: spell_save_dc
        failure:
          mandatory_action: dodge
          duration: current_turn
      extra_damage:
        event: damage.applied
        source: caster
        target: cursed_target
        eligible_causes:
        - attack_roll
        - spell
        extra_damage:
          dice: 1d8
          type: necrotic
    slot_duration:
    - levels:
      - 3
      seconds: 60
      concentration: true
    - levels:
      - 4
      seconds: 600
      concentration: true
    - levels:
      - 5
      - 6
      seconds: 28800
      concentration: false
    - levels:
      - 7
      - 8
      seconds: 86400
      concentration: false
    - levels:
      - 9
      until: dispelled
      concentration: false
splat_contracts:
- id: dnd2024:spell_bestow_curse
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    target:
      type: participant
      required: true
    curse_choice:
      type: choice
      required: true
    save_dc:
      type: integer
      required: true
  outputs:
  - effects
  - damage
  events:
  - concentration.ended
  - damage.applied
  - spell.cast_completed
  - turn.start
  capabilities:
  - spellcasting
  - effects
  - behavior
  - attacks
  - damage
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: a5fd45be321513bfa2172323602e110650f8a9d824a10f7eaba3c12f6809017c
  source_path: builtin/srd-5.2.1/spell/bestow_curse.md
  pages:
  - 112
  - 113
  engine_gaps: &id001
  - dnd2024:spell_bestow_curse
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 3e43d51904644557cf0ef1ee5cfea245c051691f8dc4e66a9791814ce6d4dd0d
splat_clauses:
- key: source_rules
  text: '# Bestow Curse Level 3 Necromancy (Bard, Cleric, Wizard) Casting Time: Action Range: Touch Components:
    V, S Duration: Concentration, up to 1 minute You touch a creature, which must succeed on a Wisdom
    saving throw or become cursed for the duration. Until the curse ends, the target suffers one of the
    following effects of your choice: • Choose one ability. The target has Disadvantage on ability checks
    and saving throws made with that ability. • The target has Disadvantage on attack rolls against you.
    • In combat, the target must succeed on a Wisdom saving throw at the start of each of its turns or
    be forced to take the Dodge action on that turn. • If you deal damage to the target with an attack
    roll or a spell, the target takes an extra 1d8 Necrotic damage. Using a Higher-Level Spell Slot. If
    you cast this spell using a level 4 spell slot, you can maintain Concentration on it for up to 10
    minutes. If you use a level 5+ spell slot, the spell doesn’t require Concentration, and the duration
    becomes 8 hours (level 5–6 slot) or 24 hours (level 7–8 slot). If you use a level 9 spell slot, the
    spell lasts until dispelled.'
  sha256: b00a9fb41aa031594c09006457d6f9b1436054d83b8b656326ad57a4b5873b08
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bestow Curse

<!-- SRD page 112 -->

Level 3 Necromancy (Bard, Cleric, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Concentration, up to 1 minute\
You touch a creature, which must succeed on a Wis-\
dom saving throw or become cursed for the dura-\
tion. Until the curse ends, the target suffers one of\
the following effects of your choice:\
•  Choose one ability. The target has Disadvantage\
on ability checks and saving throws made with\
that ability.\
•  The target has Disadvantage on attack rolls\
against you.\
•  In combat, the target must succeed on a Wisdom\
saving throw at the start of each of its turns or be\
forced to take the Dodge action on that turn.\

<!-- SRD page 113 -->

•  If you deal damage to the target with an attack\
roll or a spell, the target takes an extra 1d8 Ne-\
crotic damage.\
Using a Higher-Level Spell Slot. If you cast this\
spell using a level 4 spell slot, you can maintain\
Concentration on it for up to 10 minutes. If you use\
a level 5+ spell slot, the spell doesn’t require Con-\
centration, and the duration becomes 8 hours (level\
5–6 slot) or 24 hours (level 7–8 slot). If you use a\
level 9 spell slot, the spell lasts until dispelled.\

Original layout: [[../../srd-5.2.1/pages/112|page 112]], [[../../srd-5.2.1/pages/113|page 113]].
