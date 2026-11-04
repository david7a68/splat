---
splat_schema: 2
splat_id: e9a0df6f-d859-5bab-be28-157e061d05b1
splat_kind: item
splat_key: dnd2024:equipment/rope_1_gp
splat_name: Rope (1 GP)
splat_origin: dnd2024:reference/pages/099
splat_sources:
- SRD 5.2.1, pp. 99, https://www.dndbeyond.com/srd
---

# Rope (1 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 99 -->

As a Utilize action, you can tie a knot with Rope if you succeed on a DC 10 Dexterity (Sleight of Hand) check. The Rope can be burst with a successful DC 20 Strength (Athletics) check.

You can bind an unwilling creature with the Rope only if the creature has the Grappled, Incapacitated, or Restrained condition. If the creature’s legs are bound, the creature has the Restrained condition until it escapes. Escaping the Rope requires the creature to make a successful DC 15 Dexterity (Acrobatics) check as an action.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/rope_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_rope_1_gp ` |

### Parameters

#### Tie knot

<!-- splat:yaml path=splat_rule/parameters/tie_knot -->

```yaml
action: utilize
check:
  ability: dexterity
  skill: sleight_of_hand
  dc: 10
```

#### Bind

<!-- splat:values path=splat_rule/parameters/bind -->

| Field | Value |
| --- | --- |
| ` unwilling_target_requires_any_condition ` | ` ["grappled", "incapacitated", "restrained"] ` |
| ` bound_legs_condition ` | ` restrained ` |
| ` until ` | ` escape ` |

#### Burst

<!-- splat:yaml path=splat_rule/parameters/burst -->

```yaml
check:
  ability: strength
  skill: athletics
  dc: 20
```

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
economy: action
check:
  ability: dexterity
  skill: acrobatics
  dc: 15
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 1
> mass:
>   unit: lb
>   value: 5.0
> ```

