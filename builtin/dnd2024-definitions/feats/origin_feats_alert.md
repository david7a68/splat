---
splat_schema: 1
splat_id: 42bc1a6d-829d-5d92-8f01-3d2fd68d5155
splat_kind: rule
splat_key: dnd2024:feats/origin_feats_alert
splat_name: Alert
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
---

# Alert

## Source text

<!-- SRD page 87 -->

Origin Feat

You gain the following benefits.

Initiative Proficiency. When you roll Initiative, you can add your Proficiency Bonus to the roll.

Initiative Swap. Immediately after you roll Initiative, you can swap your Initiative with the Initiative of one willing ally in the same combat. You can’t make this swap if you or the ally has the Incapacitated condition.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/origin_feats_alert ` |
| ` implementation ` | ` dnd2024:feats_origin_feats_alert ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` origin ` |

#### Initiative bonus

<!-- splat:yaml path=splat_rule/parameters/initiative_bonus -->

```yaml
optional: true
amount:
  var: actor.proficiency_bonus
```

#### Initiative swap

<!-- splat:yaml path=splat_rule/parameters/initiative_swap -->

```yaml
timing: immediately_after_actor_initiative_roll
optional: true
target:
  willing: true
  ally: true
  same_combat: true
  incapacitated: false
requires:
  actor_incapacitated: false
swap_values:
- actor.initiative
- target.initiative
```
