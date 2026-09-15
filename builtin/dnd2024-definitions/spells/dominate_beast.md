---
splat_schema: 1
splat_id: 02dda062-ceba-5fb0-bdc9-b6c308f56dde
splat_kind: spell
splat_key: dnd2024:spells/dominate_beast
splat_name: Dominate Beast
splat_origin:
  key: dnd2024:reference/spell/dominate_beast
splat_sources:
- title: SRD 5.2.1
  locator: pp. 125
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: enchantment
  classes:
  - druid
  - ranger
  - sorcerer
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/dominate_beast
  implementation: dnd2024:spell_dominate_beast
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      visible: true
      creature_type: beast
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        condition: charmed
        duration: spell_duration
      success:
        effect: none
    save_advantage_if:
      caster_or_allies_fighting_target: true
    repeat_save:
      event: target_takes_damage
      ability: wisdom
      success: end_spell_on_target
    control:
      requires: target_charmed_by_spell
      telepathy:
        requires_same_plane: true
      command:
        event: caster_turn
        cost: none
        obey: best_effort_on_target_turn
      completed_without_new_order:
        behavior: target_choice
        priority: self_protection
      reaction_command:
        cost: caster_reaction
        target_cost: reaction
    duration_by_slot:
      '4':
        minutes: 1
      '5':
        minutes: 10
      '6':
        hours: 1
      7_plus:
        hours: 8
    concentration: true
splat_contracts:
- id: dnd2024:spell_dominate_beast
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
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - spell.cast_completed
  - damage.applied
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - conditions
  - saving_throws
  - roll_modifiers
  - behavior_constraints
splat_conversion:
  status: specified
  category: spell
  source_sha256: 500e726129e675852f6c953c2577f837912d42a64eb7d295a22e1840913166e9
  source_path: builtin/srd-5.2.1/spell/dominate_beast.md
  pages:
  - 125
  engine_gaps: &id001
  - dnd2024:spell_dominate_beast
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 40dff68c311ee746b8c441b1a2e09741197f9be4700f8627b0195ebf79fed71b
splat_clauses:
- key: source_rules
  text: '# Dominate Beast Level 4 Enchantment (Druid, Ranger, Sorcerer) Casting Time: Action Range: 60
    feet Components: V, S Duration: Concentration, up to 1 minute One Beast you can see within range must
    succeed on a Wisdom saving throw or have the Charmed condition for the duration. The target has Advantage
    on the save if you or your allies are fighting it. Whenever the target takes damage, it repeats the
    save, ending the spell on itself on a success. You have a telepathic link with the Charmed target
    while the two of you are on the same plane of existence. On your turn, you can use this link to issue
    commands to the target (no action required), such as “Attack that creature,” “Move over there,” or
    “Fetch that object.” The target does its best to obey on its turn. If it completes an order and doesn’t
    receive further direction from you, it acts and moves as it likes, focusing on protecting itself.
    You can command the target to take a Reaction but must take your own Reaction to do so. Using a Higher-Level
    Spell Slot. Your Concentration can last longer with a spell slot of level 5 (up to 10 minutes), 6
    (up to 1 hour), or 7+ (up to 8 hours).'
  sha256: 13a660cfa793f6a25e7c69646fa6644c4c8d5234034449479d9e57fb456c9017
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dominate Beast

<!-- SRD page 125 -->

Level 4 Enchantment (Druid, Ranger, Sorcerer)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
One Beast you can see within range must succeed\
on a Wisdom saving throw or have the Charmed\
condition for the duration. The target has Advan-\
tage on the save if you or your allies are fighting it.\
Whenever the target takes damage, it repeats the\
save, ending the spell on itself on a success.\
You have a telepathic link with the Charmed tar-\
get while the two of you are on the same plane of\
existence. On your turn, you can use this link to\
issue commands to the target (no action required),\
such as “Attack that creature,” “Move over there,” or\
“Fetch that object.” The target does its best to obey\
on its turn. If it completes an order and doesn’t re-\
ceive further direction from you, it acts and moves\
as it likes, focusing on protecting itself.\
You can command the target to take a Reaction\
but must take your own Reaction to do so.\
Using a Higher-Level Spell Slot. Your Concentra-\
tion can last longer with a spell slot of level 5 (up to\
10 minutes), 6 (up to 1 hour), or 7+ (up to 8 hours).\

Original layout: [[../../srd-5.2.1/pages/125|page 125]].
