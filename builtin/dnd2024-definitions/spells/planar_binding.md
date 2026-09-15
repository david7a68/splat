---
splat_schema: 1
splat_id: ed4eb4d9-23b3-5629-942a-66ce2d688863
splat_kind: spell
splat_key: dnd2024:spells/planar_binding
splat_name: Planar Binding
splat_origin:
  key: dnd2024:reference/spell/planar_binding
splat_sources:
- title: SRD 5.2.1
  locator: pp. 152, 153
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: abjuration
  classes:
  - bard
  - cleric
  - druid
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 24 hours
  casting_time: 1 hour
  ritual: false
  range:
    feet: 60
  material:
    description: a jewel worth 1,000+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/planar_binding
  implementation: dnd2024:spell_planar_binding
  parameters:
    targeting:
      kind: creature
      count: 1
      types:
      - celestial
      - elemental
      - fey
      - fiend
      within_range_for_entire_cast: true
    save:
      timing: casting_completion
      ability: charisma
      failure:
        bound_service: spell_duration
        summoned_or_created_by_other_spell: extend_that_spell_to_this_duration
    commands:
      obedience: best_of_ability
      hostile_target: twists_commands_toward_own_objectives
    completed_commands_early:
      same_plane: travel_to_caster_and_report
      different_plane: return_to_binding_location_and_wait_until_spell_end
    slot_duration_days:
      '5': 1
      '6': 10
      '7': 30
      '8': 180
      '9': 366
    spell_lifetime:
      concentration: false
      maximum:
        value: 24
        unit: hour
splat_contracts:
- id: dnd2024:spell_planar_binding
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
    campaign:
      type: campaign
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  - time.elapsed
  - position.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - spatial_effects
  - campaign_state
  - planar_travel
  - controlled_participants
splat_conversion:
  status: specified
  category: spell
  source_sha256: 642a5dfb2350dcb5937952b578f001ccf3b18508bda84dbc6c70c4a1512e7702
  source_path: builtin/srd-5.2.1/spell/planar_binding.md
  pages:
  - 152
  - 153
  engine_gaps: &id001
  - dnd2024:spell_planar_binding
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 0d32ea64ba66df5373ebea049f612ffa01dbfe13b1eb24b2ea1e5e39c609f690
splat_clauses:
- key: source_rules
  text: '# Planar Binding Level 5 Abjuration (Bard, Cleric, Druid, Warlock, Wizard) Casting Time: 1 hour
    Range: 60 feet Components: V, S, M (a jewel worth 1,000+ GP, which the spell consumes) Duration: 24
    hours You attempt to bind a Celestial, an Elemental, a Fey, or a Fiend to your service. The creature
    must be within range for the entire casting of the spell. (Typically, the creature is first summoned
    into the center of the inverted version of the Magic Circle spell to trap it while this spell is cast.)
    At the completion of the casting, the target must succeed on a Charisma saving throw or be bound to
    serve you for the duration. If the creature was summoned or created by another spell, that spell’s
    duration is extended to match the duration of this spell. A bound creature must follow your commands
    to the best of its ability. You might command the creature to accompany you on an adventure, to guard
    a location, or to deliver a message. If the creature is Hostile, it strives to twist your commands
    to achieve its own objectives. If the creature carries out your commands completely before the spell
    ends, it travels to you to report this fact if you are on the same plane of existence. If you are
    on a different plane, it returns to the place where you bound it and remains there until the spell
    ends. Using a Higher-Level Spell Slot. The duration increases with a spell slot of level 6 (10 days),
    7 (30 days), 8 (180 days), and 9 (366 days).'
  sha256: adac762987de7c7eebd374962e0cf8c6a57a1e189510cf9413ff0e321d28d432
  status: contract_specified
splat_requires_mechanics: *id001
---

# Planar Binding

<!-- SRD page 152 -->

Level 5 Abjuration (Bard, Cleric, Druid, Warlock,\
Wizard)\
Casting Time: 1 hour\
Range: 60 feet\
Components: V, S, M (a jewel worth 1,000+ GP, which\
the spell consumes)\
Duration: 24 hours\

<!-- SRD page 153 -->

You attempt to bind a Celestial, an Elemental, a\
Fey, or a Fiend to your service. The creature must\
be within range for the entire casting of the spell.\
(Typically, the creature is first summoned into the\
center of the inverted version of the Magic Circle\
spell to trap it while this spell is cast.) At the com-\
pletion of the casting, the target must succeed on\
a Charisma saving throw or be bound to serve you\
for the duration. If the creature was summoned or\
created by another spell, that spell’s duration is ex-\
tended to match the duration of this spell.\
A bound creature must follow your commands\
to the best of its ability. You might command the\
creature to accompany you on an adventure, to\
guard a location, or to deliver a message. If the\
creature is Hostile, it strives to twist your com-\
mands to achieve its own objectives. If the creature\
carries out your commands completely before the\
spell ends, it travels to you to report this fact if you\
are on the same plane of existence. If you are on a\
different plane, it returns to the place where you\
bound it and remains there until the spell ends.\
Using a Higher-Level Spell Slot. The duration\
increases with a spell slot of level 6 (10 days), 7 (30\
days), 8 (180 days), and 9 (366 days).\

Original layout: [[../../srd-5.2.1/pages/152|page 152]], [[../../srd-5.2.1/pages/153|page 153]].
