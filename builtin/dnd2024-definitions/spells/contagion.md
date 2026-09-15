---
splat_schema: 1
splat_id: 19812bef-7e04-560e-ba1d-256201a0789d
splat_kind: spell
splat_key: dnd2024:spells/contagion
splat_name: Contagion
splat_origin:
  key: dnd2024:reference/spell/contagion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 119
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: necromancy
  classes:
  - cleric
  - druid
  components:
  - verbal
  - somatic
  concentration: false
  duration: 7 days
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/contagion
  implementation: dnd2024:spell_contagion
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
    ability_choice:
    - strength
    - dexterity
    - constitution
    - intelligence
    - wisdom
    - charisma
    save:
      ability: constitution
      dc:
        binding: save_dc
      failure:
        damage:
          dice:
            count: 11
            sides: 8
            add: 0
          type: necrotic
        condition: poisoned
        while_poisoned:
          saving_throw_disadvantage_ability: chosen
        repeat_save:
          event: target_turn_end
          successes_to_end: 3
          failures_to_stop_repeating_and_persist: 3
          count_initial_save: false
          duration_days: 7
        poison_removal_attempt:
          save:
            ability: constitution
            dc:
              binding: save_dc
            failure:
              prevent_condition_removal: true
            success:
              allow_condition_removal: true
      success:
        effect: none
splat_contracts:
- id: dnd2024:spell_contagion
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
  - damage
  events:
  - spell.cast_completed
  - turn.end
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - conditions
  - saving_throws
  - roll_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: e25d4cf1e220de3d44fdda7332768ac2de505a62cd229846aa8863b812d1a8d2
  source_path: builtin/srd-5.2.1/spell/contagion.md
  pages:
  - 119
  engine_gaps: &id001
  - dnd2024:spell_contagion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: feab2116f8b040f30cffa63eea736ebc3dd6d164e3411bdba7c7e2ba0718239b
splat_clauses:
- key: source_rules
  text: '# Contagion Level 5 Necromancy (Cleric, Druid) Casting Time: Action Range: Touch Component: V,
    S Duration: 7 days Your touch inflicts a magical contagion. The target must succeed on a Constitution
    saving throw or take 11d8 Necrotic damage and have the Poisoned condition. Also, choose one ability
    when you cast the spell. While Poisoned, the target has Disadvantage on saving throws made with the
    chosen ability. The target must repeat the saving throw at the end of each of its turns until it gets
    three successes or failures. If the target succeeds on three of these saves, the spell ends on the
    target. If the target fails three of the saves, the spell lasts for 7 days on it. Whenever the Poisoned
    target receives an effect that would end the Poisoned condition, the target must succeed on a Constitution
    saving throw, or the Poisoned condition doesn’t end on it.'
  sha256: bfda05a1e9fbe6702cd88430fb81c116cdbb155d48682c0e0c7f0cd0f488c9c6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Contagion

<!-- SRD page 119 -->

Level 5 Necromancy (Cleric, Druid)\
Casting Time: Action\
Range: Touch\
Component: V, S\
Duration: 7 days\
Your touch inflicts a magical contagion. The target\
must succeed on a Constitution saving throw or\
take 11d8 Necrotic damage and have the Poisoned\
condition. Also, choose one ability when you cast\
the spell. While Poisoned, the target has Disadvan-\
tage on saving throws made with the chosen ability.\
The target must repeat the saving throw at the\
end of each of its turns until it gets three successes\
or failures. If the target succeeds on three of these\
saves, the spell ends on the target. If the target fails\
three of the saves, the spell lasts for 7 days on it.\
Whenever the Poisoned target receives an effect\
that would end the Poisoned condition, the target\
must succeed on a Constitution saving throw, or the\
Poisoned condition doesn’t end on it.\

Original layout: [[../../srd-5.2.1/pages/119|page 119]].
