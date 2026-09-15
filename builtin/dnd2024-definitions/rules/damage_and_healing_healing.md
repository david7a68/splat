---
splat_schema: 1
splat_id: 8010b042-e75d-52f8-b45d-25eefc6b2c6f
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_healing
splat_name: Healing
splat_origin:
  key: dnd2024:reference/pages/017
splat_sources:
- title: SRD 5.2.1
  locator: pp. 17
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_healing
  parameters:
    healing:
      new_hp:
        op: min
        args:
        - var: subject.hp_max
        - op: add
          args:
          - var: subject.hp
          - var: healing
      discard_excess: true
    knock_out:
      optional: true
      trigger:
        melee_attack: true
        would_reduce_target_hp_to: 0
      replace_hp_with: 1
      condition: unconscious
      start_short_rest: true
      ends:
      - short_rest_finished
      - any_hp_regained
      - successful_first_aid
      first_aid:
        action_cost: 1
        ability: wisdom
        skill: medicine
        dc: 10
  implementation: dnd2024:rules_damage_and_healing_healing
splat_contracts:
- id: dnd2024:rules_damage_and_healing_healing
  inputs:
    subject:
      type: participant
      required: true
    healing:
      required: false
      type: integer
    attack:
      required: false
      type: resolution
  outputs:
  - hit_points
  - conditions
  - rest_state
  events:
  - healing.applied
  - attack.damage_pending
  capabilities:
  - healing
  - conditions
  - rests
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 17
  engine_gaps: &id001
  - dnd2024:rules_damage_and_healing_healing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 87728e8b992a9f4281133f2e9059ad5c4a2c3dc2f06f5ab834dc9d28f5b291f7
  parent_name: Damage and Healing
  line_span:
    first: 1204
    count: 20
splat_clauses:
- key: source_rules
  text: '# Healing Hit Points can be restored by magic, such as the Cure Wounds spell or a Potion of Healing,
    or by a Short or Long Rest (see “Rules Glossary”). knockinG out a crEaturE When you would reduce a
    creature to 0 Hit Points with a melee attack, you can instead reduce the creature to 1 Hit Point and
    give it the Unconscious condition. It then starts a Short Rest, at the end of which that condition
    ends on it. The condition ends early if the creature regains any Hit Points or if someone takes an
    action to administer first aid to it, making a successful DC 10 Wisdom (Medicine) check. When you
    receive healing, add the restored Hit Points to your current Hit Points. Your Hit Points can’t exceed
    your Hit Point maximum, so any Hit Points regained in excess of the maximum are lost. For example,
    if you receive 8 Hit Points of healing and have 14 Hit Points and a Hit Point maximum of 20, you regain
    6 Hit Points, not 8.'
  sha256: 3f9c121b70420f26f019cbca99fb7edbd1980f4bac669b52ab9ace98dec534de
  status: contract_specified
splat_requires_mechanics: *id001
---

# Healing

<!-- SRD page 17 -->

Hit Points can be restored by magic, such as the\
Cure Wounds spell or a Potion of Healing, or by a\
Short or Long Rest (see “Rules Glossary”).\
knockinG out a crEaturE\
When you would reduce a creature to 0 Hit Points\
with a melee attack, you can instead reduce the crea-\
ture to 1 Hit Point and give it the Unconscious condi-\
tion. It then starts a Short Rest, at the end of which\
that condition ends on it. The condition ends early\
if the creature regains any Hit Points or if someone\
takes an action to administer first aid to it, making a\
successful DC 10 Wisdom (Medicine) check.\
When you receive healing, add the restored Hit\
Points to your current Hit Points. Your Hit Points\
can’t exceed your Hit Point maximum, so any Hit\
Points regained in excess of the maximum are lost.\
For example, if you receive 8 Hit Points of healing\
and have 14 Hit Points and a Hit Point maximum of\
20, you regain 6 Hit Points, not 8.\
