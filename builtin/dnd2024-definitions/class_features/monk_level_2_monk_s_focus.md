---
splat_schema: 1
splat_id: 36f8521e-68ca-57d2-900a-89da5c6a91b4
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_2_monk_s_focus
splat_name: 'Level 2: Monk’s Focus'
splat_origin:
  key: dnd2024:reference/pages/050
splat_sources:
- title: SRD 5.2.1
  locator: pp. 50, 51
  url: https://www.dndbeyond.com/srd
---

# Level 2: Monk’s Focus

## Source text

<!-- SRD page 50 -->

Your focus and martial training allow you to har-\
ness a well of extraordinary energy within yourself.\
This energy is represented by Focus Points. Your\
Monk level determines the number of points you\
have, as shown in the Focus Points column of the\
Monk Features table.\
You can expend these points to enhance or fuel\
certain Monk features. You start knowing three\
such features: Flurry of Blows, Patient Defense, and\
Step of the Wind, each of which is detailed below.\
When you expend a Focus Point, it is unavailable\
until you finish a Short or Long Rest, at the end of\
which you regain all your expended points.\
Some features that use Focus Points require your\
target to make a saving throw. The save DC equals 8\
plus your Wisdom modifier and Proficiency Bonus.\
Flurry of Blows. You can expend 1 Focus Point to\
make two Unarmed Strikes as a Bonus Action.\
Patient Defense. You can take the Disengage\
action as a Bonus Action. Alternatively, you can\

<!-- SRD page 51 -->

expend 1 Focus Point to take both the Disengage\
and the Dodge actions as a Bonus Action.\
Step of the Wind. You can take the Dash action\
as a Bonus Action. Alternatively, you can expend 1\
Focus Point to take both the Disengage and Dash\
actions as a Bonus Action, and your jump distance is\
doubled for the turn.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_2_monk_s_focus ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_2_monk_s_focus ` |

### Parameters

#### Resource

<!-- splat:yaml path=splat_rule/parameters/resource -->

```yaml
id: focus_points
maximum:
  stat: classes.monk.level
recover:
  events:
  - rest.short_rest_completed
  - rest.long_rest_completed
  amount: all_expended
```

#### Feature save dc

<!-- splat:yaml path=splat_rule/parameters/feature_save_dc -->

```yaml
op: add
args:
- 8
- stat: abilities.wisdom.modifier
- stat: proficiency_bonus
```

#### Options

<!-- splat:table
path: splat_rule/parameters/options
key: Variant
order:
- flurry_of_blows
- patient_defense
- step_of_the_wind
columns:
  Variant:
    path: id
    type: symbol
  Action:
    path: action
    type: symbol
  Focus cost:
    path: focus_cost
    type: integer
  Unarmed strikes:
    path: unarmed_strikes
    type: integer
    optional: true
  Actions:
    path: actions
    type: yaml
    optional: true
  Paid alternative focus cost:
    path: paid_alternative/focus_cost
    type: integer
    optional: true
  Paid alternative actions:
    path: paid_alternative/actions
    type: yaml
    optional: true
  Paid alternative jump distance multiplier:
    path: paid_alternative/jump_distance_multiplier
    type: integer
    optional: true
  Paid alternative duration:
    path: paid_alternative/duration
    type: symbol
    optional: true
-->

| Variant | Action | Focus cost | Unarmed strikes | Actions | Paid alternative focus cost | Paid alternative actions | Paid alternative jump distance multiplier | Paid alternative duration |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` Flurry of blows ` | ` Bonus action ` | ` 1 ` | ` 2 ` |  |  |  |  |  |
| ` Patient defense ` | ` Bonus action ` | ` 0 ` |  | ` ["disengage"] ` | ` 1 ` | ` ["disengage", "dodge"] ` |  |  |
| ` Step of the wind ` | ` Bonus action ` | ` 0 ` |  | ` ["dash"] ` | ` 1 ` | ` ["disengage", "dash"] ` | ` 2 ` | ` Current turn ` |
