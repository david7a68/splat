---
splat_schema: 2
splat_id: 05de10ba-5e62-5333-b1d2-a121799a62b2
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_10_heightened_focus
splat_name: 'Level 10: Heightened Focus'
splat_origin: dnd2024:reference/pages/051
splat_sources:
- SRD 5.2.1, pp. 51, 52, https://www.dndbeyond.com/srd
---

# Level 10: Heightened Focus

## Source text

<!-- SRD page 51 -->

Your Flurry of Blows, Patient Defense, and Step of the Wind gain the following benefits.

Flurry of Blows. You can expend 1 Focus Point to use Flurry of Blows and make three Unarmed Strikes with it instead of two.

<!-- SRD page 52 -->

Patient Defense. When you expend a Focus Point to use Patient Defense, you gain a number of Temporary Hit Points equal to two rolls of your Martial Arts die.

Step of the Wind. When you expend a Focus Point to use Step of the Wind, you can choose a willing creature within 5 feet of yourself that is Large or smaller. You move the creature with you until the end of your turn. The creature’s movement doesn’t provoke Opportunity Attacks.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_10_heightened_focus ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_10_heightened_focus ` |

### Parameters

#### Flurry of blows

<!-- splat:values path=splat_rule/parameters/flurry_of_blows -->

| Field | Value |
| --- | --- |
| ` focus_cost ` | ` 1 ` |
| ` replace_unarmed_strike_count ` | ` 3 ` |

#### Patient defense

<!-- splat:yaml path=splat_rule/parameters/patient_defense -->

```yaml
when_focus_spent: true
temporary_hit_points:
  dice:
    count: 2
    sides:
      stat: martial_arts_die.sides
```

#### Step of the wind

<!-- splat:yaml path=splat_rule/parameters/step_of_the_wind -->

```yaml
when_focus_spent: true
companion_choice:
  count: 1
  willing: true
  maximum_distance_feet: 5
  maximum_size: large
moves_with_source_until: current_turn_end
companion_movement_provokes_opportunity_attacks: false
```
