---
splat_schema: 1
splat_id: 912ffe59-b1cb-519d-b18a-62f944818fab
splat_kind: spell
splat_key: dnd2024:spells/modify_memory
splat_name: Modify Memory
splat_origin:
  key: dnd2024:reference/spell/modify_memory
splat_sources:
- title: SRD 5.2.1
  locator: pp. 150
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: enchantment
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/modify_memory
  implementation: dnd2024:spell_modify_memory
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 60
    target:
      kind: creature
      other_than_caster: true
      visible: true
      range_feet: 30
    save:
      ability: wisdom
      dc: spell_save_dc
      advantage_if: caster_fighting_target
      success: unaffected
    failure:
      condition: charmed
      while_charmed:
        condition: incapacitated
        aware_of_surroundings: false
        can_hear_caster: true
    interrupted:
      events:
      - target.took_damage
      - target.targeted_by_another_spell
      end_spell: true
      modify_memory: false
    memory_event:
      experienced_by_target: true
      maximum_event_minutes: 10
      history_window_by_slot:
      - level: 5
        days: 1
      - level: 6
        days: 7
      - level: 7
        days: 30
      - level: 8
        days: 365
      - level: 9
        history: any_time
    modification_choice:
      chooser: caster
      options:
      - erase_event
      - perfect_recall
      - change_details
      - create_other_event_memory
      duration: permanent
    description:
      spoken_to_target: true
      target_understands_language: true
      complete_before_spell_ends: true
      fill_missing_details: target_mind
    commit:
      event: spell.effect_ended
      requires: completed_understood_description_and_not_interrupted
      otherwise: unchanged_memory
    behavior:
      compelled_by_memory: false
      gm_assessment:
        kind: choice
        options:
        - coherent
        - dismissed_as_bad_dream
        - too_nonsensical_to_affect
        consider:
        - natural_inclinations
        - alignment
        - beliefs
    restore_true_memory_spells:
    - key: dnd2024:spells/remove_curse
    - key: dnd2024:spells/greater_restoration
splat_contracts:
- id: dnd2024:spell_modify_memory
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
    memory_event:
      type: choice
      required: true
    description:
      type: text
      required: true
    coherence_assessment:
      type: choice
      required: true
    save_dc:
      type: integer
      required: true
  outputs:
  - effects
  events:
  - concentration.ended
  - spell.cast_completed
  - spell.effect_ended
  - target.targeted_by_another_spell
  - target.took_damage
  capabilities:
  - spellcasting
  - effects
  - memory
  - behavior
  - conditions
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 63083a6941e5466274fd3980fc16870aa88fb1f198a80219e7c25e55b8d7a57b
  source_path: builtin/srd-5.2.1/spell/modify_memory.md
  pages:
  - 150
  engine_gaps: &id001
  - dnd2024:spell_modify_memory
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: cd45ac79cf8ed71185ae47b9eeec6c94aeea7d7e18e3f1b3f4918c2a389a19f4
splat_clauses:
- key: source_rules
  text: '# Modify Memory Level 5 Enchantment (Bard, Wizard) Casting Time: Action Range: 30 feet Components:
    V, S Duration: Concentration, up to 1 minute You attempt to reshape another creature’s memories. One
    creature that you can see within range makes a Wisdom saving throw. If you are fighting the creature,
    it has Advantage on the save. On a failed save, the target has the Charmed condition for the duration.
    While Charmed in this way, the target also has the Incapacitated condition and is unaware of its surroundings,
    though it can hear you. If it takes any damage or is targeted by another spell, this spell ends, and
    no memories are modified. While this charm lasts, you can affect the target’s memory of an event that
    it experienced within the last 24 hours and that lasted no more than 10 minutes. You can permanently
    eliminate all memory of the event, allow the target to recall the event with perfect clarity, change
    its memory of the event’s details, or create a memory of some other event. You must speak to the target
    to describe how its memories are affected, and it must be able to understand your language for the
    modified memories to take root. Its mind fills in any gaps in the details of your description. If
    the spell ends before you finish describing the modified memories, the creature’s memory isn’t altered.
    Otherwise, the modified memories take hold when the spell ends. A modified memory doesn’t necessarily
    affect how a creature behaves, particularly if the memory contradicts the creature’s natural inclinations,
    alignment, or beliefs. An illogical modified memory, such as a false memory of how much the creature
    enjoyed swimming in acid, is dismissed as a bad dream. The GM might deem a modified memory too nonsensical
    to affect a creature. A Remove Curse or Greater Restoration spell cast on the target restores the
    creature’s true memory. Using a Higher-Level Spell Slot. You can alter the target’s memories of an
    event that took place up to 7 days ago (level 6 spell slot), 30 days ago (level 7 spell slot), 365
    days ago (level 8 spell slot), or any time in the creature’s past (level 9 spell slot).'
  sha256: 06631facd308bf6f579d4d361fd8f3b35f7b37f77dc30d3717580696babc32f2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Modify Memory

<!-- SRD page 150 -->

Level 5 Enchantment (Bard, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
You attempt to reshape another creature’s mem-\
ories. One creature that you can see within range\
makes a Wisdom saving throw. If you are fighting\
the creature, it has Advantage on the save. On a\
failed save, the target has the Charmed condition\
for the duration. While Charmed in this way, the\
target also has the Incapacitated condition and is\
unaware of its surroundings, though it can hear\
you. If it takes any damage or is targeted by another\
spell, this spell ends, and no memories are modified.\
While this charm lasts, you can affect the target’s\
memory of an event that it experienced within the\
last 24 hours and that lasted no more than 10 min-\
utes. You can permanently eliminate all memory of\
the event, allow the target to recall the event with\
perfect clarity, change its memory of the event’s de-\
tails, or create a memory of some other event.\
You must speak to the target to describe how its\
memories are affected, and it must be able to under-\
stand your language for the modified memories to\
take root. Its mind fills in any gaps in the details of\
your description. If the spell ends before you finish\
describing the modified memories, the creature’s\
memory isn’t altered. Otherwise, the modified\
memories take hold when the spell ends.\
A modified memory doesn’t necessarily affect\
how a creature behaves, particularly if the mem-\
ory contradicts the creature’s natural inclinations,\
alignment, or beliefs. An illogical modified memory,\
such as a false memory of how much the creature\
enjoyed swimming in acid, is dismissed as a bad\
dream. The GM might deem a modified memory too\
nonsensical to affect a creature.\
A Remove Curse or Greater Restoration spell cast\
on the target restores the creature’s true memory.\
Using a Higher-Level Spell Slot. You can alter the\
target’s memories of an event that took place up to\
7 days ago (level 6 spell slot), 30 days ago (level 7\
spell slot), 365 days ago (level 8 spell slot), or any\
time in the creature’s past (level 9 spell slot).\

Original layout: [[../../srd-5.2.1/pages/150|page 150]].
