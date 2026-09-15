---
splat_schema: 1
splat_id: 8435cd6f-3662-5bf2-92b7-032054b2fa8d
splat_kind: spell
splat_key: dnd2024:spells/suggestion
splat_name: Suggestion
splat_origin:
  key: dnd2024:reference/spell/suggestion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 166
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: enchantment
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - material
  concentration: true
  duration: Concentration, up to 8 hours
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a drop of honey
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/suggestion
  implementation: dnd2024:spell_suggestion
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 28800
    target:
      kind: creature
      visible: true
      range_feet: 30
      hears_caster: true
      understands_caster: true
    instruction:
      maximum_words: 25
      kind: course_of_activity
      must_sound_achievable: true
      obvious_damage_to_target_or_allies: forbidden
      feasibility_resolution: supplied_assessment
    save:
      ability: wisdom
      dc: spell_save_dc
      success: unaffected
    failure:
      condition: charmed
      source: caster
      instruction_compliance: best_effort
    end_target:
      any:
      - caster_or_allies_deal_damage_to_target
      - suggested_activity_completed
      - spell_duration_ended
splat_contracts:
- id: dnd2024:spell_suggestion
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
    suggested_activity:
      type: text
      required: true
    feasibility_assessment:
      type: resolution
      required: true
    save_dc:
      type: integer
      required: true
  outputs:
  - effects
  events:
  - concentration.ended
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
  source_sha256: 4b8814b86863188ffa7099d187393eaf2686d560ffc0e2bc3a59c9864154860b
  source_path: builtin/srd-5.2.1/spell/suggestion.md
  pages:
  - 166
  engine_gaps: &id001
  - dnd2024:spell_suggestion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 8ff318cad3c2bfe9bb55d1271b95a32dca47c3681f9f85baac880d2c3116b014
splat_clauses:
- key: source_rules
  text: '# Suggestion Level 2 Enchantment (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    30 feet Components: V, M (a drop of honey) Duration: Concentration, up to 8 hours You suggest a course
    of activity—described in no more than 25 words—to one creature you can see within range that can hear
    and understand you. The suggestion must sound achievable and not involve anything that would obviously
    deal damage to the target or its allies. For example, you could say, “Fetch the key to the cult’s
    treasure vault, and give the key to me.” Or you could say, “Stop fighting, leave this library peacefully,
    and don’t return.” The target must succeed on a Wisdom saving throw or have the Charmed condition
    for the duration or until you or your allies deal damage to the target. The Charmed target pursues
    the suggestion to the best of its ability. The suggested activity can continue for the entire duration,
    but if the suggested activity can be completed in a shorter time, the spell ends for the target upon
    completing it.'
  sha256: 81b608cdc586ae3db5551bc0c6e10fda1902dffeea7ec58bba22e9d2d37da418
  status: contract_specified
splat_requires_mechanics: *id001
---

# Suggestion

<!-- SRD page 166 -->

Level 2 Enchantment (Bard, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, M (a drop of honey)\
Duration: Concentration, up to 8 hours\
You suggest a course of activity—described in no\
more than 25 words—to one creature you can see\
within range that can hear and understand you.\
The suggestion must sound achievable and not in-\
volve anything that would obviously deal damage\
to the target or its allies. For example, you could\
say, “Fetch the key to the cult’s treasure vault, and\
give the key to me.” Or you could say, “Stop fighting,\
leave this library peacefully, and don’t return.”\
The target must succeed on a Wisdom saving\
throw or have the Charmed condition for the dura-\
tion or until you or your allies deal damage to the\
target. The Charmed target pursues the suggestion\
to the best of its ability. The suggested activity\
can continue for the entire duration, but if the sug-\
gested activity can be completed in a shorter time,\
the spell ends for the target upon completing it.\

Original layout: [[../../srd-5.2.1/pages/166|page 166]].
