---
splat_schema: 2
splat_id: 64cf776a-1dcb-503a-b035-94e83b5f868a
splat_kind: rule
splat_key: dnd2024:glossary/influence_action
splat_name: Influence [Action]
splat_origin: dnd2024:reference/glossary/influence_action
splat_sources:
- SRD 5.2.1, pp. 184, https://www.dndbeyond.com/srd
---

# Influence [Action]

## Source text

<!-- SRD page 184 -->

With the Influence action, you urge a monster to do something. Describe or roleplay how you’re communicating with the monster. Are you trying to deceive, intimidate, amuse, or gently persuade? The GM then determines whether the monster feels willing, unwilling, or hesitant due to your interaction; this determination establishes whether an ability check is necessary, as explained below.

Willing. If your urging aligns with the monster’s desires, no ability check is necessary; the monster fulfills your request in a way it prefers.

Unwilling. If your urging is repugnant to the monster or counter to its alignment, no ability check is necessary; it doesn’t comply.

Hesitant. If you urge the monster to do something that it is hesitant to do, you must make an ability check, which is affected by the monster’s attitude: Indifferent, Friendly, or Hostile, each of which is defined in this glossary. The Influence Checks table suggests which ability check to make based on how you’re interacting with the monster. The GM chooses the check, which has a default DC equal to 15 or the monster’s Intelligence score, whichever is higher. On a successful check, the monster does as urged. On a failed check, you must wait 24 hours (or a duration set by the GM) before urging it in the same way again.

**Influence Checks**

| Ability Check | Interaction |
| --- | --- |
| Charisma (Deception) | Deceiving a monster that understands you |
| Charisma (Intimidation) | Intimidating a monster |
| Charisma (Performance) | Amusing a monster |
| Charisma (Persuasion) | Persuading a monster that understands you |
| Wisdom (Animal Handling) | Gently coaxing a Beast or Monstrosity |

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

<!-- splat:table
path: splat_rule/parameters/suggested_checks
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
columns:
  Key:
    type: key
  Mode:
    path: mode
    type: symbol
  Ability:
    path: ability
    type: symbol
  Skill:
    path: skill
    type: symbol
  Target understands:
    path: target_understands
    type: boolean
    optional: true
  Target types:
    path: target_types
    type: yaml
    optional: true
-->

| Key | Mode | Ability | Skill | Target understands | Target types |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Deceive ` | ` Charisma ` | ` Deception ` | true |  |
| ` row_2 ` | ` Intimidate ` | ` Charisma ` | ` Intimidation ` |  |  |
| ` row_3 ` | ` Amuse ` | ` Charisma ` | ` Performance ` |  |  |
| ` row_4 ` | ` Persuade ` | ` Charisma ` | ` Persuasion ` | true |  |
| ` row_5 ` | ` Gently coax ` | ` Wisdom ` | ` Animal handling ` |  | ` ["beast", "monstrosity"] ` |
