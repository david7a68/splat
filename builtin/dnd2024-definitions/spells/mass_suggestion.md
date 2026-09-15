---
splat_schema: 1
splat_id: 584df967-6461-5764-a029-9a90b1989351
splat_kind: spell
splat_key: dnd2024:spells/mass_suggestion
splat_name: Mass Suggestion
splat_origin:
  key: dnd2024:reference/spell/mass_suggestion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 148
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: enchantment
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - material
  concentration: false
  duration: 24 hours
  casting_time: Action
  ritual: false
  range:
    feet: 60
  material:
    description: a snake’s tongue
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/mass_suggestion
  implementation: dnd2024:spell_mass_suggestion
  parameters:
    targets:
      kind: creature
      maximum: 12
      within_spell_range: true
      visible: true
      can_hear_caster: true
      understands_caster: true
    suggestion:
      word_limit: 25
      must_sound_achievable: true
      must_not_obviously_damage:
      - targets
      - target_allies
      content_validation:
        chooser: gm
        input_type: choice
        constraints:
        - achievable_appearance
        - no_obvious_damage
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        condition: charmed
        duration: spell_duration
        behavior: pursue_suggestion_best_ability
        end_on_target:
        - caster_or_caster_ally_deals_damage_to_target
        - suggested_activity_complete
      success:
        effect: none
    duration_by_slot:
      '6':
        hours: 24
      '7':
        days: 10
      '8':
        days: 30
      '9':
        days: 366
splat_contracts:
- id: dnd2024:spell_mass_suggestion
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
    message:
      required: false
      type: text
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
  - behavior_constraints
splat_conversion:
  status: specified
  category: spell
  source_sha256: eb05eac186ae004288178f1ffb7dafcc636bc113bf05096dabcafaf70a229e2e
  source_path: builtin/srd-5.2.1/spell/mass_suggestion.md
  pages:
  - 148
  engine_gaps: &id001
  - dnd2024:spell_mass_suggestion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 8ed16632ebfbec59a3c9d7c8ac319595a97129eeb396055bd59f359a8191643e
splat_clauses:
- key: source_rules
  text: '# Mass Suggestion Level 6 Enchantment (Bard, Sorcerer, Wizard) Casting Time: Action Range: 60
    feet Components: V, M (a snake’s tongue) Duration: 24 hours You suggest a course of activity—described
    in no more than 25 words—to twelve or fewer creatures you can see within range that can hear and understand
    you. The suggestion must sound achievable and not involve anything that would obviously deal damage
    to any of the targets or their allies. For example, you could say, “Walk to the village down that
    road, and help the villagers there harvest crops until sunset.” Or you could say, “Now is not the
    time for violence. Drop your weapons, and dance! Stop in an hour.” Each target must succeed on a Wisdom
    saving throw or have the Charmed condition for the duration or until you or your allies deal damage
    to the target. Each Charmed target pursues the suggestion to the best of its ability. The suggested
    activity can continue for the entire duration, but if the suggested activity can be completed in a
    shorter time, the spell ends for a target upon completing it. Using a Higher-Level Spell Slot. The
    duration is longer with a spell slot of level 7 (10 days), 8 (30 days), or 9 (366 days).'
  sha256: 6a12759f38151aea6a182b0097ae19af43d763f9eea2fda14cff2f9d27c341d0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mass Suggestion

<!-- SRD page 148 -->

Level 6 Enchantment (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, M (a snake’s tongue)\
Duration: 24 hours\
You suggest a course of activity—described in no\
more than 25 words—to twelve or fewer creatures\
you can see within range that can hear and under-\
stand you. The suggestion must sound achievable\
and not involve anything that would obviously deal\
damage to any of the targets or their allies. For\
example, you could say, “Walk to the village down\
that road, and help the villagers there harvest crops\
until sunset.” Or you could say, “Now is not the time\
for violence. Drop your weapons, and dance! Stop in\
an hour.”\
Each target must succeed on a Wisdom saving\
throw or have the Charmed condition for the dura-\
tion or until you or your allies deal damage to the\
target. Each Charmed target pursues the suggestion\
to the best of its ability. The suggested activity can\
continue for the entire duration, but if the sug-\
gested activity can be completed in a shorter time,\
the spell ends for a target upon completing it.\
Using a Higher-Level Spell Slot. The duration is\
longer with a spell slot of level 7 (10 days), 8 (30\
days), or 9 (366 days).\

Original layout: [[../../srd-5.2.1/pages/148|page 148]].
