---
splat_schema: 1
splat_id: 74a70df4-dd74-55d0-bd7f-1efc59873ae8
splat_kind: spell
splat_key: dnd2024:spells/dominate_person
splat_name: Dominate Person
splat_origin:
  key: dnd2024:reference/spell/dominate_person
splat_sources:
- title: SRD 5.2.1
  locator: pp. 126
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: enchantment
  classes:
  - bard
  - sorcerer
  - wizard
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
  mechanic: dnd2024:spells/dominate_person
  implementation: dnd2024:spell_dominate_person
  parameters:
    target:
      kind: humanoid
      count: 1
      visible_to_caster: true
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        condition: charmed
      success:
        effect: none
    initial_save_advantage_if: caster_or_allies_fighting_target
    repeat_save:
      event: damage.applied
      subject: target
      ability: wisdom
      success: end_spell_on_self
    link:
      telepathic: true
      same_plane_required: true
      command_on: caster_turn
      command_action_cost: none
      obey: best_effort_on_own_turn
      after_completed_without_new_order: free_actions_focused_on_self_protection
      reaction_command_cost: caster_reaction
    duration_by_slot:
      '5':
        minutes: 1
      '6':
        minutes: 10
      '7':
        hours: 1
      '8':
        hours: 8
      '9':
        hours: 8
    concentration: true
splat_contracts:
- id: dnd2024:spell_dominate_person
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
  - saves
  - charmed
  - telepathic_link
  - commands
  events:
  - spell.cast_completed
  - damage.applied
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - conditions
  - time
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 1a2d20d646bde1ba119ea02e78ed6c6d3ebc703b5f0a0a8b259720fa525dd699
  source_path: builtin/srd-5.2.1/spell/dominate_person.md
  pages:
  - 126
  engine_gaps: &id001
  - dnd2024:spell_dominate_person
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 57f8962e37ad8dcab2a652b7bf9037eced6333698a8dd7746c082b34c394217a
splat_clauses:
- key: source_rules
  text: '# Dominate Person Level 5 Enchantment (Bard, Sorcerer, Wizard) Casting Time: Action Range: 60
    feet Components: V, S Duration: Concentration, up to 1 minute One Humanoid you can see within range
    must succeed on a Wisdom saving throw or have the Charmed condition for the duration. The target has
    Advantage on the save if you or your allies are fighting it. Whenever the target takes damage, it
    repeats the save, ending the spell on itself on a success. You have a telepathic link with the Charmed
    target while the two of you are on the same plane of existence. On your turn, you can use this link
    to issue commands to the target (no action required), such as “Attack that creature,” “Move over there,”
    or “Fetch that object.” The target does its best to obey on its turn. If it completes an order and
    doesn’t receive further direction from you, it acts and moves as it likes, focusing on protecting
    itself. You can command the target to take a Reaction but must take your own Reaction to do so. Using
    a Higher-Level Spell Slot. Your Concentration can last longer with a spell slot of level 6 (up to
    10 minutes), 7 (up to 1 hour), or 8+ (up to 8 hours).'
  sha256: 3ac95e298afc7a4f95b755914519aee8c783424ffe512e415b7600723b48427d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dominate Person

<!-- SRD page 126 -->

Level 5 Enchantment (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
One Humanoid you can see within range must\
succeed on a Wisdom saving throw or have the\
Charmed condition for the duration. The target has\
Advantage on the save if you or your allies are fight-\
ing it. Whenever the target takes damage, it repeats\
the save, ending the spell on itself on a success.\
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
tion can last longer with a spell slot of level 6 (up to\
10 minutes), 7 (up to 1 hour), or 8+ (up to 8 hours).\

Original layout: [[../../srd-5.2.1/pages/126|page 126]].
