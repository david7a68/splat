---
splat_schema: 1
splat_id: 6f2e880d-4f14-5820-a14c-228a2bc55e20
splat_kind: rule
splat_key: dnd2024:rules/fear_and_mental_stress_mental_stress_effects
splat_name: Mental Stress Effects
splat_origin:
  key: dnd2024:reference/pages/196
splat_sources:
- title: SRD 5.2.1
  locator: pp. 196
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/fear_and_mental_stress_mental_stress_effects
  parameters:
    damage_type: psychic
    save_ability_choice:
      default: wisdom
      alternatives:
      - intelligence
      - charisma
      authority: gm
    success_damage_choice:
      options:
      - none
      - half
      authority: gm
    example_effects:
    - id: hallucinogenic_substance
      save_dc: 10
      damage_dice:
        count: 1
        sides: 6
        add: 0
    - id: fiendish_idol
      save_dc: 15
      damage_dice:
        count: 3
        sides: 6
        add: 0
    - id: far_realm_trap
      save_dc: 20
      damage_dice:
        count: 9
        sides: 6
        add: 0
      planar_displacement:
        plane: far_realm
        until: target_next_turn_end
    examples_are_suggestions: true
  implementation: dnd2024:rules_fear_and_mental_stress_mental_stress_effects
splat_contracts:
- id: dnd2024:rules_fear_and_mental_stress_mental_stress_effects
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    hazard:
      type: object
      required: true
    event:
      type: event
      required: true
    resolution:
      type: resolution
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - effects
  - hazard_updates
  events: []
  capabilities:
  - hazards
  - effects
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 196
  engine_gaps: &id001
  - dnd2024:rules_fear_and_mental_stress_mental_stress_effects
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: f7b17c6b13c3ab2cdf93a03e09880fa9495d06d01fff250fbe83ba9eff4b044c
  parent_name: Fear and Mental Stress
  line_span:
    first: 9724
    count: 24
splat_clauses:
- key: source_rules
  text: '# Mental Stress Effects When a character is subjected to an effect that causes intense mental
    stress, Psychic damage is the best way to emulate that effect. The Sample Mental Stress Effects table
    provides a few examples of such effects, with suggested saving throw DCs and damage. Mental stress
    can usually be resisted with a successful Wisdom save, but sometimes an Intelligence or Charisma save
    is more appropriate. On a successful save, a character might take half as much damage instead of no
    damage, at your discretion. Sample Mental Stress Effects Save Psychic Example DC Damage A character
    ingests a hallucinogenic 10 1d6 substance that distorts the character’s perception of reality. A character
    touches a fiendish idol 15 3d6 that tears at the character’s mind, threatening to shatter it. A magical
    trap flings a character into 20 9d6 the Far Realm until the end of that character’s next turn.'
  sha256: 97c453a775c75901076c2b4325f2f2d963303997484393a1acaf7524c4a08f19
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mental Stress Effects

<!-- SRD page 196 -->

When a character is subjected to an effect that\
causes intense mental stress, Psychic damage is the\
best way to emulate that effect.\
The Sample Mental Stress Effects table provides a\
few examples of such effects, with suggested saving\
throw DCs and damage. Mental stress can usually\
be resisted with a successful Wisdom save, but\
sometimes an Intelligence or Charisma save is more\
appropriate. On a successful save, a character might\
take half as much damage instead of no damage, at\
your discretion.\
Sample Mental Stress Effects\
Save Psychic\
Example DC Damage\
A character ingests a hallucinogenic 10 1d6\
substance that distorts the charac-\
ter’s perception of reality.\
A character touches a fiendish idol 15 3d6\
that tears at the character’s mind,\
threatening to shatter it.\
A magical trap flings a character into 20 9d6\
the Far Realm until the end of that\
character’s next turn.\
