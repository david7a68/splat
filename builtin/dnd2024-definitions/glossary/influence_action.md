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
---

# Influence [Action]

## Source text

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

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/influence_action ` |
| ` implementation ` | ` dnd2024:glossary_influence_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` target ` | ` monster ` |
| ` interaction_modes ` | ` ["deceive", "intimidate", "amuse", "persuade", "gently_coax"] ` |

#### Receptiveness

<!-- splat:yaml path=splat_rule/parameters/receptiveness -->

```yaml
chooser: gm
options:
- willing
- unwilling
- hesitant
constraints:
  willing: urging_aligns_with_desires
  unwilling: urging_repugnant_or_counter_to_alignment
  hesitant: hesitant_to_comply
```

#### Branches

<!-- splat:yaml path=splat_rule/parameters/branches -->

```yaml
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
```

#### Suggested checks

<!-- splat:rows path=splat_rule/parameters/suggested_checks -->

| Row | mode | ability | skill | target_understands | target_types |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` deceive ` | ` charisma ` | ` deception ` | ` true ` |  |
| ` 1 ` | ` intimidate ` | ` charisma ` | ` intimidation ` |  |  |
| ` 2 ` | ` amuse ` | ` charisma ` | ` performance ` |  |  |
| ` 3 ` | ` persuade ` | ` charisma ` | ` persuasion ` | ` true ` |  |
| ` 4 ` | ` gently_coax ` | ` wisdom ` | ` animal_handling ` |  | ` ["beast", "monstrosity"] ` |
