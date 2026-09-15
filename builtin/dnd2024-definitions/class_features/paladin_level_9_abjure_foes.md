---
splat_schema: 1
splat_id: be190777-faf7-5c4d-803e-bc4ac916d144
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_9_abjure_foes
splat_name: 'Level 9: Abjure Foes'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_9_abjure_foes
  parameters:
    action: magic
    resource_cost:
      id: paladin.channel_divinity
      amount: 1
    present_item_any:
    - holy_symbol
    - weapon
    targets:
      maximum:
        op: max
        args:
        - 1
        - var: actor.abilities.charisma.modifier
      visible: true
      range_feet: 60
      type: creature
    save:
      ability: wisdom
      dc:
        op: add
        args:
        - 8
        - var: actor.abilities.charisma.modifier
        - var: actor.proficiency_bonus
      failure:
        condition: frightened
        source: actor
        duration:
          minutes: 1
        ends_on:
        - target.damage_taken
        turn_activity_limit:
          choose_only_one:
          - move
          - action
          - bonus_action
    class_scope:
      class: paladin
      minimum_level: 9
  implementation: dnd2024:class_features_paladin_level_9_abjure_foes
splat_contracts:
- id: dnd2024:class_features_paladin_level_9_abjure_foes
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - action.magic
  - damage.taken
  - turn.start
  capabilities:
  - character_features
  - resources
  - damage
  - conditions
  - proficiencies
  - d20_tests
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 55
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_9_abjure_foes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 8ac9d276387bca2c1c67027f77a54b063507b5ec01a655258636d854d74e548b
  parent_name: Paladin
  line_span:
    first: 4625
    count: 12
splat_clauses:
- key: source_rules
  text: '# Level 9: Abjure Foes As a Magic action, you can expend one use of this class’s Channel Divinity
    to overwhelm foes with awe. As you present your Holy Symbol or weapon, you can target a number of
    creatures equal to your Charisma modifier (minimum of one creature) that you can see within 60 feet
    of yourself. Each target must succeed on a Wisdom saving throw or have the Frightened condition for
    1 minute or until it takes any damage. While Frightened in this way, a target can do only one of the
    following on its turns: move, take an action, or take a Bonus Action.'
  sha256: 84f6af8d4753ef8d9a13ddf310becebda0eb71b18952524a80dd533428331866
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 9: Abjure Foes

<!-- SRD page 55 -->

As a Magic action, you can expend one use of this\
class’s Channel Divinity to overwhelm foes with\
awe. As you present your Holy Symbol or weapon,\
you can target a number of creatures equal to your\
Charisma modifier (minimum of one creature) that\
you can see within 60 feet of yourself. Each target\
must succeed on a Wisdom saving throw or have the\
Frightened condition for 1 minute or until it takes\
any damage. While Frightened in this way, a target\
can do only one of the following on its turns: move,\
take an action, or take a Bonus Action.\
