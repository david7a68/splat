---
splat_schema: 1
splat_id: 64cf776a-1dcb-503a-b035-94e83b5f868a
splat_kind: rule
splat_key: dnd2024:glossary/influence_action
splat_name: Influence [Action]
splat_origin:
  key: dnd2024:reference/glossary/influence_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/influence_action
  parameters:
    cost: action
    target: monster
    interaction_modes:
    - deceive
    - intimidate
    - amuse
    - persuade
    - gently_coax
    receptiveness:
      chooser: gm
      options:
      - willing
      - unwilling
      - hesitant
      constraints:
        willing: urging_aligns_with_desires
        unwilling: urging_repugnant_or_counter_to_alignment
        hesitant: hesitant_to_comply
    branches:
      willing:
        check_required: false
        complies: true
        fulfillment_method_chooser: monster
      unwilling:
        check_required: false
        complies: false
      hesitant:
        check_required: true
        check_chooser: gm
        default_dc:
          op: max
          args:
          - 15
          - var: monster_intelligence_score
        attitude_modes:
          friendly: advantage
          hostile: disadvantage
          indifferent: normal
        success: complies
        failure:
          repeat_same_urging_cooldown:
            default_hours: 24
            override:
              chooser: gm
              type: duration
    suggested_checks:
    - mode: deceive
      ability: charisma
      skill: deception
      target_understands: true
    - mode: intimidate
      ability: charisma
      skill: intimidation
    - mode: amuse
      ability: charisma
      skill: performance
    - mode: persuade
      ability: charisma
      skill: persuasion
      target_understands: true
    - mode: gently_coax
      ability: wisdom
      skill: animal_handling
      target_types:
      - beast
      - monstrosity
  implementation: dnd2024:glossary_influence_action
splat_contracts:
- id: dnd2024:glossary_influence_action
  inputs:
    subject:
      type: participant
      required: true
    target:
      type: participant
      required: true
    interaction:
      type: choice
      required: true
    gm_receptiveness:
      type: choice
      required: true
    gm_check:
      type: choice
      required: true
    gm_dc:
      required: false
      type: integer
    gm_cooldown:
      required: false
      type: duration
  outputs:
  - compliance
  - check
  - repeat_cooldown
  events:
  - action.requested
  - ability_check.completed
  capabilities:
  - action_economy
  - relationships
  - checks
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: b72965562e6763366b449722ded72793f99c1589a5b48d91700b62c74d8fefbc
  source_path: builtin/srd-5.2.1/glossary/influence_action.md
  pages:
  - 184
  engine_gaps: &id001
  - dnd2024:glossary_influence_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 102caffb0cbd3c49d1e69868f41307a40da1852c257e93db6cf32f0188ef4357
splat_clauses:
- key: source_rules
  text: '# Influence [Action] With the Influence action, you urge a monster to do something. Describe
    or roleplay how you’re communicating with the monster. Are you trying to deceive, intimidate, amuse,
    or gently persuade? The GM then determines whether the monster feels willing, unwilling, or hesitant
    due to your interaction; this determination establishes whether an ability check is necessary, as
    explained below. Willing. If your urging aligns with the monster’s desires, no ability check is necessary;
    the monster fulfills your request in a way it prefers. Unwilling. If your urging is repugnant to the
    monster or counter to its alignment, no ability check is necessary; it doesn’t comply. Hesitant. If
    you urge the monster to do something that it is hesitant to do, you must make an ability check, which
    is affected by the monster’s attitude: Indifferent, Friendly, or Hostile, each of which is defined
    in this glossary. The Influence Checks table suggests which ability check to make based on how you’re
    interacting with the monster. The GM chooses the check, which has a default DC equal to 15 or the
    monster’s Intelligence score, whichever is higher. On a successful check, the monster does as urged.
    On a failed check, you must wait 24 hours (or a duration set by the GM) before urging it in the same
    way again. Influence Checks Ability Check Interaction Charisma (Deception) Deceiving a monster that
    understands you Charisma (Intimidation) Intimidating a monster Charisma (Performance) Amusing a monster
    Charisma (Persuasion) Persuading a monster that understands you Wisdom (Animal Handling) Gently coaxing
    a Beast or Monstrosity'
  sha256: 84fcd1ff6b2effb6ace2efe2544ee4d1218f7d4a4887999ad8dbeef728e3f0b4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Influence [Action]

<!-- SRD page 184 -->

With the Influence action, you urge a monster to\
do something. Describe or roleplay how you’re\
communicating with the monster. Are you trying to\
deceive, intimidate, amuse, or gently persuade? The\
GM then determines whether the monster feels will-\
ing, unwilling, or hesitant due to your interaction;\
this determination establishes whether an ability\
check is necessary, as explained below.\
Willing. If your urging aligns with the monster’s\
desires, no ability check is necessary; the monster\
fulfills your request in a way it prefers.\
Unwilling. If your urging is repugnant to the mon-\
ster or counter to its alignment, no ability check is\
necessary; it doesn’t comply.\
Hesitant. If you urge the monster to do something\
that it is hesitant to do, you must make an ability\
check, which is affected by the monster’s attitude:\
Indifferent, Friendly, or Hostile, each of which is\
defined in this glossary. The Influence Checks ta-\
ble suggests which ability check to make based on\
how you’re interacting with the monster. The GM\
chooses the check, which has a default DC equal to\
15 or the monster’s Intelligence score, whichever\
is higher. On a successful check, the monster does\
as urged. On a failed check, you must wait 24 hours\
(or a duration set by the GM) before urging it in the\
same way again.\
Influence Checks\
Ability Check Interaction\
Charisma (Deception) Deceiving a monster\
that understands you\
Charisma (Intimidation) Intimidating a monster\
Charisma (Performance) Amusing a monster\
Charisma (Persuasion) Persuading a monster\
that understands you\
Wisdom (Animal Handling) Gently coaxing a Beast\
or Monstrosity\

Original layout: [[../../srd-5.2.1/pages/184|page 184]].
