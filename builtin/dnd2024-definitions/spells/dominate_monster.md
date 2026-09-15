---
splat_schema: 1
splat_id: 8e4dbe10-3c5c-5109-8de2-9f59553a8b4a
splat_kind: spell
splat_key: dnd2024:spells/dominate_monster
splat_name: Dominate Monster
splat_origin:
  key: dnd2024:reference/spell/dominate_monster
splat_sources:
- title: SRD 5.2.1
  locator: pp. 125, 126
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: enchantment
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/dominate_monster
  implementation: dnd2024:spell_dominate_monster
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 3600
    target:
      kind: creature
      visible: true
      range_feet: 60
    initial_save:
      ability: wisdom
      dc: spell_save_dc
      advantage_if: caster_or_allies_fighting_target
      success: unaffected
    failure:
      condition: charmed
      source: caster
    repeat_save:
      event: damage.applied
      subject: target
      ability: wisdom
      dc: spell_save_dc
      success: end_on_target
      inherits_initial_save_advantage_rule: true
    telepathic_link:
      requires: same_plane
      while: charmed_by_spell
    commands:
      economy: none
      timing: caster_turn
      medium: telepathic_link
      compliance: best_effort_on_target_turn
      after_completion_without_new_order:
        autonomous: true
        focus: self_protection
    command_reaction:
      caster_economy: reaction
      target_economy: reaction
    slot_duration:
    - levels:
      - 8
      seconds: 3600
    - levels:
      - 9
      seconds: 28800
splat_contracts:
- id: dnd2024:spell_dominate_monster
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
    commands:
      required: false
      type: choice
    save_dc:
      type: integer
      required: true
  outputs:
  - effects
  events:
  - concentration.ended
  - damage.applied
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - behavior
  - conditions
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: d1c80107aa984c0c3c3fbdd07e251ff3530dd17701bb47b277ce03c3f7a0412c
  source_path: builtin/srd-5.2.1/spell/dominate_monster.md
  pages:
  - 125
  - 126
  engine_gaps: &id001
  - dnd2024:spell_dominate_monster
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 78580e7c66ed65332229650b13bea08a74b4480e54a049d206f89738396b5a50
splat_clauses:
- key: source_rules
  text: '# Dominate Monster Level 8 Enchantment (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action
    Range: 60 feet Components: V, S Duration: Concentration, up to 1 hour One creature you can see within
    range must succeed on a Wisdom saving throw or have the Charmed condition for the duration. The target
    has Advantage on the save if you or your allies are fighting it. Whenever the target takes damage,
    it repeats the save, ending the spell on itself on a success. You have a telepathic link with the
    Charmed target while the two of you are on the same plane of existence. On your turn, you can use
    this link to issue commands to the target (no action required), such as “Attack that creature,” “Move
    over there,” or “Fetch that object.” The target does its best to obey on its turn. If it completes
    an order and doesn’t receive further direction from you, it acts and moves as it likes, focusing on
    protecting itself. You can command the target to take a Reaction but must take your own Reaction to
    do so. Using a Higher-Level Spell Slot. Your Concentration can last longer with a level 9 spell slot
    (up to 8 hours).'
  sha256: 9f6a74c91388cf6722bf2dda33efb84a01529c14b4849dfb171d68eb3b7d2004
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dominate Monster

<!-- SRD page 125 -->

Level 8 Enchantment (Bard, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 hour\
One creature you can see within range must\
succeed on a Wisdom saving throw or have the\
Charmed condition for the duration. The target has\
Advantage on the save if you or your allies are fight-\
ing it. Whenever the target takes damage, it repeats\
the save, ending the spell on itself on a success.\

<!-- SRD page 126 -->

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
tion can last longer with a level 9 spell slot (up to 8\
hours).\

Original layout: [[../../srd-5.2.1/pages/125|page 125]], [[../../srd-5.2.1/pages/126|page 126]].
